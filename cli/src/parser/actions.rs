use clap::{Args, Subcommand};

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
    pub state: String,
}

#[derive(Debug, Args)]
#[command(rename_all = "snake_case")]
pub struct ApplyActionArgs {
    pub state: String,
    pub action: String,
}