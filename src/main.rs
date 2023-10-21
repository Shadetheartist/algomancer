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

    let mut game = game::Game::new(&options);
    let json = serde_json::to_string_pretty(&game).expect("serialized game json");

    loop {
        let actions = game.valid_actions();

        if actions.len() < 1 {
            eprintln!("out of actions");
            break;
        }

        let action = &actions[0];
        game.apply_action(&action);
    }

    println!("{json}");

}

