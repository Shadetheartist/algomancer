use crate::game::GameOptions;
use crate::game::state::{DeckMode, PlayMode};

mod game;

fn main() {
    let options = GameOptions{
        seed: game::state::AlgomancerRngSeed::default(),
        num_players: 0,
        play_mode: PlayMode::Teams,
        deck_mode: DeckMode::CommonDeck,
    };

    let game = game::Game::new(&options);
    game.print_history();
}

