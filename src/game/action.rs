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
        println!("Applying Action [{:?}] during phase [{:?}]", action, self.state.phase);

        let mut next_state = self.state.clone();

        match action {
            Action::Resolve => {
                next_state.phase = next_state.phase.next()
            }

            Action::Draft { .. } => {
                todo!()
            }

            Action::Cast(_) => {
                todo!()
            }
        }

        self.state = next_state
    }

    pub fn valid_actions(&self) -> HashSet<Action> {
        let mut valid_actions = HashSet::new();

        match &self.state.phase {
            Phase::MainPhase(MainPhaseStep::NITMain) => {
                // dont put a valid action, for testing
            }
            Phase::PrecombatPhase(PrecombatPhaseStep::Draft) => {
                valid_actions.insert(Action::Resolve);
            }
            _ => {
                valid_actions.insert(Action::Resolve);
            }
        }


        valid_actions
    }
}