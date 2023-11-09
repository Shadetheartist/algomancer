use std::cmp::Ordering;
use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::game::Game;
use crate::game::state::card::CardId;
use crate::game::state::card::CardType::Resource;
use crate::game::state::formation::Formation;
use crate::game::state::permanent::PermanentId;
use crate::game::state::player::{PlayerId, StateError};
use crate::game::state::progression::{CombatPhaseAStep, MainPhaseStep, PrecombatPhaseStep};
use crate::game::state::progression::Phase::{CombatPhaseA, MainPhase, PrecombatPhase};
use crate::game::state::region::RegionId;
use crate::game::state::resource::ResourceType;

mod draft;
mod pass_priority;
mod mana_phase_actions;
mod play_card;
mod combat;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub enum Action {
    // resolves the next stack item, if there are no stack items, it passes priority
    // Once both players pass priority consecutively, the game moves to the next step or phase.
    PassPriority(PlayerId),

    // a player selects a hand of cards from a draft pack, leaving 10 cards in the pack
    Draft { player_id: PlayerId, cards_to_keep: Vec<CardId> },

    // a card is recycled in exchange for a resource
    RecycleForResource { card_id: CardId, resource_type: ResourceType},

    // a card is played
    PlayCard { card_id: CardId },

    Attack { home_region_id: RegionId, under_attack_region_id: RegionId, formation: Formation<PermanentId> },
}


impl PartialOrd for Action {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Action {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Action::PassPriority(_) => {
                match other {
                    Action::PassPriority(_) => Ordering::Equal,
                    _ => Ordering::Less,
                }
            }
            _ => {
                Ordering::Equal
            }
        }
    }
}

pub enum ActionValidationError {
    PlayerDoesNotExist,
    Draft(DraftValidationError),
}

pub enum DraftValidationError {
    IncorrectNumberOfCardsDrafted,
    CardNotInHand(CardId),
    InvalidPackCard(CardId),
}

impl Game {
    pub fn apply_action(&mut self, action: Action) -> Result<(), StateError> {
        if let Err(_) = self.validate_action(&action) {
            panic!("cannot apply this action, it is not valid");
        };

        eprintln!("Applying Action [{:?}]", &action);

        let mut next_state = self.state.clone();

        // action routing
        match action {
            Action::PassPriority(_) => {
                next_state = self.apply_pass_priority_action(next_state, &action)?;
            }
            Action::Draft { .. } => {
                next_state = self.apply_draft_action(next_state, &action)?;
            }
            Action::RecycleForResource { .. } => {
                next_state = self.apply_recycle_for_resource_action(next_state, &action)?;
            }
            Action::PlayCard { .. } => {
                next_state = self.apply_play_card_action(next_state, &action)?;
            }
            Action::Attack { .. } => {
                next_state = self.apply_attack_action(next_state, &action)?;
            }
        }

        self.action_history.push(action);
        self.state = next_state;

        Ok(())
    }

    pub fn validate_action(&self, action: &Action) -> Result<(), ActionValidationError> {
        match action {
            Action::PassPriority(_) => {}
            Action::Draft { player_id, cards_to_keep } => {
                return match self.state.find_player(*player_id) {
                    Err(_) => {
                        Err(ActionValidationError::PlayerDoesNotExist)
                    }
                    Ok(player) => {
                        if player.hand.cards.len() - cards_to_keep.len() != 10 {
                            // enforce that there must be 10 cards remaining to create the next pack
                            return Err(ActionValidationError::Draft(DraftValidationError::IncorrectNumberOfCardsDrafted));
                        }

                        // enforce that each card selected actually exists in the player's hand
                        for card_id in cards_to_keep {
                            if player.hand.cards.iter().find(|c| c.card_id == *card_id) == None {
                                return Err(ActionValidationError::Draft(DraftValidationError::CardNotInHand(*card_id)));
                            }
                        }

                        let cards_for_pack = player.hand.cards.iter().filter(|c| !cards_to_keep.contains(&c.card_id));

                        // enforce that each card left for the pack is not a resource
                        for card in cards_for_pack {
                            let proto = &self.cards_db.prototypes[&card.prototype_id];
                            if let Resource(_) = proto.card_type {
                                return Err(ActionValidationError::Draft(DraftValidationError::InvalidPackCard(card.card_id)));
                            }

                        }
                        Ok(())
                    }
                };
            }
            Action::RecycleForResource { .. } => {}
            Action::PlayCard { .. } => {}
            Action::Attack { .. } => {}
        }

        Ok(())
    }

    pub fn valid_actions(&self) -> HashSet<Action> {
        let mut valid_actions = HashSet::new();

        for region in &self.state.regions {
            match &region.step {

                PrecombatPhase(PrecombatPhaseStep::Draft) => {
                    for a in self.valid_drafts(region.region_id) {
                        valid_actions.insert(a);
                    }
                }

                PrecombatPhase(PrecombatPhaseStep::PassPack) => {
                    // no players can take any actions during this step
                    // after the last player drafts, all regions are automatically transitioned
                }

                PrecombatPhase(PrecombatPhaseStep::ITMana) | PrecombatPhase(PrecombatPhaseStep::NITMana) => {
                    for a in self.valid_mana_phase_actions(region.region_id) {
                        valid_actions.insert(a);
                    }
                }

                CombatPhaseA(CombatPhaseAStep::ITAttack) => {
                    for a in self.valid_attack_actions(region.region_id) {
                        valid_actions.insert(a);
                    }
                }

                MainPhase(MainPhaseStep::NITMain) => {
                    // dont put a valid action, so that during testing
                    // the game sim stops after one round
                }

                _ => {
                    for p in &region.players {
                        if self.state.player_can_act(p.player_id) {
                            valid_actions.insert(Action::PassPriority(p.player_id));
                        }
                    }
                }
            }
        }


        valid_actions
    }
}