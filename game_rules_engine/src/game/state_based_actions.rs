use crate::game::state::error::StateError;
use crate::game::state::mutation::StateMutation;
use crate::game::state::State;

impl State {

    fn generate_state_based_mutations(&self) -> Result<Vec<StateMutation>, StateError> {
        let mut mutations = Vec::new();

        mutations = add_sba_skip(self, mutations)?;

        Ok(mutations)
    }
}

fn add_sba_skip(state: &State, mut mutations: Vec<StateMutation>) -> Result<Vec<StateMutation>, StateError> {
    for p in state.players() {
        if p.health == 0 {

        }
    }

    Ok(mutations)
}