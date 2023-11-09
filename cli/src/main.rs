use std::{env, fs, process};
use clap::Parser;

use game_rules_engine::game::state;
use game_rules_engine::game::{Game, GameOptions};
use game_rules_engine::game::action::Action;
use game_rules_engine::game::state::{GameMode, TeamConfiguration};
use game_rules_engine::game::state::resource::Faction::{Earth, Wood};
use crate::parser::{Cli, Commands};
use crate::parser::new::{GameModeCommand, LiveDraftArgs, Mode, NewArgs};

mod parser;

fn new_game() {
    let game = Game::new(&default_options()).expect("game object");

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

fn cmd_parse() {
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
        }

        "get_actions" => {
            if args.len() != 3 {
                println!("Usage: get_actions <state>");
                process::exit(1);
            }

            get_actions(&args[2])
        }
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

fn default_options() -> GameOptions {
    GameOptions {
        seed: state::rng::AlgomancerRngSeed::default(),
        game_mode: GameMode::LiveDraft {
            selected_deck_types: vec![Earth, Wood],
            team_configuration: TeamConfiguration::Teams { teams_of_players: vec![2, 2] },
        },
    }
}

fn run_it() {
    let mut game = Game::new(&default_options()).expect("game object");
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

fn game_options_from_new_args(args: &NewArgs) -> GameOptions {
    let seed_bytes = args.seed.to_be_bytes();

    match &args.game_mode {
        GameModeCommand::LiveDraft(args) => {
            match args {
                LiveDraftArgs { factions: faction_args, mode } => {
                    let factions = faction_args.into_iter().map(|f_a| f_a.to_faction()).collect();
                    GameOptions {
                        seed: seed_bytes,
                        game_mode: GameMode::LiveDraft {
                            selected_deck_types: factions,
                            team_configuration: match mode {
                                Mode::OneVsOne => {
                                    TeamConfiguration::one_v_one()
                                }
                                Mode::TwoVsTwo => {
                                    TeamConfiguration::two_v_two()
                                }
                                Mode::ThreeVsThree => {
                                    TeamConfiguration::three_v_three()
                                }
                                Mode::FFA(args) => {
                                    TeamConfiguration::ffa(args.num_players)
                                }
                            },
                        },
                    }
                }
            }
        }
        GameModeCommand::PreDraft => {
            todo!()
        }
        GameModeCommand::TeamDraft => {
            todo!()
        }
        GameModeCommand::Constructed => {
            todo!()
        }
    }
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::New(args) => {
            let options = game_options_from_new_args(&args);
            eprintln!("{:?}", options);
        }
    }
}

