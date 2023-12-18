use serde::{Deserialize, Serialize};
use crate::game::action::{Action, ActionTrait, ActionType};
use database::CardPrototypeDatabase;

use crate::game::state::error::StateError;
use crate::game::state::mutation::{StateMutation};
use crate::game::state::player::{Player};
use crate::game::state::stack::Next;
use crate::game::state::State;
use crate::{sm_static};
use crate::game::state::mutation::stack_pass_priority::StackPassPriorityMutation;

#[derive(Hash, Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct PassPriorityAction {}

impl ActionTrait for PassPriorityAction {
    fn generate_mutations(&self, state: &State, _db: &CardPrototypeDatabase, issuer: &Player) -> Result<Vec<StateMutation>, StateError> {
        let mut mutations = Vec::new();

        let player = issuer;
        let region = state.find_region_containing_player(player.id)?;

        // consider what the next stack item is
        match region.stack.next() {
            Next::PassPriority(_) => {
                mutations.push(sm_static!(StackPassPriority, StackPassPriorityMutation{
                    region_id: region.id
                }));
            }
            _ => {
                panic!("invalid state - player shouldn't be able to pass priority");
            }
        }

        Ok(mutations)
    }

    fn get_valid(state: &State, _db: &CardPrototypeDatabase) -> Vec<Action> {
        let mut actions = Vec::new();

        for region in &state.regions {
            for player in &region.players {
                if state.player_can_act(player.id) {
                    actions.push(Action {
                        issuer_player_id: player.id,
                        action: ActionType::PassPriority(PassPriorityAction {}),
                    })
                }
            }
        }

        actions
    }
}


