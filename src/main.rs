use game::state;
use crate::game::action::Action;
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

    let mut game = game::Game::new(&options).expect("game object");
    let json = serde_json::to_string_pretty(&game).expect("serialized game json");
    let mut counter = 0;
    while counter < 500 {
        let actions: Vec<Action> = game.valid_actions().iter().cloned().collect();

        if actions.len() < 1 {
            eprintln!("out of actions");
            break;
        }

        let mut sorted_actions = actions.clone();
        sorted_actions.sort();
        sorted_actions.reverse();

        let action = sorted_actions.iter().next().expect("any action");
        game.apply_action(action);

        counter += 1;
    }

    println!("{json}");

}

