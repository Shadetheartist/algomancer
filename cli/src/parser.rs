use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::parser::actions::ActionsArgs;
use crate::parser::new::NewArgs;

pub mod new;
pub mod actions;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "algomancer")]
#[command(about = "The Algomancy Game Rules Engine", long_about = None, version)]

pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Initialize a new game
    #[command(arg_required_else_help = true)]
    New(NewArgs),

    #[command(arg_required_else_help = true)]
    Action(ActionsArgs),
}
