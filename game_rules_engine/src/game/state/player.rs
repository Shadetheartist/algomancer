use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

use crate::game::state::card_collection::{CardCollectionId};
use crate::game::state::error::{EntityNotFoundError, StateError};
use crate::game::state::progression::{CombatPhaseAStep, CombatPhaseBStep, MainPhaseStep, Phase, PrecombatPhaseStep};
use crate::game::state::{GameMode, State};
use crate::game::state::deck::Deck;
use crate::game::state::stack::Next;
use crate::game::state::unordered_cards::UnorderedCards;

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub struct TeamId(pub u8);

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub struct PlayerId(pub u8);

impl Display for PlayerId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Player {
    pub id: PlayerId,
    pub team_id: TeamId,
    pub pack: Option<UnorderedCards>,
    pub own_deck: Option<Deck>,
    pub is_alive: bool,
    pub health: i32,
    pub hand: UnorderedCards,
    pub discard: UnorderedCards,
    pub resources_played_this_turn: u8,
}

impl Player {
    pub fn new(player_id: PlayerId, team_id: TeamId, deck: Option<Deck>, pack: Option<UnorderedCards>) -> Player {
        Player {
            id: player_id,
            team_id,
            own_deck: deck,
            is_alive: true,
            health: 30,
            hand: UnorderedCards::new(CardCollectionId::new_hand(player_id)),
            discard: UnorderedCards::new(CardCollectionId::new_discard(player_id)),
            pack,
            resources_played_this_turn: 0,
        }
    }

    pub fn deck<'a>(&'a self, state: &'a State) -> &'a Deck {
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
    pub fn find_player(&self, player_id: PlayerId) -> Result<&Player, EntityNotFoundError> {
        let find_result = self.players().find(|p| p.id == player_id);
        match find_result {
            None => {
                Err(EntityNotFoundError::Player(player_id))
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
        let region_id = self.find_region_id_containing_player(player_id);
        let region = self.find_region(region_id).expect("a region");

        match region.stack.next() {
            Next::PassPriority(active_player_id) => {
                if active_player_id != player_id {
                    return false
                }
            }
            _ => {}
        }

        let initiative_team = self.initiative_team();
        
        let has_initiative_action_window = {
            // otherwise if the player is on the initiative team (and implicitly, has not passed)
            if initiative_team == player.team_id {
                true
            } else {
                // otherwise if the player is on the non-initiative team,
                // then they cannot act until all the players on the initiative team have passed
                // self.all_players_on_team_passed_priority(initiative_team).expect("a result")
                false
            }
        };

        match &region.step {
            Phase::PrecombatPhase(step) => match step {
                PrecombatPhaseStep::ITMana => initiative_team == player.team_id,
                PrecombatPhaseStep::NITMana => initiative_team != player.team_id,
                _ => true
            }
            Phase::CombatPhaseA(step) => match step {
                CombatPhaseAStep::ITAttack => initiative_team == player.team_id,
                CombatPhaseAStep::AfterITAttackPriorityWindow => has_initiative_action_window,
                CombatPhaseAStep::NITBlock => initiative_team != player.team_id,
                CombatPhaseAStep::AfterNITBlockPriorityWindow => has_initiative_action_window,
                CombatPhaseAStep::Damage => false,
                CombatPhaseAStep::AfterCombatPriorityWindow => has_initiative_action_window,
            },
            Phase::CombatPhaseB(step) => match step {
                CombatPhaseBStep::NITAttack => initiative_team != player.team_id,
                CombatPhaseBStep::AfterNITAttackPriorityWindow => has_initiative_action_window,
                CombatPhaseBStep::ITBlock => initiative_team == player.team_id,
                CombatPhaseBStep::AfterITBlockPriorityWindow => has_initiative_action_window,
                CombatPhaseBStep::Damage => false,
                CombatPhaseBStep::AfterCombatPriorityWindow => has_initiative_action_window,
            },
            Phase::MainPhase(step) => match step {
                MainPhaseStep::Regroup => false,
                MainPhaseStep::ITMain => initiative_team == player.team_id,
                MainPhaseStep::NITMain => initiative_team != player.team_id
            }
        }

    }

    pub fn initiative_team(&self) -> TeamId {
        self.find_player(self.initiative_player).unwrap().team_id
    }

    pub fn non_initiative_team(&self) -> TeamId {
        self.team_ids().into_iter().find(|&t| t != self.initiative_team()).expect("a non-initative team")
    }

}


// mutable methods relating to state
impl State {
    pub fn find_player_mut(&mut self, player_id: PlayerId) -> Result<&mut Player, EntityNotFoundError> {
        let mut players_mut = self.regions.iter_mut().flat_map(|r| &mut r.players);
        let find_result = players_mut.find(|p| p.id == player_id);
        match find_result {
            None => {
                Err(EntityNotFoundError::Player(player_id))
            }
            Some(player) => {
                Ok(player)
            }
        }
    }


    pub fn player_draw_n_cards(&mut self, player_id: PlayerId, n: usize){

        let deck = self.player_deck(player_id).expect("a deck");
        let mut cards = Vec::new();
        for _ in 0..n {
            let card = deck.draw().expect("a card");
            cards.push(card);
        }

        let player = self.find_player_mut(player_id).expect("a player");
        for card in cards {
            player.hand.add(card);
        }
    }

    pub fn player_deck(&mut self, player_id: PlayerId) -> Result<&mut Deck, StateError> {
        match &self.game_mode {
            GameMode::LiveDraft { .. } => {
                if let Some(common_deck) = &mut self.common_deck {
                    Ok(common_deck)
                } else {
                    panic!("player is supposed to draw from the common deck in live-draft, but it doesn't exist");
                }
            },
            GameMode::PreDraft { .. } | GameMode::Constructed { .. } => {
                let player = self.find_player_mut(player_id).expect("player");
                if let Some(player_deck) = player.own_deck.as_mut() {
                    Ok(player_deck)
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