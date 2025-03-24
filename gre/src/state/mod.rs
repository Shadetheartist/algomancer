pub mod ability;
pub mod action;
pub mod card;
pub mod effect;
pub mod event;
pub mod library;
pub mod object;
pub mod permanent;
pub mod player;
pub mod priority;
pub mod procedure;
pub mod sba;
mod seating;
pub mod stack;
mod seat;

use rand_core::SeedableRng;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;

use crate::database::{Database, DatabaseError};
use crate::game_mode::{GameMode, TeamConfiguration};
use crate::options::Options;
use crate::rng::GreRng;
use crate::state::action::{Action, PlayerAction};
use crate::state::card::CardId;
use crate::state::library::{Library, LibraryId};
use crate::state::object::{Object, ObjectId};
use crate::state::player::{Player, PlayerId};
use crate::state::procedure::Phase;
use crate::state::seating::interlace_players;
use crate::state::stack::Stack;
use action::ActionError;
use event::Event;
use priority::Priority;
use crate::state::seat::{Seat, TeamId};

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
    pub(crate) seating: Vec<Seat>,
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

        let seating = seating_from_game_mode(&options);

        let state = Self {
            options: options.clone(),
            rng,
            seating,
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

fn seating_from_game_mode(options: &&Options) -> Vec<Seat> {
    match &options.game_mode {
        GameMode::LiveDraft {
            team_configuration, ..
        } => match team_configuration {
            TeamConfiguration::Teams { teams_of_players } => {
                let teams = interlace_players(teams_of_players)
                    .into_iter()
                    .map(|v| v as usize)
                    .collect::<Vec<usize>>();

                teams.iter().enumerate().map(|(idx, t)| Seat {
                    seat_number: idx,
                    team_id: TeamId(*t),
                    player_id: None,
                }).collect()
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

impl State {}
