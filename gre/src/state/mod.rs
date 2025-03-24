pub mod  procedure;
pub mod sba;
pub mod stack;
pub mod action;
pub mod card;
pub mod object;
pub mod player;
pub mod library;
pub mod priority;
pub mod event;
pub mod permanent;
pub mod ability;
pub mod effect;

use rand_core::SeedableRng;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;

use action::ActionError;
use event::Event;
use priority::Priority;
use crate::database::{Database, DatabaseError};
use crate::options::Options;
use crate::rng::GreRng;
use crate::state::action::{Action, PlayerAction};
use crate::state::card::{CardId};
use crate::state::library::{Library, LibraryId};
use crate::state::object::{Object, ObjectId};
use crate::state::player::{Player, PlayerId};
use crate::state::procedure::Phase;
use crate::state::stack::Stack;

#[derive(Debug, Clone, Hash, Default, Serialize, Deserialize)]
pub struct State {
    pub(crate) options: Options,
    pub(crate) rng: GreRng,
    pub(crate) stack: Stack,
    pub(crate) phase: Phase,
    pub(crate) priority: Priority,
    pub(crate) event_queue: Vec<Event>,
    pub(crate) players: BTreeMap<PlayerId, Player>,
    pub(crate) libraries: BTreeMap<LibraryId, Library>,
    pub(crate) seating: Vec<PlayerId>,
    pub(crate) objects: BTreeMap<ObjectId, Object>,
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
            rng,
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

        state.state_based_actions(db)?;

        // after state based actions, a player must have priority or the game must be over

        Ok(state)
    }

    pub fn valid_actions(&self) -> Vec<Action> {
        vec![]
    }

    pub fn is_terminal(&self) -> bool {
        false
    }
}

impl State {


}
