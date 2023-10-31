use std::cmp::Ordering;
use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::game::Game;
use crate::game::state::card::CardId;
use crate::game::state::card::CardType::Resource;
use crate::game::state::player::{PlayerId, StateError};
use crate::game::state::progression::{MainPhaseStep, PrecombatPhaseStep};
use crate::game::state::progression::Phase::{MainPhase, PrecombatPhase};

mod draft;
mod pass_priority;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub enum Action {
    // resolves the next stack item, if there are no stack items, it passes priority
    // Once both players pass priority consecutively, the game moves to the next step or phase.
    PassPriority(PlayerId),

    // a player selects a hand of cards from a draft pack, leaving 10 cards in the pack
    Draft { player_id: PlayerId, cards_to_keep: Vec<CardId> },

    // a card is cast
    Cast(CardId),
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

        println!("Applying Action [{:?}]", &action);

        let mut next_state = self.state.clone();

        match action {
            Action::PassPriority(_) => {
                next_state = self.apply_pass_priority_action(next_state, &action)?;
            }
            Action::Draft { .. } => {
                next_state = self.apply_draft_action(next_state, &action)?;
            }

            Action::Cast(_) => {
                todo!()
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
                            return Err(ActionValidationError::Draft(DraftValidationError::IncorrectNumberOfCardsDrafted))
                        }

                        // enforce that each card selected actually exists in the player's hand
                        for card_id in cards_to_keep {
                            if player.hand.cards.iter().find(|c| c.card_id == *card_id) == None {
                                return Err(ActionValidationError::Draft(DraftValidationError::CardNotInHand(*card_id)))
                            }
                        }

                        let cards_for_pack = player.hand.cards.iter().filter(|c| !cards_to_keep.contains(&c.card_id));

                        // enforce that each card left for the pack is not a resource
                        for card in cards_for_pack {
                            let proto = &self.cards_db.prototypes[&card.prototype_id];
                            if proto.card_type == Resource {
                                return Err(ActionValidationError::Draft(DraftValidationError::InvalidPackCard(card.card_id)))
                            }
                        }

                        Ok(())
                    }
                };
            }
            Action::Cast(_) => {}
        }

        Ok(())
    }

    pub fn valid_actions(&self) -> HashSet<Action> {
        let mut valid_actions = HashSet::new();

        for region in &self.state.regions {

            match &region.step {
                MainPhase(MainPhaseStep::NITMain) => {
                    // dont put a valid action, for testing
                }

                PrecombatPhase(PrecombatPhaseStep::Draft) => {
                    for p in &region.players {
                        for a in self.valid_drafts(p.player_id) {
                            valid_actions.insert(a);
                        }

                        if !p.passed_priority {
                            valid_actions.insert(Action::PassPriority(p.player_id));
                        }
                    }
                }

                // can only pass priority in the pass pack step when the clockwise neighbour can accept the pack,
                // as they have integrated theirs into their hand during their draft step
                // once all players have passed priority, they should be synchronized to enter the mana step
                PrecombatPhase(PrecombatPhaseStep::PassPack) => {
                    let neighbour_region = self.state.region_clockwise_neighbour(region.region_id).expect("a neighbouring region");
                    if let PrecombatPhase(step) = &neighbour_region.step {
                        if let PrecombatPhaseStep::Draft | PrecombatPhaseStep::PassPack = step {
                            for p in &region.players {
                                if !p.passed_priority {
                                    valid_actions.insert(Action::PassPriority(p.player_id));
                                }
                            }
                        }
                    }
                }

                _ => {
                    if !self.is_over() {
                        for p in &region.players {
                            if !p.passed_priority {
                                valid_actions.insert(Action::PassPriority(p.player_id));
                            }
                        }
                    }
                }
            }
        }


        valid_actions
    }
}