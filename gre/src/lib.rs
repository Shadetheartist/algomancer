mod action;
mod card;
mod database;
mod history;
mod object;
mod options;
mod player;
mod rng;
mod state;
mod zone;
mod library;
mod priority;
mod event;
mod permanent;
mod ability;
mod effect;
mod resource_type;
mod faction;
mod cost;
mod affinity;
mod card_type;
mod timing;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::action::{ActionError};
use crate::state::StateError;
use history::HistoryItem;
use state::State;
use action::Action;

pub use crate::options::Options;
use crate::database::Database;

#[derive(Debug, Serialize, Deserialize)]
pub struct GameRulesEngine {
    pub(crate) state: State,
    pub(crate) database: Database,
    pub(crate) history: Vec<HistoryItem>,
}

impl GameRulesEngine {
    fn push_history_item(&mut self, action: &Action) {
        self.history
            .push(HistoryItem::from_state_action_pair(&self.state, action))
    }

    pub fn apply_action(&mut self, action: &Action) -> Result<(), GameRulesEngineError> {
        let next_state = self.state.apply_action(action, &self.database)?;
        self.push_history_item(action);
        self.state = next_state;
        Ok(())
    }

    pub fn valid_actions(&self) -> Vec<Action> {
        self.state.valid_actions()
    }
}

#[derive(Error, Debug)]
pub enum GameRulesEngineError {
    #[error("state error: {0}")]
    StateError(#[from] StateError),
    #[error("action error: {0}")]
    ActionError(#[from] ActionError),
}

impl GameRulesEngine {

    pub fn new_from_options(db: Database, options: &Options) -> Result<Self, GameRulesEngineError> {
        let gre = Self {
            state: State::try_from(options)?,
            database: db,
            history: Default::default(),
        };

        Ok(gre)
    }
}

#[cfg(test)]
mod tests {
    use crate::ability::{Ability, OneShotAbility};
    use crate::card::{Card, CardId};
    use crate::card_type::CardType;
    use crate::database::{Database, PaperCard, PaperCardId};
    use crate::effect::Effect;
    use crate::event::Event;
    use crate::options::Options;
    use crate::GameRulesEngine;
    use crate::library::{Library, LibraryId};
    use crate::object::{Object, ObjectId};
    use crate::player::{Player, PlayerId};
    use crate::zone::Zone;

    #[test]
    fn test() {
        let db = Database::from_path("../resources/core_cards.json").unwrap();
        let gre = GameRulesEngine::new_from_options(db, &Options::default()).unwrap();
    }


    fn test_scenario() -> GameRulesEngine {
        let options = Options { seed: 0 };

        let mut db = Database::default();
        let paper_card_id = PaperCardId("draw one".into());
        db.cards.insert(
            paper_card_id.clone(),
            PaperCard {
                id: paper_card_id.clone(),
                card_type: CardType::UnitToken,
                abilities: vec![Ability::OneShot(OneShotAbility {
                    effect: Effect::Draw {
                        recipient: Default::default(),
                        amount: 1,
                    },
                })],
            },
        );

        GameRulesEngine::new_from_options(db, &options).unwrap()
    }

    #[test]
    fn test_2() {
        let mut gre = test_scenario();
        let library_id = LibraryId(1);

        gre.state.libraries.insert(
            library_id,
            Library {
                id: library_id,
                card_ids: Default::default(),
            },
        );

        let player_id = PlayerId(1);
        gre.state.players.insert(
            player_id,
            Player {
                id: player_id,
                library_id,
            },
        );

        let card_id = CardId::from(ObjectId(1));
        let paper_card_id = PaperCardId("draw one".into());
        let card = Card {
            id: card_id,
            card_ref: paper_card_id,
            zone: Zone::Hand(player_id),
        };
        gre.state
            .objects
            .insert(card_id.into(), Object::Card { card });

        gre.state.player_cast(&gre.database, &player_id, &card_id).unwrap();

        assert_eq!(
            gre.state.event_queue[0],
            Event::PlayerCastSpell(player_id, card_id.into())
        );

    }
}
