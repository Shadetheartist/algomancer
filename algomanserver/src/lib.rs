//! The 'Algomanserver' crate provides the base functionality for matchmaking and game running.
//!
//! # Game Coordination / Matchmaking
//! The `coordinator` (aka matchmaker) module allows players to register as 'agents' and create or
//! join a server. When the agents in the lobby are ready, the lobby is migrated to a `runner`.
//!
//! # Running a Game
//! The `runner` acts as a game server, providing clients with synchronized game state and
//! processing their actions.

pub mod coordinator;
pub mod runner;

pub use coordinator::{
    Coordinator,
    agent::{AgentId, AgentKey, Agent},
    lobby::{LobbyId, LobbyEvent, Lobby},
};

pub use runner::{
    Runner,
    RunnerId,
};