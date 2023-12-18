use serde::{Deserialize, Serialize};
use crate::game::action::{Action, ActionTrait, ActionType};
use crate::game::action::pass_priority::PassPriorityAction;
use database::CardPrototypeDatabase;

use crate::game::state::error::{StateError};
use crate::game::state::formation::{Formation, FormationId, FormationPos};
use crate::game::state::mutation::StateMutation;

use crate::game::state::permanent::{Permanent, PermanentId};
use crate::game::state::player::Player;
use crate::game::state::region::RegionId;

use crate::game::state::State;

#[derive(Hash, Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct AttackAction {
    pub home_region_id: RegionId,
    pub under_attack_region_id: RegionId,
    pub formation: Formation<PermanentId>
}

impl ActionTrait for AttackAction {
    fn generate_mutations(&self, state: &State, _db: &CardPrototypeDatabase, _issuer: &Player) -> Result<Vec<StateMutation>, StateError> {
        let mutations: Vec<StateMutation> = vec![
            state.generate_mutation_for_phase_transition(self.home_region_id),
            state.generate_mutation_for_phase_transition(self.under_attack_region_id)
        ];

        Ok(mutations)
    }

    fn get_valid(state: &State, _db: &CardPrototypeDatabase) -> Vec<Action> {
        let mut actions : Vec<Action> = Vec::new();

        for region in &state.regions {
            if !region.step.is_attack() {
                continue
            }

            let player = region.sole_player();

            if state.player_can_act(player.id) {
                actions.push(Action {
                    issuer_player_id: player.id,
                    action: ActionType::PassPriority(PassPriorityAction {}),
                });
            } else {
                break
            }

            let clockwise_neighbour_id = state.region_clockwise_neighbour(region.id).expect("a neighbour").id;

            let mut formation = Formation::new(FormationId(state.next_formation_id), player.id);
            let some_permanent = region.unformed_permanents.first().expect("some permanent to exist");
            if let Permanent::Resource { common, .. } = some_permanent {
                formation.insert_at(FormationPos::FrontRow(0), common.permanent_id).expect("permanent inserted into formation");
            }
            actions.push(Action {
                issuer_player_id: player.id,
                action: ActionType::Attack(AttackAction {
                    home_region_id: region.id,
                    under_attack_region_id: clockwise_neighbour_id,
                    formation,
                }),
            });
        }

        Vec::new()
        // actions
    }
}

