use serde::{Deserialize, Serialize};
use crate::database::Database;
use crate::state::player::PlayerId;
use crate::state::{State, StateError};

#[derive(Debug, Clone, Hash, Default, Serialize, Deserialize)]
pub struct Priority(pub Option<PlayerId>);

impl State {
    pub(crate) fn player_pass_priority(
        &mut self,
        _: &Database,
        player_id: &PlayerId,
    ) -> Result<(), StateError> {
        let current_player_seat_idx = self
            .seating
            .iter()
            .find(|p_id| *p_id == player_id)
            .ok_or(StateError::PlayerDoesNotExist(*player_id))?
            .0;

        let next_player_id = {
            let mut next_seat_idx = current_player_seat_idx;
            loop {
                next_seat_idx = (next_seat_idx + 1) % self.seating.len();
                let player_id = self.seating[next_seat_idx];
                let player = self.get_player(&player_id)?;
                if player.alive {
                    break player_id;
                }
            }
        };

        assert_ne!(
            *player_id, next_player_id,
            "the next living player with priority can't be the current player"
        );

        self.priority.0 = Some(next_player_id);

        Ok(())
    }
}