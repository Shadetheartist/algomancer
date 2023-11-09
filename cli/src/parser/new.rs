use std::ffi::c_char;
use clap::{Args, Subcommand, ValueEnum};
use game_rules_engine::game::state::resource::Faction;

#[derive(Debug, Args)]
#[command(rename_all = "snake_case")]
pub struct NewArgs {
    /// a 128 bit unsigned integer used as seed for the random number generator
    #[arg(short, long, default_value = "0")]
    pub seed: u128,

    #[command(subcommand)]
    pub game_mode: GameModeCommand,
}

// cli new --seed 1 live_draft  ffa 6
// cli new --seed 1 live_draft  1v1
// cli new --seed 1 live_draft  2v2
// cli new --seed 1 constructed 2v2

#[derive(Debug, Subcommand)]
#[command(rename_all = "snake_case")]
pub enum GameModeCommand {
    LiveDraft(LiveDraftArgs),
    PreDraft,
    TeamDraft ,
    Constructed,
}

#[derive(Debug, Subcommand)]
pub enum Mode {
    OneVsOne,
    TwoVsTwo,
    ThreeVsThree,
    FFA(FFAArgs),
}

#[derive(Debug, Args)]
pub struct FFAArgs {
    pub num_players: u8,
}

#[derive(Debug, ValueEnum, Clone)]
pub enum FactionArg {
    Fire,
    Water,
    Wood,
    Earth,
    Metal
}

impl FactionArg  {
    pub fn to_faction(&self) -> Faction {
        match self {
            FactionArg::Fire => Faction::Fire,
            FactionArg::Water => Faction::Water,
            FactionArg::Wood => Faction::Wood,
            FactionArg::Earth => Faction::Earth,
            FactionArg::Metal => Faction::Metal,
        }
    }
}

#[derive(Debug, Args)]
pub struct LiveDraftArgs {

    #[arg(short, long, num_args = 1)]
    pub factions: Vec<FactionArg>,

    #[command(subcommand)]
    pub mode: Mode,
}
