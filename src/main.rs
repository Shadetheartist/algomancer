use std::cell::RefCell;
use crate::game::{GameOptions, PlayMode};

mod game;

fn main() {
    let options = GameOptions{
        seed: game::state::AlgomancerRngSeed::default(),
        num_players: 0,
        play_mode: PlayMode::FFA,
    };

    let game = game::Game::new(&options);
    game.print_history();
}

