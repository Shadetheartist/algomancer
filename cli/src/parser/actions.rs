use clap::{Args, Subcommand};
use algomancer_gre::game::action::Action;
use algomancer_gre::game::Game;

#[derive(Debug, Args)]
#[command(rename_all = "snake_case")]
pub struct ActionsArgs {
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
    #[arg(value_parser = crate::json_value_parser::json_value_parser::<Game>)]
    pub state: Game,
}

#[derive(Debug, Args)]
#[command(rename_all = "snake_case")]
pub struct ApplyActionArgs {
    #[arg(value_parser = crate::json_value_parser::json_value_parser::<Game>)]
    pub state: Game,

    #[arg(value_parser = crate::json_value_parser::json_value_parser::<Action>)]
    pub action: Action,
}