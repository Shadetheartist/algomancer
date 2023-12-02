use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

use crate::game::state::card_collection::{CardCollectionId};
use crate::game::state::error::{EntityNotFoundError, StateError};
use crate::game::state::progression::{CombatPhaseStep, MainPhaseStep, Phase, PrecombatPhaseStep, Team};
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
            }
            GameMode::PreDraft { .. } | GameMode::Constructed { .. } => {
                if let Some(own_deck) = &self.own_deck {
                    own_deck
                } else {
                    panic!("player is supposed to draw from their own deck in pre-draft & constructed, but it doesn't exist");
                }
            }
            GameMode::TeamDraft { .. } => {
                // weird, this needs a common deck per team i guess
                todo!("need to implement team draft, which deck the player is drawing from")
            }
        }
    }
}


impl State {
    /// create an iterator over all the players in the game
    pub fn players(&self) -> impl Iterator<Item=&Player> {
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
        let region = self.find_region_containing_player(player_id).expect("the player must be in a region");
        let player = self.find_player(player_id).expect("the player was supposed to be in this region");

        let player_is_on_initiative_team = player.team_id == self.initiative_team();

        let active_on_stack = match region.stack.next() {
            Next::PassPriority(active_player_id) => {
                active_player_id == player_id
            }
            _ => false
        };

        match region.step {
            Phase::PrecombatPhase(step) => {
                match step {
                    // these are async
                    PrecombatPhaseStep::Untap => true,
                    PrecombatPhaseStep::Draw => true,

                    // during the draft step, players implicitly pass priority by selecting a draft
                    PrecombatPhaseStep::Draft => false,

                    // the pass pack is a global sync step, you can't pass priority here
                    // players instead wait for the last player to reach this step, then all regions
                    // transition to the next step automatically
                    PrecombatPhaseStep::PassPack => false,

                    PrecombatPhaseStep::Mana(Team::IT) => player_is_on_initiative_team && active_on_stack,
                    PrecombatPhaseStep::Mana(Team::NIT) => !player_is_on_initiative_team && active_on_stack,
                    PrecombatPhaseStep::Haste(Team::IT) => player_is_on_initiative_team && active_on_stack,
                    PrecombatPhaseStep::Haste(Team::NIT) => !player_is_on_initiative_team && active_on_stack,
                }
            }
            Phase::CombatPhaseA(step) => {
                match step {
                    CombatPhaseStep::Attack(Team::IT) => player_is_on_initiative_team && active_on_stack,
                    CombatPhaseStep::AfterAttackPriorityWindow => active_on_stack,
                    CombatPhaseStep::Block(Team::NIT) => !player_is_on_initiative_team && active_on_stack,
                    CombatPhaseStep::AfterBlockPriorityWindow => active_on_stack,

                    // this step happens, but is really just a step where mutations are applied,
                    // players don't take any actions
                    CombatPhaseStep::Damage => false,
                    CombatPhaseStep::AfterCombatPriorityWindow => active_on_stack,
                    _ => { panic!("weird phase") }
                }
            }
            Phase::CombatPhaseB(step) => {
                match step {
                    CombatPhaseStep::Attack(Team::NIT) => !player_is_on_initiative_team && active_on_stack,
                    CombatPhaseStep::AfterAttackPriorityWindow => active_on_stack,
                    CombatPhaseStep::Block(Team::IT) => player_is_on_initiative_team && active_on_stack,
                    CombatPhaseStep::AfterBlockPriorityWindow => active_on_stack,

                    // this step happens, but is really just a step where mutations are applied,
                    // players don't take any actions
                    CombatPhaseStep::Damage => false,
                    CombatPhaseStep::AfterCombatPriorityWindow => active_on_stack,
                    _ => { panic!("weird phase") }
                }
            }
            Phase::MainPhase(step) => {
                match step {
                    // just a cleanup step, no user interaction
                    MainPhaseStep::Regroup => false,
                    MainPhaseStep::Main(Team::IT) => player_is_on_initiative_team && active_on_stack,
                    MainPhaseStep::Main(Team::NIT) => !player_is_on_initiative_team && active_on_stack,
                }
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


    pub fn player_draw_n_cards(&mut self, player_id: PlayerId, n: usize) {
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
            }
            GameMode::PreDraft { .. } | GameMode::Constructed { .. } => {
                let player = self.find_player_mut(player_id).expect("player");
                if let Some(player_deck) = player.own_deck.as_mut() {
                    Ok(player_deck)
                } else {
                    panic!("player is supposed to draw from their own deck in pre-draft & constructed, but it doesn't exist");
                }
            }
            GameMode::TeamDraft { .. } => {
                // weird, this needs a common deck per team i guess
                todo!("need to implement team draft, which deck the player is drawing from")
            }
        }
    }
}