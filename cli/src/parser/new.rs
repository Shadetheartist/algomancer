use algomancer_gre::game::state::resource::Faction;
use clap::{Args, Subcommand, ValueEnum};

#[derive(Debug, Args)]
#[command(rename_all = "snake_case")]
pub struct NewArgs {
    /// a 128 bit unsigned integer used as seed for the random number generator
    #[arg(short, long, default_value = "0")]
    pub seed: u128,

    /// A game mode to initialize the game with
    #[command(subcommand)]
    pub game_mode: GameModeCommand,
}

#[derive(Debug, Subcommand)]
#[command(rename_all = "snake_case")]
pub enum GameModeCommand {

    /// Create a new game using the Live Draft game mode
    LiveDraft(LiveDraftArgs),

    /// Not Implemented Yet
    PreDraft,

    /// Not Implemented Yet
    TeamDraft ,

    /// Not Implemented Yet
    Constructed,
}

#[derive(Debug, Subcommand)]
pub enum Mode {
    #[command(name = "1v1")]
    OneVsOne,

    #[command(name = "2v2")]
    TwoVsTwo,

    #[command(name = "3v3")]
    ThreeVsThree,

    #[command(name = "ffa")]
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

    /// The factions of the Decks to shuffle into the common deck
    #[arg(short, long, required = true)]
    pub factions: Vec<FactionArg>,

    #[command(subcommand)]
    pub mode: Mode,
}
