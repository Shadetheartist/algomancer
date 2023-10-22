
// priority is this system which enables a team to take actions.
// a team can only take actions if they have priority.
// the initiative team takes priority first, whenever priority is reset.
// players on a team share priority, teams pass priority collectively.

use crate::game::state::State;
use crate::game::state::team::Team;

struct Priority {

}

impl State {

    // resets priority state for all teams
    fn reset_priority(&mut self) {
        self.teams.iter_mut().for_each(|t| {
            t.passed_priority = true;
            t.has_priority = false;
        })
    }

    // gets the team with initiative right now
    fn get_initiative_team(&mut self) -> &mut Team {
        self.teams.iter_mut().find(|t| t.has_initiative).unwrap()
    }

    fn begin_window(&mut self) {
        self.reset_priority();

        let initiative_team = self.get_initiative_team();
        initiative_team.has_priority = true;
    }
}
