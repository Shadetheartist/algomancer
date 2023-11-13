use serde::{Deserialize, Serialize};

use crate::game::state::{GameMode, State};
use crate::game::state::card::CardId;
use crate::game::state::card_collection::CardCollection;
use crate::game::state::deck::Deck;
use crate::game::state::progression::{CombatPhaseAStep, CombatPhaseBStep, MainPhaseStep, Phase, PrecombatPhaseStep};
use crate::game::state::region::{RegionId};

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub struct TeamId(pub u8);

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub struct PlayerId(pub u8);

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Player {
    pub player_id: PlayerId,
    pub team_id: TeamId,
    pub pack: Option<CardCollection>,
    pub player_deck: Option<Deck>,
    pub is_alive: bool,
    pub health: i32,
    pub hand: CardCollection,
    pub discard: CardCollection,
    pub passed_priority: bool,
    pub resources_played_this_turn: u8,
}

impl Player {
    pub fn new(player_id: PlayerId,team_id: TeamId, deck: Option<Deck>, pack: Option<CardCollection>) -> Player {
        Player {
            player_id,
            team_id,
            player_deck: deck,
            is_alive: true,
            health: 30,
            hand: CardCollection::new_hand(player_id),
            discard: CardCollection::new_discard(player_id),
            passed_priority: false,
            pack: pack,
            resources_played_this_turn: 0,
        }
    }
}

#[derive(Debug)]
pub enum CardNotPlayableError {
    CardDoesNotExist,
    NotInPlayableZone,
    NotInPlayableStep,
    CardLacksCorrectTiming,
    CannotPlayMoreResources,
    MustBePlayedFromHand,
    CannotCastANonSpellTokenPermanentFromPlay,
}


#[derive(Debug)]
pub enum StateError {
    PlayerNotFound(PlayerId),
    RegionNotFound(RegionId),
    InvalidDraft,
    InvalidRecycle,
    NoPlayersOnTeam(TeamId),
    CardNotPlayable(CardNotPlayableError),
    MutationError,
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

    pub fn player_hand_mut(&mut self, player_id: PlayerId) -> &mut CardCollection {
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

