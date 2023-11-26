use std::collections::HashSet;

use algomancer_gre::game::{Game, GameOptions};
use algomancer_gre::game::action::{Action};
use algomancer_gre::game::game_builder::NewGameError;
use algomancer_gre::game::state::error::StateError;
use algomancer_gre::game::state::GameMode;
use algomancer_gre::game::state::faction::Faction;
use algomancer_gre::game::state::team_configuration::TeamConfiguration;
use clap::Parser;

use crate::parser::{Cli, Commands, Output};
use crate::parser::actions::{ActionsCommand, ApplyActionArgs, ListActionsArgs};
use crate::parser::new::{FactionArg, GameModeCommand, LiveDraftArgs, Mode, NewArgs};

mod parser;
mod json_value_parser;

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
                ActionsCommand::Apply(apply_args) => {
                    apply_action(apply_args, &args.output)?;
                    Ok(())
                }
            }
        }
    }
}


#[derive(Debug)]
enum CLIError {
    FailedToSerializeGame(serde_json::Error),
    FailedToInitializeGame(NewGameError),

    FailedToSerializeActions(serde_json::Error),

    InvalidAction(Action, StateError),
    NotImplemented,
}



/// creates a new game instance, serializes it, and prints it to stdout
fn print_new_game(args: &NewArgs) -> Result<(), CLIError> {
    let options = game_options_from_new_args(args)?;
    let game = Game::new(&options);

    match game {
        Ok(game) => {
            let game_json = serialize_game(&game, &args.output)?;
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

fn serialize_game(game: &Game, output: &Output) -> Result<String, CLIError> {

    let game_serialized: Result<String, serde_json::Error>;

    match output {
        Output::Full => {
            game_serialized = serde_json::to_string(game);
        }
        Output::State => {
            game_serialized = serde_json::to_string(&game.state);
        }
        Output::Database => {
            game_serialized = serde_json::to_string(&game.cards_db);
        }
        Output::History => {
            game_serialized = serde_json::to_string(&game.action_history);
        }
    }

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

fn list_actions(args: &ListActionsArgs) -> Result<(), CLIError> {
    let actions = args.state.valid_actions();
    let actions_json = serialize_actions(&actions)?;
    println!("{}", actions_json);
    Ok(())
}

fn apply_action(mut args: ApplyActionArgs, output: &Output) -> Result<(), CLIError> {
    let result = args.state.apply_action(args.action.clone());
    match result {
        Ok(_mutations) => {
            let game_serialized = serialize_game(&args.state, output)?;
            println!("{}", game_serialized);
            Ok(())
        }
        Err(err) => Err(CLIError::InvalidAction(args.action, err)),
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
            let get_valid_duration = start.elapsed();

            let mut actions_vec: Vec<Action> = actions.into_iter().collect();
            actions_vec.sort();

            if actions_vec.is_empty() {
                eprintln!("no more actions");
                break;
            }
            actions_vec.reverse();

            let action = actions_vec.remove(0);
            let start = Instant::now();
            let mutations = game.apply_action(action).unwrap();

            let apply_duration = start.elapsed();
            //eprintln!("t_get {:?} | t_apply: {:?} | mutations {:?}\n", get_valid_duration, apply_duration, mutations)
        }
    }
}