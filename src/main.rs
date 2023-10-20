use game::state;
use crate::game::GameOptions;
use crate::game::state::{DeckMode, PlayMode};

mod game;
mod wrap_index;

fn main() {
    let options = GameOptions{
        seed: state::rng::AlgomancerRngSeed::default(),
        num_players: 4,
        play_mode: PlayMode::Teams,
        deck_mode: DeckMode::CommonDeck,
    };

    let game = game::Game::new(&options);
    game.print_history();
}

