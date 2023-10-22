use std::collections::HashSet;
use crate::game::Game;
use crate::game::state::card::{CardId, Hand};
use crate::game::state::player::PlayerId;
use crate::game::state::progression::{MainPhaseStep, Phase, PrecombatPhaseStep};

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Action {
    // resolves the next stack item, if there are no stack items, it passes priority
    // Once both players pass priority consecutively, the game moves to the next step or phase.
    Resolve,

    // a player selects a hand of cards from a draft pack, leaving 10 cards in the pack
    Draft { player_id: PlayerId, hand: Hand },

    // a card is cast
    Cast(CardId),
}


impl Game {
    pub fn apply_action(&mut self, action: &Action) {
        // todo: teams
        // todo: priority system (need teams)
        println!("Applying Action [{:?}] during phase [{:?}]", action, self.state.step);

        let mut next_state = self.state.clone();

        match action {
            Action::Resolve => {
                next_state.transition_to_next_step();
            }

            Action::Draft { player_id, .. } => {
                let mut player = player_id.get_player(&mut next_state).unwrap();
                player.has_drafted = true;
                println!("Player [{:?}] has finished drafting.", player.id);

                if next_state.players.iter().find(|p| p.has_drafted) == None {
                    next_state.transition_to_next_step();
                }
            }

            Action::Cast(_) => {
                todo!()
            }
        }
        next_state.transition_to_next_step();

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
                    if p.has_drafted {
                        continue;
                    }

                    valid_actions.insert(Action::Draft {
                        player_id: p.id,
                        hand: Hand {
                            cards: vec![
                                CardId(1)
                            ]
                        },
                    });
                }
            }

            _ => {
                if self.is_over() {
                    valid_actions.insert(Action::Resolve);
                }
            }
        }

        valid_actions
    }
}