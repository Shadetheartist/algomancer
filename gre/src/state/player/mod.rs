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

    pub fn player_card_mut(
        &mut self,
        player_id: &PlayerId,
        card_id: &CardId,
    ) -> Result<&mut Card, StateError> {
        self.cards_mut()
            .find(|card| matches!(card.zone, Zone::Hand(id) if id == *player_id))
            .ok_or(StateError::CardNotInHand(*player_id, *card_id))
    }

    pub fn player_draw(&mut self, player_id: &PlayerId) -> Result<(), StateError> {
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

    pub fn player_draw_n(&mut self, player_id: &PlayerId, n: u64) -> Result<(), StateError> {
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
}
