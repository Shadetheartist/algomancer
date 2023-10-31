use std::{env, fs, process};

use game::state;

use crate::game::action::Action;
use crate::game::{Game, GameOptions};
use crate::game::state::{GameMode, TeamConfiguration};
use crate::game::state::resource::Faction::{Earth, Wood};

mod game;
mod wrap_index;

fn new_game() {
    let options = GameOptions{
        seed: state::rng::AlgomancerRngSeed::default(),
        game_mode: GameMode::LiveDraft{
            selected_deck_types: vec![Earth, Wood],
            team_configuration:TeamConfiguration::Teams { teams_of_players: vec![3, 3] } ,
        },
    };

    let game = Game::new(&options).expect("game object");

    let json = serde_json::to_string(&game).expect("serialized game json");
    print!("{}", json);
}

fn get_actions(state_json: &str) {
    let state: Game = match serde_json::from_str(state_json) {
        Ok(val) => val,
        Err(err) => {
            println!("Error parsing state JSON: {}", err);
            process::exit(1);
        }
    };

    let valid_actions = state.valid_actions();
    let json = serde_json::to_string(&valid_actions).expect("serialized game json");
    print!("{}", json);
}

fn apply_action(state_json: &str, action_json: &str) {
    // Parse the JSON strings to serde_json::Value
    let mut game: Game = match serde_json::from_str(state_json) {
        Ok(val) => val,
        Err(err) => {
            println!("Error parsing state JSON: {}", err);
            process::exit(1);
        }
    };

    let action: Action = match serde_json::from_str(action_json) {
        Ok(val) => val,
        Err(err) => {
            println!("Error parsing action JSON: {}", err);
            process::exit(1);
        }
    };

    match game.apply_action(action) {
        Ok(_) => {
            let json = serde_json::to_string(&game).expect("serialized game json");
            print!("{}", json);
        }
        Err(err) => {
            println!("Error applying action: {:?}", err);
            process::exit(1);
        }
    }
}

fn run_it(){
    let options = GameOptions{
        seed: state::rng::AlgomancerRngSeed::default(),
        game_mode: GameMode::LiveDraft{
            selected_deck_types: vec![Earth, Wood],
            team_configuration:TeamConfiguration::Teams { teams_of_players: vec![3, 3] } ,
        },
    };

    let mut game = game::Game::new(&options).expect("game object");
    let mut counter = 0;
    while counter < 400 {
        let actions: Vec<Action> = game.valid_actions().iter().cloned().collect();

        if actions.len() < 1 {
            eprintln!("out of actions");
            break;
        }

        let mut sorted_actions = actions.clone();
        sorted_actions.sort();
        sorted_actions.reverse();

        let action = sorted_actions.into_iter().next().expect("any action");
        let result = game.apply_action(action);

        match result {
            Ok(_) => {}
            Err(err) => {
                panic!("{:?}", err)
            }
        }

        counter += 1;
    }

    let json = serde_json::to_string_pretty(&game).expect("serialized game json");
    fs::write("game_data.json", json).expect("written game data");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: <get_actions | apply_action <state> <action>>");
        process::exit(1);
    }

    match args[1].as_str() {
        "new_game" => {
            if args.len() != 2 {
                println!("Usage: new_game");
                process::exit(1);
            }

            new_game()
        },

        "get_actions" => {
            if args.len() != 3 {
                println!("Usage: get_actions <state>");
                process::exit(1);
            }

            get_actions(&args[2])
        },
        "apply_action" => {
            if args.len() != 4 {
                println!("Usage: apply_action <state> <action>");
                process::exit(1);
            }
            apply_action(&args[2], &args[3]);
        }
        _ => {
            println!("Unknown command");
            process::exit(1);
        }
    }
}

