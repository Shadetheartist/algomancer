use serde::{Deserialize, Serialize};

use crate::game::state::card_collection::CardCollection;
use crate::game::state::error::StateError;
use crate::game::state::progression::{CombatPhaseAStep, CombatPhaseBStep, MainPhaseStep, Phase, PrecombatPhaseStep};
use crate::game::state::{GameMode, State};

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub struct TeamId(pub u8);

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub struct PlayerId(pub u8);

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Player {
    pub id: PlayerId,
    pub team_id: TeamId,
    pub pack: Option<CardCollection>,
    pub own_deck: Option<CardCollection>,
    pub is_alive: bool,
    pub health: i32,
    pub hand: CardCollection,
    pub discard: CardCollection,
    pub passed_priority: bool,
    pub resources_played_this_turn: u8,
}

impl Player {
    pub fn new(player_id: PlayerId,team_id: TeamId, deck: Option<CardCollection>, pack: Option<CardCollection>) -> Player {
        Player {
            id: player_id,
            team_id,
            own_deck: deck,
            is_alive: true,
            health: 30,
            hand: CardCollection::new_hand(player_id),
            discard: CardCollection::new_discard(player_id),
            passed_priority: false,
            pack,
            resources_played_this_turn: 0,
        }
    }

    pub fn deck<'a>(&'a self, state: &'a State) -> &'a CardCollection {
        match state.game_mode {
            GameMode::LiveDraft { .. } => {
                if let Some(common_deck) = &state.common_deck {
                    common_deck
                } else {
                    panic!("player is supposed to draw from the common deck in live-draft, but it doesn't exist");
                }
            },
            GameMode::PreDraft { .. } | GameMode::Constructed { .. } => {
                if let Some(own_deck) = &self.own_deck {
                    own_deck
                } else {
                    panic!("player is supposed to draw from their own deck in pre-draft & constructed, but it doesn't exist");
                }
            },
            GameMode::TeamDraft { .. } => {
                // weird, this needs a common deck per team i guess
                todo!("need to implement team draft, which deck the player is drawing from")
            }
        }
    }
}


impl State {

    /// create an iterator over all the players in the game
    pub fn players(&self) -> impl Iterator<Item = &Player> {
        self.regions.iter().flat_map(|r| &r.players)
    }

    /// looks through all regions for a player matching the player_id
    pub fn find_player(&self, player_id: PlayerId) -> Result<&Player, StateError> {
        let find_result = self.players().find(|p| p.id == player_id);
        match find_result {
            None => {
                Err(StateError::PlayerNotFound(player_id))
            }
            Some(player) => {
                Ok(player)
            }
        }
    }

    pub fn living_players_in_team(&self, team_id: TeamId) -> Vec<&Player> {
        self.players().filter(|p| p.team_id == team_id && p.is_alive).collect()
    }

    pub fn team_ids(&self) -> Vec<TeamId> {
        self.players().fold(Vec::new(), |mut acc, player| {
            // add the team to the list if it's no already there
            if !acc.iter().any(|t_id| *t_id == player.team_id) {
                acc.push(player.team_id)
            }
            acc
        })
    }

    /// Returns true if the player is capable of any actions during the current step in their region.
    /// This considers which team has initiative, and what step the player is experiencing,
    /// as well as if they are waiting to receive priority during an action window
    pub fn player_can_act(&self, player_id: PlayerId) -> bool {
        let player = self.find_player(player_id).expect("a player");

        // if the player has passed priority then they cannot do anything until they receive priority again.
        if player.passed_priority {
            return false
        }

        let region_id = self.find_region_id_containing_player(player_id);
        let region = self.find_region(region_id).expect("a region");

        let has_initiative_action_window = {
            // otherwise if the player is on the initiative team (and implicitly, has not passed)
            if self.initiative_team == player.team_id {
                true
            } else {
                // otherwise if the player is on the non-initiative team,
                // then they cannot act until all the players on the initiative team have passed
                self.all_players_on_team_passed_priority(self.initiative_team).expect("a result")
            }
        };

        match &region.step {
            Phase::PrecombatPhase(step) => match step {
                PrecombatPhaseStep::ITMana => self.initiative_team == player.team_id,
                PrecombatPhaseStep::NITMana => self.initiative_team != player.team_id,
                _ => true
            }
            Phase::CombatPhaseA(step) => match step {
                CombatPhaseAStep::ITAttack => self.initiative_team == player.team_id,
                CombatPhaseAStep::AfterITAttackPriorityWindow => has_initiative_action_window,
                CombatPhaseAStep::NITBlock => self.initiative_team != player.team_id,
                CombatPhaseAStep::AfterNITBlockPriorityWindow => has_initiative_action_window,
                CombatPhaseAStep::Damage => false,
                CombatPhaseAStep::AfterCombatPriorityWindow => has_initiative_action_window,
            },
            Phase::CombatPhaseB(step) => match step {
                CombatPhaseBStep::NITAttack => self.initiative_team != player.team_id,
                CombatPhaseBStep::AfterNITAttackPriorityWindow => has_initiative_action_window,
                CombatPhaseBStep::ITBlock => self.initiative_team == player.team_id,
                CombatPhaseBStep::AfterITBlockPriorityWindow => has_initiative_action_window,
                CombatPhaseBStep::Damage => false,
                CombatPhaseBStep::AfterCombatPriorityWindow => has_initiative_action_window,
            },
            Phase::MainPhase(step) => match step {
                MainPhaseStep::Regroup => false,
                MainPhaseStep::ITMain => self.initiative_team == player.team_id,
                MainPhaseStep::NITMain => self.initiative_team != player.team_id
            }
        }

    }


    pub fn non_initiative_team(&self) -> TeamId {
        self.team_ids().into_iter().find(|&t| t != self.initiative_team).expect("a non-initative team")
    }

}

