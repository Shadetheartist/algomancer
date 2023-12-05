use std::collections::HashSet;
use std::{fs, io};

use algomancer_gre::game::{Game, GameOptions};
use algomancer_gre::game::action::{Action};
use algomancer_gre::game::game_builder::NewGameError;
use algomancer_gre::game::state::error::StateError;
use algomancer_gre::game::state::{GameMode};
use algomancer_gre::game::state::faction::Faction;
use algomancer_gre::game::state::team_configuration::TeamConfiguration;
use clap::Parser;
use algomancer_gre::game::state::mutation::StaticStateMutation;
use thiserror::Error;
use crate::parser::{Cli, Commands, Include};
use crate::parser::actions::{ActionsCommand, ApplyActionArgs, ListActionsArgs};
use crate::parser::new::{FactionArg, GameModeCommand, LiveDraftArgs, Mode, NewArgs};

mod parser;
mod json_value_parser;

fn main() -> Result<(), CLIError> {
    let args = Cli::parse();

    match args.command {
        Commands::New(args) => {
            match args.output_file {
                None => {
                    print_new_game(&args)?;
                    Ok(())
                }
                Some(_) => {
                    write_new_game_to_file(&args)?;
                    Ok(())
                }
            }
        }
        Commands::Action(args) => {
            match args.command {
                ActionsCommand::List(args) => {
                    list_actions(&args)?;
                    Ok(())
                }
                ActionsCommand::Apply(apply_args) => {
                    apply_action(apply_args, &args.include)?;
                    Ok(())
                }
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum CLIError {
    #[error("Failed to serialize game data: {0}")]
    FailedToSerializeGame(serde_json::Error),

    #[error("Failed to deserialize game data: {0}")]
    FailedToDeserializeGame(serde_json::Error),

    #[error("Failed to initialize new game: {0}")]
    FailedToInitializeGame(NewGameError),

    #[error("Failed to serialize actions: {0}")]
    FailedToSerializeActions(serde_json::Error),

    #[error("Invalid action {0:?} for the current game state: {1}")]
    InvalidAction(Action, StateError),

    #[error("I/O error: {0}")]
    IoError(io::Error),

    #[error("Invalid arguments: {0}")]
    InvalidArgs(&'static str),

    #[error("Feature not implemented")]
    NotImplemented,
}


/// creates a new game instance, serializes it, and prints it to stdout
fn print_new_game(args: &NewArgs) -> Result<(), CLIError> {
    let options = game_options_from_new_args(args)?;
    let game = Game::new(&options);

    match game {
        Ok(game) => {
            let game_json = serialize_game(&game, &args.include)?;
            println!("{}", game_json);
            Ok(())
        }
        Err(err) => {
            Err(CLIError::FailedToInitializeGame(err))
        }
    }
}

fn write_mutations_to_file(mutations: &Vec<StaticStateMutation>, path: &String) -> Result<(), CLIError> {
    let game_json = serialize_mutations(mutations)?;
    match fs::write(path, game_json) {
        Ok(_) => Ok(()),
        Err(err) => Err(CLIError::IoError(err))
    }
}

fn write_game_to_file(game: &Game, path: &String, include: &Include) -> Result<(), CLIError> {
    let game_json = serialize_game(game, include)?;
    match fs::write(path, game_json) {
        Ok(_) => Ok(()),
        Err(err) => Err(CLIError::IoError(err))
    }
}

fn write_new_game_to_file(args: &NewArgs) -> Result<(), CLIError> {
    let options = game_options_from_new_args(args)?;
    let game = Game::new(&options);

    match game {
        Ok(game) => {
            let out_file = args.output_file.as_ref().expect("an output file");
            write_game_to_file(&game, out_file, &args.include)
        }
        Err(err) => {
            Err(CLIError::FailedToInitializeGame(err))
        }
    }
}

/// converts the args from the 'new' command into a GameOptions instance
fn game_options_from_new_args(args: &NewArgs) -> Result<GameOptions, CLIError> {
    let seed_bytes = args.seed.to_be_bytes();

    match &args.game_mode {
        GameModeCommand::LiveDraft(args) => {
            let LiveDraftArgs { factions: faction_args, mode } = args;
            let game_options = GameOptions {
                seed: seed_bytes,
                game_mode: GameMode::LiveDraft {
                    selected_deck_types: unique_factions(faction_args),
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
                        Mode::Ffa(args) => {
                            TeamConfiguration::ffa(args.num_players)
                        }
                    },
                },
            };

            Ok(game_options)
        }
        GameModeCommand::PreDraft => {
            Err(CLIError::NotImplemented)
        }
        GameModeCommand::TeamDraft => {
            Err(CLIError::NotImplemented)
        }
        GameModeCommand::Constructed => {
            Err(CLIError::NotImplemented)
        }
    }
}

fn serialize_game(game: &Game, _output: &Include) -> Result<String, CLIError> {
    let game_serialized = serde_json::to_string(game);
    match game_serialized {
        Ok(game_serialized) => Ok(game_serialized),
        Err(err) => Err(CLIError::FailedToSerializeGame(err)),
    }
}

fn serialize_actions(actions: &HashSet<Action>) -> Result<String, CLIError> {
    let action_serialized: Result<String, serde_json::Error> = serde_json::to_string(actions);
    match action_serialized {
        Ok(action_serialized) => Ok(action_serialized),
        Err(err) => Err(CLIError::FailedToSerializeActions(err)),
    }
}

fn serialize_mutations(mutations: &Vec<StaticStateMutation>) -> Result<String, CLIError> {
    let mutations_serialized = serde_json::to_string(mutations);
    match mutations_serialized {
        Ok(mutations_serialized) => Ok(mutations_serialized),
        Err(err) => Err(CLIError::FailedToSerializeGame(err)),
    }
}

fn list_actions(args: &ListActionsArgs) -> Result<(), CLIError> {
    let write = |actions_json| {
        match &args.output_file {
            None => {
                println!("{}", actions_json);
                Ok(())
            }
            Some(path) => {
                match fs::write(path, actions_json) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(CLIError::IoError(err))
                }
            }
        }
    };

    match &args.state {
        None => {
            let state = read_state_file(&args.state_file)?;
            let actions = state.valid_actions();
            let actions_json = serialize_actions(&actions)?;
            write(actions_json)
        }
        Some(state) => {
            let actions = state.valid_actions();
            let actions_json = serialize_actions(&actions)?;
            write(actions_json)
        }
    }
}

fn read_state_file(path: &Option<String>) -> Result<Game, CLIError> {
    // this is required to be set if state is not
    let state_file_path = match path.as_ref() {
        Some(path) => path,
        None => {
            return Err(CLIError::InvalidArgs("if state is not passed in through stdin, it must \
            be passed in by file."));
        }
    };

    let file_contents = match fs::read_to_string(state_file_path) {
        Ok(contents) => contents,
        Err(err) => {
            return Err(CLIError::IoError(err));
        }
    };

    let game: Result<Game, serde_json::Error> = serde_json::from_str(&file_contents);
    match game {
        Ok(game) => {
            Ok(game)
        }
        Err(err) => {
            Err(CLIError::FailedToDeserializeGame(err))
        }
    }

}


fn read_action_file(path: &Option<String>) -> Result<Action, CLIError> {
    // this is required to be set if state is not
    let action_file_path = match path.as_ref() {
        Some(path) => path,
        None => {
            return Err(CLIError::InvalidArgs("if action is not passed in through stdin, it must \
            be passed in by file."));
        }
    };

    let file_contents = match fs::read_to_string(action_file_path) {
        Ok(contents) => contents,
        Err(err) => {
            return Err(CLIError::IoError(err));
        }
    };

    let action: Action = serde_json::from_str(&file_contents).unwrap();

    Ok(action)
}

fn apply_action(args: ApplyActionArgs, include: &Include) -> Result<(), CLIError> {
    let action = match args.action {
        None => {
            read_action_file(&args.action_file)?
        }
        Some(action) => action
    };

    match args.state {
        None => {
            let mut state = read_state_file(&args.state_file)?;
            let result = state.apply_action(action.clone());
            match result {
                Ok(mutations) => {
                    if let Some(path) = args.mutations_output_file {
                        write_mutations_to_file(&mutations, &path)?
                    };

                    match args.state_output_file {
                        None => {
                            let game_serialized = serialize_game(&state, include)?;
                            println!("{}", game_serialized);
                        }
                        Some(file) => {
                            write_game_to_file(&state, &file, include)?;
                        }
                    }

                    Ok(())
                }
                Err(err) => Err(CLIError::InvalidAction(action, err)),
            }
        }
        Some(mut state) => {
            let result = state.apply_action(action.clone());
            match result {
                Ok(mutations) => {
                    if let Some(path) = args.mutations_output_file {
                        write_mutations_to_file(&mutations, &path)?
                    };

                    let game_serialized = serialize_game(&state, include)?;
                    println!("{}", game_serialized);

                    Ok(())
                }
                Err(err) => Err(CLIError::InvalidAction(action, err)),
            }
        }
    }
}

/// get the unique elements of the faction args by converting to hash set and then back to vec
fn unique_factions(factions: &[FactionArg]) -> Vec<Faction> {
    let factions_set: HashSet<Faction> = HashSet::from_iter(factions.iter().map(|f_a| f_a.to_faction()));
    factions_set.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use algomancer_gre::game::{Game, GameOptions};
    use algomancer_gre::game::action::{Action};
    use algomancer_gre::game::state::GameMode;
    use algomancer_gre::game::state::faction::Faction::{Fire, Wood};
    use algomancer_gre::game::state::team_configuration::TeamConfiguration;

    // utility function to avoid code duplication
    // creates a pre-defined rng instance
    #[test]
    fn run_test_game() {
        let test_seed_int: u128 = 0;
        let test_seed = test_seed_int.to_be_bytes();
        let options = GameOptions {
            seed: test_seed,
            game_mode: GameMode::LiveDraft {
                selected_deck_types: vec![Wood, Fire],
                team_configuration: TeamConfiguration::Teams {
                    teams_of_players: vec![1, 1],
                },
            },
        };

        let mut game = Game::new(&options).unwrap();


        for _ in 0..500 {
            let start = Instant::now();
            let actions = game.valid_actions();
            let _get_valid_duration = start.elapsed();

            let mut actions_vec: Vec<Action> = actions.into_iter().collect();
            actions_vec.sort();

            if actions_vec.is_empty() {
                eprintln!("no more actions");
                break;
            }
            actions_vec.reverse();

            let action = actions_vec.remove(0);
            let start = Instant::now();
            let _mutations = game.apply_action(action).unwrap();

            let _apply_duration = start.elapsed();
            //eprintln!("t_get {:?} | t_apply: {:?} | mutations {:?}\n", get_valid_duration, apply_duration, mutations)
        }
    }
}
