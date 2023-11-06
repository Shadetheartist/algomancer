use serde::{Deserialize, Serialize};

use crate::game::state::{GameMode, State};
use crate::game::state::card::CardId;
use crate::game::state::deck::Deck;
use crate::game::state::discard::Discard;
use crate::game::state::hand::Hand;
use crate::game::state::pack::Pack;
use crate::game::state::progression::{CombatPhaseAStep, CombatPhaseBStep, MainPhaseStep, Phase, PrecombatPhaseStep};
use crate::game::state::region::RegionId;

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub struct TeamId(pub u8);

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub struct PlayerId(pub u8);

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Player {
    pub player_id: PlayerId,
    pub team_id: TeamId,
    pub pack: Option<Pack>,
    pub player_deck: Option<Deck>,
    pub is_alive: bool,
    pub health: i32,
    pub hand: Hand,
    pub discard: Discard,
    pub passed_priority: bool,
}

impl Player {
    pub fn new(player_id: PlayerId,team_id: TeamId, deck: Option<Deck>, pack: Option<Pack>) -> Player {
        Player {
            player_id,
            team_id,
            player_deck: deck,
            is_alive: true,
            health: 30,
            hand: Hand::new(),
            discard: Discard::new(),
            passed_priority: false,
            pack: pack,
        }
    }
}

#[derive(Debug)]
pub enum StateError {
    PlayerNotFound(PlayerId),
    RegionNotFound(RegionId),
    InvalidDraft,
    InvalidRecycle,
    NoPlayersOnTeam(TeamId),
}

impl State {
    /// looks through all regions for a player matching the player_id
    pub fn find_player(&self, player_id: PlayerId) -> Result<&Player, StateError> {
        let find_result = self.players().into_iter().find(|p| p.player_id == player_id);
        match find_result {
            None => {
                Err(StateError::PlayerNotFound(player_id))
            }
            Some(player) => {
                Ok(player)
            }
        }
    }

    pub fn find_player_mut(&mut self, player_id: PlayerId) -> Result<&mut Player, StateError> {
        let find_result = self.players_mut().into_iter().find(|p| p.player_id == player_id);
        match find_result {
            None => {
                Err(StateError::PlayerNotFound(player_id))
            }
            Some(player) => {
                Ok(player)
            }
        }
    }

    pub fn player_hand_mut(&mut self, player_id: PlayerId) -> &mut Hand {
        &mut self.find_player_mut(player_id).expect("a player").hand
    }

    pub fn get_deck_for_player(&mut self, player_id: PlayerId) -> Result<&mut Deck, StateError> {
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
                if let Some(player_deck) = player.player_deck.as_mut() {
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

    pub fn players(&self) -> Vec<&Player> {
        self.regions.iter().flat_map(|r| &r.players).collect()
    }

    pub fn players_mut(&mut self) -> Vec<&mut Player> {
        self.regions.iter_mut().flat_map(|r| &mut r.players).collect()
    }

    pub fn living_players_in_team(&self, team_id: TeamId) -> Vec<&Player> {
        self.players().into_iter().filter(|p| p.team_id == team_id && p.is_alive).collect()
    }

    pub fn team_ids(&self) -> Vec<TeamId> {
        self.players().into_iter().fold(Vec::new(), |mut acc, player| {
            // add the team to the list if it's no already there
            if acc.iter().find(|&t_id| *t_id == player.team_id) == None {
                acc.push(player.team_id)
            }
            acc
        })
    }

    pub fn player_draw_n_cards(&mut self, player_id: PlayerId, n: usize){

        let deck = self.get_deck_for_player(player_id).expect("a deck");
        let mut cards = Vec::new();
        for _ in 0..n {
            let card = deck.draw().expect("a card");
            cards.push(card);
        }

        let player = self.find_player_mut(player_id).expect("a player");
        for card in cards {
            player.hand.cards.push(card);
        }
    }

    pub fn player_recycle_card(&mut self, player_id: PlayerId, card_id: CardId){

        // remove the card from the player's hand
        let card = {
            let player = self.find_player_mut(player_id).expect("a player");
            let card_idx = player.hand.cards.iter().position(|c| c.card_id == card_id).expect("a card in hand");
            player.hand.cards.remove(card_idx)
        };

        // add the removed card to the bottom of the deck
        let deck = self.get_deck_for_player(player_id).expect("a deck");
        deck.cards.push(card);
    }

    /// Returns true if the player is capable of any actions during the current step in their region.
    /// This considers which team has initiative, and what step the player is experiencing
    pub fn player_can_act(&self, player_id: PlayerId) -> bool {
        let player = self.find_player(player_id).expect("a player");
        let region_id = self.find_region_id_containing_player(player_id);
        let region = self.find_region(region_id).expect("a region");

        let is_initiative_team = self.initiative_team == player.team_id;

        match &region.step {
            Phase::PrecombatPhase(step) => match step {
                PrecombatPhaseStep::ITMana => is_initiative_team,
                PrecombatPhaseStep::NITMana => !is_initiative_team,
                _ => true
            }
            Phase::CombatPhaseA(step) => match step {
                CombatPhaseAStep::ITPrepareFormation |
                CombatPhaseAStep::ITAttack => is_initiative_team,
                CombatPhaseAStep::AfterITAttackPriorityWindow => true,
                CombatPhaseAStep::NITBlock => !is_initiative_team,
                CombatPhaseAStep::AfterNITBlockPriorityWindow => true,
                CombatPhaseAStep::AfterCombatPriorityWindow => true,
                CombatPhaseAStep::Damage => false,
            },
            Phase::CombatPhaseB(step) => match step {
                CombatPhaseBStep::NITPrepareFormation |
                CombatPhaseBStep::NITAttack => !is_initiative_team,
                CombatPhaseBStep::AfterNITAttackPriorityWindow => true,
                CombatPhaseBStep::ITBlock => is_initiative_team,
                CombatPhaseBStep::AfterITBlockPriorityWindow => true,
                CombatPhaseBStep::Damage => false,
                CombatPhaseBStep::AfterCombatPriorityWindow => true,
            },
            Phase::MainPhase(step) => match step {
                MainPhaseStep::Regroup => false,
                MainPhaseStep::ITMain => is_initiative_team,
                MainPhaseStep::NITMain => !is_initiative_team
            }
        }

    }


    pub fn non_initiative_team(&self) -> TeamId {
        self.team_ids().into_iter().find(|&t| t != self.initiative_team).expect("a non-initative team")
    }

}

