use std::cmp::Ordering;
use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use crate::game::Game;
use crate::game::state::card::{CardId};
use crate::game::state::hand::Hand;
use crate::game::state::player::PlayerId;
use crate::game::state::progression::{MainPhaseStep, Phase, PrecombatPhaseStep};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub enum Action {
    // resolves the next stack item, if there are no stack items, it passes priority
    // Once both players pass priority consecutively, the game moves to the next step or phase.
    PassPriority(PlayerId),

    // a player selects a hand of cards from a draft pack, leaving 10 cards in the pack
    Draft { player_id: PlayerId, hand: Hand },

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

impl Game {


    pub fn apply_action(&mut self, action: &Action) {
        // todo: teams
        // todo: priority system (need teams)
        println!("Applying Action [{:?}] during phase [{:?}]", action, self.state.step);

        let mut next_state = self.state.clone();

        match action {
            Action::PassPriority(player_id) => {
                let mut player = player_id.get_player(&mut next_state).expect("the player that passed priority");
                player.passed_priority = true;
                if next_state.all_players_passed_priority() {
                    next_state.transition_to_next_step();
                }
            }

            Action::Draft { player_id, .. } => {
                let mut player = player_id.get_player(&mut next_state).unwrap();

                // todo: constructed vs draft (constructed draw and draft are combined)
                // todo: set hand to cards selected in draft

                player.has_drafted = true;
                println!("Player [{:?}] has selected their draft.", player.id);
            }

            Action::Cast(_) => {
                todo!()
            }
        }

        self.state = next_state
    }

    pub fn valid_actions(&self) -> HashSet<Action> {
        let mut valid_actions = HashSet::new();

        match &self.state.step {
            Phase::MainPhase(MainPhaseStep::NITMain) => {
                // dont put a valid action, for testing
            }

            Phase::PrecombatPhase(PrecombatPhaseStep::Draft) => {
                for p in &self.state.players {
                    if !p.has_drafted {
                        valid_actions.insert(Action::Draft {
                            player_id: p.id,
                            hand: Hand {
                                cards: vec![
                                    CardId(1)
                                ]
                            },
                        });
                    }

                    if !self.is_over() {
                        if !p.passed_priority {
                            valid_actions.insert(Action::PassPriority(p.id));
                        }
                    }
                }
            }

            _ => {
                if !self.is_over() {
                    for p in &self.state.players {
                        if !p.passed_priority {
                            valid_actions.insert(Action::PassPriority(p.id));
                        }
                    }
                }
            }
        }

        valid_actions
    }
}