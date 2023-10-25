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
            let mut player = Player::new(PlayerId(0), 0, TeamId(0), Pack{ owner_player_id: PlayerId(0), cards: vec![] });
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