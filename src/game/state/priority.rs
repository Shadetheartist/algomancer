// priority is this system which enables a team to take actions.
// a team can only take actions if they have priority.
// the initiative team takes priority first, whenever priority is reset.
// players on a team share priority, teams pass priority collectively.

use crate::game::state::State;
use crate::game::state::team::Team;

struct Priority {}

impl State {
    pub fn all_players_passed_priority(&self) -> bool {
        if self.players.len() == 0 {
            panic!("wtf there's no players")
        }
        !self.players.iter().any(|p| p.passed_priority == false)
    }

    // resets priority state for all teams
    pub fn reset_priority(&mut self) {
        self.teams.iter_mut().for_each(|t| {
            t.passed_priority = true;
            t.has_priority = false;
        })
    }

    // gets the team with initiative right now
    pub fn get_initiative_team(&mut self) -> &mut Team {
        self.teams.iter_mut().find(|t| t.has_initiative).unwrap()
    }

    pub fn begin_window(&mut self) {
        self.reset_priority();

        let initiative_team = self.get_initiative_team();
        initiative_team.has_priority = true;
    }
}



#[cfg(test)]
mod tests {
    use crate::game::state::pack::Pack;
    use crate::game::state::player::{Player, PlayerId};
    use crate::game::state::State;
    use crate::game::state::team::TeamId;

    #[test]
    fn test_all_players_passed_priority(){

        // test with no players with passed priority
        // this player creation process is mighty scuffed
        let mut state = State::default();
        for _ in 0..state.players.len() {
            let mut player = Player::new(PlayerId(0), 0, TeamId(0), Pack{ owner: PlayerId(0), cards: vec![] });
            player.passed_priority = false;
            state.players.push(player);
        }
        assert_eq!(state.all_players_passed_priority(), false);


        // test all but one player with passed priority
        for player in state.players.iter_mut() {
            player.passed_priority = true;
        }
        state.players[0].passed_priority = false;
        assert_eq!(state.all_players_passed_priority(), false);


        // test all players with passed priority
        for player in state.players.iter_mut() {
            player.passed_priority = true;
        }
        assert_eq!(state.all_players_passed_priority(), true);

    }
}