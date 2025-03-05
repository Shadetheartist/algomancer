mod procedure;
mod sba;
mod stack;

use rand_core::SeedableRng;
use std::collections::BTreeMap;
use thiserror::Error;

use crate::action::ActionError;
use crate::action::{Action, PlayerAction};
use crate::card::{Card, CardId};
use crate::database::{Database, DatabaseError};
use crate::event::Event;
use crate::library::{Library, LibraryId};
use crate::object::{Object, ObjectId};
use crate::options::Options;
use crate::permanent::Permanent;
use crate::player::{Player, PlayerId};
use crate::priority::Priority;
use crate::rng::GreRng;
use crate::state::procedure::Phase;
use crate::state::stack::Stack;
use crate::zone::Zone;

#[derive(Clone, Hash, PartialEq, Eq, Default)]
pub struct State {
    options: Options,
    rng: GreRng,
    stack: Stack,
    phase: Phase,
    priority: Priority,
    event_queue: Vec<Event>,
    objects: BTreeMap<ObjectId, Object>,
    players: BTreeMap<PlayerId, Player>,
    libraries: BTreeMap<LibraryId, Library>,
}

#[derive(Error, Debug)]
pub enum StateError {
    #[error("player {0} does not exist")]
    PlayerDoesNotExist(PlayerId),
    #[error("library {0} does not exist")]
    LibraryDoesNotExist(LibraryId),
    #[error("object {0} does not exist")]
    ObjectDoesNotExist(ObjectId),
    #[error("player {0} does not have card {1} in hand")]
    CardNotInHand(PlayerId, CardId),
    #[error("database error: {0}")]
    DatabaseError(#[from] DatabaseError),
}

impl TryFrom<&Options> for State {
    type Error = StateError;

    fn try_from(options: &Options) -> Result<Self, Self::Error> {
        let rng = GreRng::from_seed(options.seed.to_le_bytes());

        let state = Self {
            options: options.clone(),
            rng: rng,
            ..Default::default()
        };

        Ok(state)
    }
}

impl State {
    pub fn apply_action(&self, action: &Action, db: &Database) -> Result<Self, ActionError> {
        let mut state = self.clone();

        match action {
            Action::PlayerAction { player_id, action } => match action {
                PlayerAction::Cast { card_id } => state.player_cast(db, player_id, card_id)?,
            },
        }


        Ok(state)
    }

    pub fn valid_actions() -> Vec<Action> {
        vec![]
    }

    pub fn is_terminal() -> bool {
        false
    }
}

impl State {
    pub fn player(&self, player_id: &PlayerId) -> Result<&Player, StateError> {
        self.players
            .get(player_id)
            .ok_or(StateError::PlayerDoesNotExist(*player_id))
    }

    pub fn library(&self, library_id: &LibraryId) -> Result<&Library, StateError> {
        self.libraries
            .get(library_id)
            .ok_or(StateError::LibraryDoesNotExist(*library_id))
    }

    pub fn library_mut(&mut self, library_id: &LibraryId) -> Result<&mut Library, StateError> {
        self.libraries
            .get_mut(library_id)
            .ok_or(StateError::LibraryDoesNotExist(*library_id))
    }

    pub fn object(&self, object_id: &ObjectId) -> Result<&Object, StateError> {
        self.objects
            .get(object_id)
            .ok_or(StateError::ObjectDoesNotExist(*object_id))
    }

    pub fn object_mut(&mut self, object_id: &ObjectId) -> Result<&mut Object, StateError> {
        self.objects
            .get_mut(object_id)
            .ok_or(StateError::ObjectDoesNotExist(*object_id))
    }

    pub fn card_mut(&mut self, card_id: &CardId) -> Result<&mut Card, StateError> {
        let object_id = ObjectId::from(*card_id);
        let object = self.object_mut(&object_id)?;

        match object {
            Object::Card { card } => Ok(card),
            _ => panic!("card query found an object but it's not a card"),
        }
    }

    pub fn player_draw(&mut self, player_id: &PlayerId) -> Result<(), StateError> {
        let library_id = self.player(&player_id)?.library_id;
        let mut library = self.library_mut(&library_id)?;
        let card_id = library
            .card_ids
            .pop_back()
            .unwrap_or_else(|| unimplemented!("milled out"));
        let mut card = self.card_mut(&card_id)?;
        card.zone = Zone::Hand(*player_id);

        Ok(())
    }

    pub fn player_draw_n(&mut self, player_id: &PlayerId, n: u64) -> Result<(), StateError> {
        for _ in 0..n {
            self.player_draw(player_id)?;
        }

        Ok(())
    }

    pub fn player_card_mut(
        &mut self,
        player_id: &PlayerId,
        card_id: &CardId,
    ) -> Result<&mut Card, StateError> {
        self.cards_mut()
            .find(|card| matches!(card.zone, Zone::Hand(id) if id == *player_id))
            .ok_or(StateError::CardNotInHand(*player_id, *card_id))
    }

    pub(crate) fn cards_mut(&mut self) -> impl Iterator<Item = &mut Card> {
        self.objects.iter_mut().filter_map(|(_, obj)| {
            if let Object::Card { card, .. } = obj {
                Some(card)
            } else {
                None
            }
        })
    }

    pub(crate) fn permanents(&self) -> impl Iterator<Item = &Permanent> {
        self.objects.iter().filter_map(|(_, obj)| {
            if let Object::Permanent { permanent, .. } = obj {
                Some(permanent)
            } else {
                None
            }
        })
    }

    pub(crate) fn permanents_mut(&mut self) -> impl Iterator<Item = &mut Permanent> {
        self.objects.iter_mut().filter_map(|(_, obj)| {
            if let Object::Permanent { permanent, .. } = obj {
                Some(permanent)
            } else {
                None
            }
        })
    }

    pub(crate) fn player_cast(
        &mut self,
        db: &Database,
        player_id: &PlayerId,
        card_id: &CardId,
    ) -> Result<(), StateError> {
        let card = self.player_card_mut(&player_id, &card_id)?;
        card.zone = Zone::Stack;
        self.stack.push((*card_id).into());
        self.event_queue
            .push(Event::PlayerCastSpell(*player_id, (*card_id).into()));
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ability::{Ability, OneShotAbility};
    use crate::database::{CardData, CardRef};
    use crate::effect::Effect;

    #[test]
    fn test_cast_trigger() {
        let mut db = Database::default();
        db.cards.insert(
            CardRef("draw one".into()),
            CardData {
                abilities: vec![Ability::OneShot(OneShotAbility {
                    effect: Effect::Draw {
                        recipient: Default::default(),
                        amount: 1,
                    },
                })],
            },
        );

        let mut state = State::try_from(&Options::default()).unwrap();

        let library_id = LibraryId(1);
        state.libraries.insert(
            library_id,
            Library {
                id: library_id,
                card_ids: Default::default(),
            },
        );

        let player_id = PlayerId(1);
        state.players.insert(
            player_id,
            Player {
                id: player_id,
                library_id: library_id,
            },
        );

        let card_id = CardId::from(ObjectId(1));
        let card = Card {
            id: card_id,
            card_ref: CardRef("draw one".into()),
            zone: Zone::Hand(player_id),
        };
        state
            .objects
            .insert(card_id.into(), Object::Card { card: card });

        state.player_cast(&db, &player_id, &card_id).unwrap();

        assert_eq!(
            state.event_queue[0],
            Event::PlayerCastSpell(player_id, card_id.into())
        );

        // try creating a permanent with an on-cast trigger

        // trigger it with this cast

        // check the stack for correctness

        // resolve stack, check for correctness

        // check correctness of priority
    }
}
