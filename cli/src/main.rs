use std::collections::HashSet;

use algomancer_gre::game::{Game, GameOptions};
use algomancer_gre::game::action::Action;
use algomancer_gre::game::game_builder::NewGameError;
use algomancer_gre::game::state::error::StateError;
use algomancer_gre::game::state::GameMode;
use algomancer_gre::game::state::resource::Faction;
use algomancer_gre::game::state::team_configuration::TeamConfiguration;
use clap::Parser;

use crate::parser::{Cli, Commands};
use crate::parser::actions::{ActionsCommand, ApplyActionArgs, ListActionsArgs};
use crate::parser::new::{FactionArg, GameModeCommand, LiveDraftArgs, Mode, NewArgs};

mod parser;

fn main() -> Result<(), CLIError> {
    let args = Cli::parse();

    match args.command {
        Commands::New(args) => {
            print_new_game(&args)?;
            Ok(())
        }
        Commands::Action(args) => {
            match args.command {
                ActionsCommand::List(args) => {
                    list_actions(&args)?;
                    Ok(())
                }
                ActionsCommand::Apply(args) => {
                    apply_action(&args)?;
                    Ok(())
                }
            }
        }
    }
}


#[derive(Debug)]
enum CLIError {
    FailedToSerializeGame(serde_json::Error),
    FailedToDeserializeGame(serde_json::Error),
    FailedToInitializeGame(NewGameError),

    FailedToSerializeActions(serde_json::Error),
    FailedToDeserializeAction(serde_json::Error),

    InvalidAction(Action, StateError),
    NotImplemented,
}

/// creates a new game instance, serializes it, and prints it to stdout
fn print_new_game(args: &NewArgs) -> Result<(), CLIError> {
    let options = game_options_from_new_args(args)?;
    let game = Game::new(&options);

    match game {
        Ok(game) => {
            let game_json = serialize_game(&game)?;
            println!("{}", game_json);
            Ok(())
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

fn serialize_game(game: &Game) -> Result<String, CLIError> {
    let game_serialized: Result<String, serde_json::Error> = serde_json::to_string(game);
    match game_serialized {
        Ok(game_serialized) => Ok(game_serialized),
        Err(err) => Err(CLIError::FailedToSerializeGame(err)),
    }
}

fn deserialize_game(game_serialized: &str) -> Result<Game, CLIError> {
    let game: Result<Game, serde_json::Error> = serde_json::from_str(game_serialized);
    match game {
        Ok(game) => Ok(game),
        Err(err) => Err(CLIError::FailedToDeserializeGame(err)),
    }
}

fn serialize_actions(actions: &HashSet<Action>) -> Result<String, CLIError> {
    let action_serialized: Result<String, serde_json::Error> = serde_json::to_string(actions);
    match action_serialized {
        Ok(action_serialized) => Ok(action_serialized),
        Err(err) => Err(CLIError::FailedToSerializeActions(err)),
    }
}

fn deserialize_action(action_serialized: &str) -> Result<Action, CLIError> {
    let action: Result<Action, serde_json::Error> = serde_json::from_str(action_serialized);
    match action {
        Ok(action) => Ok(action),
        Err(err) => Err(CLIError::FailedToDeserializeAction(err)),
    }
}

fn list_actions(args: &ListActionsArgs) -> Result<(), CLIError> {
    let game = deserialize_game(&args.state)?;
    let actions = game.valid_actions();
    let actions_json = serialize_actions(&actions)?;
    println!("{}", actions_json);
    Ok(())
}

fn apply_action(args: &ApplyActionArgs) -> Result<(), CLIError> {
    let mut game = deserialize_game(&args.state)?;
    let action = deserialize_action(&args.action)?;
    let result = game.apply_action(action.clone());
    match result {
        Ok(_) => {
            let game_serialized = serialize_game(&game)?;
            println!("{}", game_serialized);
            Ok(())
        }
        Err(err) => Err(CLIError::InvalidAction(action, err)),
    }
}

/// get the unique elements of the faction args by converting to hash set and then back to vec
fn unique_factions(factions: &[FactionArg]) -> Vec<Faction> {
    let factions_set: HashSet<Faction> = HashSet::from_iter(factions.iter().map(|f_a| f_a.to_faction()));
    factions_set.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use algomancer_gre::game::{Game, GameOptions};
    use algomancer_gre::game::action::Action;
    use algomancer_gre::game::state::GameMode;
    use algomancer_gre::game::state::resource::Faction::{Fire, Wood};
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
            let actions = game.valid_actions();
            let mut actions_vec: Vec<Action> = actions.into_iter().collect();
            actions_vec.sort();

            if actions_vec.is_empty() {
                eprintln!("no more actions");
                break;
            }
            let action = actions_vec.remove(0);
            let mutations = game.apply_action(action).unwrap();
            eprintln!("{:?}", mutations)
        }
    }
}