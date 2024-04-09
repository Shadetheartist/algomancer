use algomancer_gre::game::{Game, GameOptions};
use algomancer_gre::game::game_builder::NewGameError;
use crate::coordinator::LobbyId;

#[derive(Debug)]
pub enum Error {
    NewGameError(NewGameError)
}

#[derive(Debug)]
pub struct Runner {
    lobby_id: LobbyId,
    game: Game
}

impl Runner {
    pub fn new(lobby_id: LobbyId, game_options: &GameOptions) -> Result<Self, Error> {
        let game = match Game::new(game_options) {
            Ok(game) => game,
            Err(err) => return Err(Error::NewGameError(err))
        };

        let runner = Self {
            lobby_id: lobby_id,
            game: game,
        };

        Ok(runner)
    }
}