use algomancer_gre::game::{Game};
use algomancer_gre::game::game_builder::NewGameError;
use crate::{Lobby};

#[derive(Debug)]
pub enum Error {
    NewGameError(NewGameError)
}

#[derive(Debug)]
pub struct Runner {
    game: Game
}

impl Runner {
    pub fn new(lobby: &Lobby) -> Result<Self, Error> {

        // send out connection info to each client
        // wait for clients to connect
        // once all clients are connected, begin the game
        // loop
        //  wait for a valid action from a client
        //  apply action and broadcast change to clients
        //  if game ends then clean up
        // if clients disconnect unexpectedly - wait to reconnect process
        // if a clients disconnect on purpose, end remove the player and potentially end the game
        // game ends - escape runner and clean up

        let game = match Game::new(&lobby.options) {
            Ok(game) => game,
            Err(err) => return Err(Error::NewGameError(err))
        };

        let runner = Self {
            game,
        };

        Ok(runner)
    }

    pub fn run() {
        tokio::spawn(async move {

        });
    }

    fn wait_for_clients() {}

    fn send_client_info() {}

}