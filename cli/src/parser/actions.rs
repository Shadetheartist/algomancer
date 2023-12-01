use clap::{Args, Subcommand};
use algomancer_gre::game::action::{Action};
use algomancer_gre::game::Game;
use crate::parser::Include;

#[derive(Debug, Args)]
#[command(rename_all = "snake_case")]
pub struct ActionsArgs {
    #[arg(short, long, default_value="all")]
    pub output: Include,

    #[command(subcommand)]
    pub command: ActionsCommand,
}

#[derive(Debug, Subcommand)]
#[command(rename_all = "snake_case")]
pub enum ActionsCommand {

    /// List the valid actions for a given state
    #[command(name = "ls")]
    List(ListActionsArgs),

    /// Apply an action to a given state and receive the resulting state
    #[command()]
    Apply(ApplyActionArgs),
}

#[derive(Debug, Args)]
#[command(rename_all = "snake_case")]
pub struct ListActionsArgs {

    #[arg(short='f', long, required_unless_present = "state")]
    pub state_file: Option<String>,

    #[arg(required_unless_present = "state_file", value_parser = crate::json_value_parser::json_value_parser::<Game>)]
    pub state: Option<Game>,
}

#[derive(Debug, Args)]
#[command(rename_all = "snake_case")]
pub struct ApplyActionArgs {
    #[arg(short, long)]
    pub mutations: bool,

    #[arg(short='f', long, required_unless_present = "state")]
    pub state_file: Option<String>,

    #[arg(required_unless_present = "state_file", value_parser = crate::json_value_parser::json_value_parser::<Game>)]
    pub state: Option<Game>,

    #[arg(value_parser = crate::json_value_parser::json_value_parser::<Action>)]
    pub action: Action,
}