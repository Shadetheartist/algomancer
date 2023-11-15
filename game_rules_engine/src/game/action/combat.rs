use crate::game::action::Action;
use crate::game::Game;
use crate::game::state::error::StateError;
use crate::game::state::formation::{Formation, FormationId, FormationPos};
use crate::game::state::mutation::StateMutation;
use crate::game::state::permanent::Permanent;
use crate::game::state::region::RegionId;

impl Game {

    pub fn valid_attack_actions(&self, region_id: RegionId) -> Vec<Action> {
        let mut actions : Vec<Action> = Vec::new();

        let region = self.state.find_region(region_id).expect("a region");
        let player = region.sole_player();

        if self.state.player_can_act(player.id) {
            actions.push(Action::PassPriority(player.id));
        } else {
            return actions
        }

        let clockwise_neighbour_id = self.state.region_clockwise_neighbour(region_id).expect("a neighbour").region_id;

        let mut formation = Formation::new(FormationId(self.state.next_formation_id), player.id);
        let some_permanent = region.unformed_permanents.first().expect("some permanent to exist");
        if let Permanent::Resource { common, .. } = some_permanent {
            formation.insert_at(FormationPos::FrontRow(0), common.permanent_id).expect("permanent inserted into formation");
        }

        actions.push(Action::Attack {
            home_region_id: region_id,
            under_attack_region_id: clockwise_neighbour_id,
            formation,
        });

        actions
    }

    pub fn generate_attack_mutations(&self, action: &Action) -> Result<Vec<StateMutation>, StateError> {
        if let Action::Attack { home_region_id, under_attack_region_id, .. } = action {
            let mut mutations = Vec::new();

            mutations.append(&mut self.gen_next_phase(*home_region_id));
            mutations.append(&mut self.gen_next_phase(*under_attack_region_id));

            Ok(mutations)
        } else {
            panic!("only call this for Action::Attack")
        }
    }
}