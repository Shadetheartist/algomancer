use std::fs;
use game::state;

use crate::game::action::Action;
use crate::game::GameOptions;
use crate::game::state::{GameMode, TeamConfiguration};
use crate::game::state::resource::Faction::{Earth, Wood};

mod game;
mod wrap_index;

fn main() {
    let options = GameOptions{
        seed: state::rng::AlgomancerRngSeed::default(),
        game_mode: GameMode::LiveDraft{
            selected_deck_types: vec![Earth, Wood],
            team_configuration:TeamConfiguration::Teams { teams_of_players: vec![3, 3] } ,
        },
    };

    let mut game = game::Game::new(&options).expect("game object");
    let mut counter = 0;
    while counter < 20 {
        let actions: Vec<Action> = game.valid_actions().iter().cloned().collect();

        if actions.len() < 1 {
            eprintln!("out of actions");
            break;
        }

        let mut sorted_actions = actions.clone();
        sorted_actions.sort();
        sorted_actions.reverse();

        let action = sorted_actions.into_iter().next().expect("any action");
        game.apply_action(action);

        counter += 1;
    }

    let json = serde_json::to_string_pretty(&game).expect("serialized game json");
    fs::write("game_data.json", json).expect("written game data");

}

