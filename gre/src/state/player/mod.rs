use crate::database::Database;
use crate::state::card::{Card, CardId};
use crate::state::event::Event;
use crate::state::library::LibraryId;
use crate::state::{State, StateError};
use crate::zone::Zone;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct PlayerId(pub usize);

impl Display for PlayerId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("P_{}", self.0))
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Player {
    pub id: PlayerId,
    pub library_id: LibraryId,
    pub alive: bool,
}

impl Player {
    pub fn new(id: PlayerId, library_id: LibraryId) -> Self {
        Self {
            id,
            library_id,
            alive: true,
        }
    }
}

impl State {

    pub(crate) fn add_player(&mut self, player: Player, seat_number: usize) -> Result<(), StateError> {
        assert!(self.seating.len() >= seat_number, "seat does not exist");
        assert_eq!(self.seating.get(seat_number).unwrap().player_id, None, "seat not available");

        let player_id = player.id;
        self.players.insert(player_id, player);
        self.seating[seat_number].player_id = Some(player_id);

        Ok(())
    }

    pub(crate) fn get_player(&self, player_id: &PlayerId) -> Result<&Player, StateError> {
        self.players
            .get(player_id)
            .ok_or(StateError::PlayerDoesNotExist(*player_id))
    }

    pub(crate) fn get_player_mut(
        &mut self,
        player_id: &PlayerId,
    ) -> Result<&mut Player, StateError> {
        self.players
            .get_mut(player_id)
            .ok_or(StateError::PlayerDoesNotExist(*player_id))
    }

    pub(crate) fn player_card_mut(
        &mut self,
        player_id: &PlayerId,
        card_id: &CardId,
    ) -> Result<&mut Card, StateError> {
        self.cards_mut()
            .find(|card| matches!(card.zone, Zone::Hand(id) if id == *player_id))
            .ok_or(StateError::CardNotInHand(*player_id, *card_id))
    }

    pub(crate) fn player_draw(&mut self, player_id: &PlayerId) -> Result<(), StateError> {
        let library_id = self.get_player(player_id)?.library_id;
        let library = self.library_mut(&library_id)?;
        let card_id = library
            .card_ids
            .pop_back()
            .unwrap_or_else(|| unimplemented!("milled out"));
        let card = self.card_mut(&card_id)?;
        card.zone = Zone::Hand(*player_id);

        Ok(())
    }

    pub(crate) fn player_draw_n(&mut self, player_id: &PlayerId, n: u64) -> Result<(), StateError> {
        for _ in 0..n {
            self.player_draw(player_id)?;
        }

        Ok(())
    }

    pub(crate) fn player_cast(
        &mut self,
        _: &Database,
        player_id: &PlayerId,
        card_id: &CardId,
    ) -> Result<(), StateError> {
        let card = self.player_card_mut(player_id, card_id)?;
        card.zone = Zone::Stack;
        self.stack.push((*card_id).into());
        self.event_queue
            .push(Event::PlayerCastSpell(*player_id, (*card_id).into()));
        Ok(())
    }

    pub(crate) fn pass_priority(&mut self, _: &Database) -> Result<(), StateError> {
        let currently_prioritized_player = self
            .priority
            .0
            .expect("a player must have priority in order to pass it");

        let current_player_seat_idx = self
            .seating
            .iter()
            .enumerate()
            .find(|(_, seat)| {
                if let Some(player_id) = &seat.player_id {
                    *player_id == currently_prioritized_player
                } else {
                    false
                }
            })
            .expect("player with priority does not exist in seating")
            .0;

        let next_player_id = {
            let mut next_seat_idx = current_player_seat_idx;
            loop {
                next_seat_idx = (next_seat_idx + 1) % self.seating.len();
                let player_id = self.seating[next_seat_idx].player_id;
                if let Some(player_id) = player_id {
                    let player = self.get_player(&player_id).expect("seated player does not exist");
                    if player.alive {
                        break player_id;
                    }
                }
            }
        };

        assert_ne!(
            currently_prioritized_player, next_player_id,
            "the next living player with priority can't be the current player with priority"
        );

        self.priority.0 = Some(next_player_id);

        Ok(())
    }
}
