// priority is this system which enables a team to take actions.
// a team can only take actions if they have priority.
// the initiative team takes priority first, whenever priority is reset.
// players on a team share priority, teams pass priority collectively.

use crate::game::state::State;

struct Priority {}

impl State {
    pub fn all_players_passed_priority(&self) -> bool {
        if self.players().len() == 0 {
            panic!("wtf there's no players")
        }
        !self.players().iter().any(|p| p.passed_priority == false)
    }
}
