use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use algocore::{Affinity, CardType, Cost, Faction, ResourceType};
use database::CardPrototypeDatabase;

use crate::game::state::card_collection::{CardCollectionId};
use crate::game::state::error::{EntityNotFoundError, StateError};
use crate::game::state::progression::{BattlePhaseStep, DeploymentPhaseStep, Phase, PlanningPhaseStep, Team};
use crate::game::state::{GameMode, State};
use crate::game::state::deck::Deck;
use crate::game::state::permanent::Permanent;
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
            Phase::PlanningPhase(step) => {
                match step {
                    // these are async
                    PlanningPhaseStep::Refresh => true,
                    PlanningPhaseStep::Draw => true,

                    // during the draft step, players implicitly pass priority by selecting a draft
                    PlanningPhaseStep::Draft => false,

                    // the pass pack is a global sync step, you can't pass priority here
                    // players instead wait for the last player to reach this step, then all regions
                    // transition to the next step automatically
                    PlanningPhaseStep::PassPack => false,

                    PlanningPhaseStep::Mana(Team::IT) => player_is_on_initiative_team && active_on_stack,
                    PlanningPhaseStep::Mana(Team::NIT) => !player_is_on_initiative_team && active_on_stack,
                    PlanningPhaseStep::Haste(Team::IT) => player_is_on_initiative_team && active_on_stack,
                    PlanningPhaseStep::Haste(Team::NIT) => !player_is_on_initiative_team && active_on_stack,
                }
            }
            Phase::BattlePhaseA(step) => {
                match step {
                    BattlePhaseStep::Attack(Team::IT) => player_is_on_initiative_team && active_on_stack,
                    BattlePhaseStep::AfterAttackPriorityWindow => active_on_stack,
                    BattlePhaseStep::Block(Team::NIT) => !player_is_on_initiative_team && active_on_stack,
                    BattlePhaseStep::AfterBlockPriorityWindow => active_on_stack,

                    // this step happens, but is really just a step where mutations are applied,
                    // players don't take any actions
                    BattlePhaseStep::Damage => false,
                    BattlePhaseStep::AfterCombatPriorityWindow => active_on_stack,
                    _ => { panic!("weird phase") }
                }
            }
            Phase::BattlePhaseB(step) => {
                match step {
                    BattlePhaseStep::Attack(Team::NIT) => !player_is_on_initiative_team && active_on_stack,
                    BattlePhaseStep::AfterAttackPriorityWindow => active_on_stack,
                    BattlePhaseStep::Block(Team::IT) => player_is_on_initiative_team && active_on_stack,
                    BattlePhaseStep::AfterBlockPriorityWindow => active_on_stack,

                    // this step happens, but is really just a step where mutations are applied,
                    // players don't take any actions
                    BattlePhaseStep::Damage => false,
                    BattlePhaseStep::AfterCombatPriorityWindow => active_on_stack,
                    _ => { panic!("weird phase") }
                }
            }
            Phase::DeploymentPhase(step) => {
                match step {
                    // just a cleanup step, no user interaction
                    DeploymentPhaseStep::Regroup => false,
                    DeploymentPhaseStep::Deployment(Team::IT) => player_is_on_initiative_team && active_on_stack,
                    DeploymentPhaseStep::Deployment(Team::NIT) => !player_is_on_initiative_team && active_on_stack,
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

    /// returns resources in the region the player is in that the player is currently controlling
    pub fn player_resources(&self, player_id: PlayerId) -> Result<Vec<&Permanent>, EntityNotFoundError> {
        let resources = self.find_region_containing_player(player_id)?.resources().into_iter().filter(|permanent| {
            if let Permanent::Resource { common, .. } = permanent {
                common.controller_player_id == player_id
            } else {
                false
            }
        }).collect();

        Ok(resources)
    }

    /// gets the currently available mana for a player in the form of a vec of affinities and
    /// a 'total available mana' count
    pub fn player_available_mana(&self, db: &CardPrototypeDatabase, player_id: PlayerId) -> Result<(Vec<Affinity>, u32), EntityNotFoundError> {
        let player_resources = self.player_resources(player_id)?;

        let mut affinity_map: HashMap<Faction, u32> = HashMap::new();
        let mut total_mana: u32 = 0;

        for permanent in player_resources {
            if let Permanent::Resource { tapped, card_prototype_id, .. } = permanent {
                let card_type = &db.prototypes[card_prototype_id].card_type;
                if let CardType::Resource(resource_type) = card_type {
                    let faction = Faction::from_resource_type(*resource_type);

                    // if the resource type maps to a faction, add it to the affinity map
                    if let Some(faction) = faction {
                        *affinity_map.entry(faction).or_insert(0) += 1;
                    }

                    // if the resource is tapped, it can't contribute to total available mana
                    if !tapped {
                        // if the resource is dormant, it can't be tapped
                        if let ResourceType::Dormant = resource_type {} else {
                            total_mana += 1;
                        }
                    }
                } else {
                    panic!("permanent card type is not a resource")
                }
            } else {
                panic!("permanent is not a resource")
            }
        }

        Ok((affinity_map.into_iter().map(|e| Affinity {
            faction: e.0,
            quantity: e.1,
        }).collect(), total_mana))
    }

    /// checks if the player can currently afford some cost. Returns an enum describing if the
    /// player can afford the cost, or why they can't.
    pub fn player_check_affordability(&self, db: &CardPrototypeDatabase, player_id: PlayerId, cost: &Cost) -> Result<Affordability, EntityNotFoundError> {
        let available_mana = self.player_available_mana(db, player_id)?;

        // zero mana - can't cast shit
        if available_mana.1 == 0 {
            return Ok(Affordability::NotEnoughMana);
        }

        match cost {
            Cost::Standard { threshold, cost } => {
                // player straight-up does not have the mana to afford the cost
                if *cost > available_mana.1 {
                    return Ok(Affordability::NotEnoughMana);
                }

                for threshold_affinity in threshold {
                    let available_affinity = available_mana.0.iter().find(|available_affinity| available_affinity.faction == threshold_affinity.faction);
                    if let Some(available_affinity) = available_affinity {
                        if available_affinity.quantity < threshold_affinity.quantity {
                            return Ok(Affordability::DoesntMeetThreshold);
                        }
                    }
                }
            }
            Cost::X { .. } => {
                // not sure how to handle the X cost enum without a cost associated with it.
                // probably have to resolve the X in the cost and then create a new std cost.
            }
        }

        Ok(Affordability::Affordable)
    }

    /// returns true if the player can currently afford some cost.
    pub fn player_can_afford(&self, db: &CardPrototypeDatabase, player_id: PlayerId, cost: &Cost) -> Result<bool, EntityNotFoundError> {
        if let Affordability::Affordable = self.player_check_affordability(db, player_id, cost)? {
            return Ok(true);
        }

        Ok(false)
    }
}


pub enum Affordability {
    Affordable,
    NotEnoughMana,
    DoesntMeetThreshold,
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