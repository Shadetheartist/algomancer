use clap::{Args, Subcommand};
use algomancer_gre::game::action::{Action};
use algomancer_gre::game::Game;
use crate::parser::Include;

#[derive(Debug, Args)]
#[command(rename_all = "snake_case")]
pub struct ActionsArgs {


    #[arg(short, long, default_value="all")]
    pub include: Include,

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

    #[arg(short='o', long="out_file")]
    pub output_file: Option<String>,

    #[arg(short='f', long, required_unless_present = "state")]
    pub state_file: Option<String>,

    #[arg(required_unless_present = "state_file", value_parser = crate::json_value_parser::json_value_parser::<Game>)]
    pub state: Option<Game>,
}

#[derive(Debug, Args)]
#[command(rename_all = "snake_case")]
pub struct ApplyActionArgs {
    #[arg(short='m', long="out_file")]
    pub mutations_output_file: Option<String>,

    #[arg(short='o', long="out_file")]
    pub state_output_file: Option<String>,

    #[arg(short='f', long, required_unless_present = "state")]
    pub state_file: Option<String>,

    #[arg(required_unless_present = "state_file", value_parser = crate::json_value_parser::json_value_parser::<Game>)]
    pub state: Option<Game>,

    #[arg(short='a', long, required_unless_present = "action")]
    pub action_file: Option<String>,

    #[arg(required_unless_present = "action_file", value_parser = crate::json_value_parser::json_value_parser::<Action>)]
    pub action: Option<Action>,
}