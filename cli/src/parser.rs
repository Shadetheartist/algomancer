use clap::{Parser, Subcommand};

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
#[allow(clippy::large_enum_variant)]
pub enum Commands {
    /// Initialize a new game
    #[command(arg_required_else_help = true)]
    New(NewArgs),

    /// List actions, Apply an Action
    #[command(arg_required_else_help = true)]
    Action(ActionsArgs),
}
