
use serde::{Deserialize, Serialize};

use crate::game::state::{GameMode};



#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
#[serde(tag = "phase")]
pub enum Phase {
    PrecombatPhase(PrecombatPhaseStep),
    CombatPhaseA(CombatPhaseStep),
    CombatPhaseB(CombatPhaseStep),
    MainPhase(MainPhaseStep),
}

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
#[serde(tag = "step")]
pub enum PrecombatPhaseStep {
    Untap,
    Draw,
    Draft,
    PassPack,
    Mana(Team),
}

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
#[serde(tag = "step")]
pub enum CombatPhaseStep {
    Attack(Team),
    AfterAttackPriorityWindow,
    Block(Team),
    AfterBlockPriorityWindow,
    Damage,
    AfterCombatPriorityWindow,
}


#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
#[serde(tag = "step")]
pub enum MainPhaseStep {
    Regroup,
    Main(Team),
}

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
#[serde(tag = "team")]
pub enum Team {
    IT,
    NIT,
}

impl Phase {

    /// This method returns the next phase for a game running with a given game mode.
    pub fn get_next_phase(&self, game_mode: &GameMode) -> Phase {
        match self {
            Phase::PrecombatPhase(step) => {
                match step {
                    PrecombatPhaseStep::Untap => {
                        Phase::PrecombatPhase(PrecombatPhaseStep::Draw)
                    }
                    PrecombatPhaseStep::Draw => {
                        match &game_mode {
                            // skip the draft step in constructed
                            GameMode::Constructed { .. } => Phase::PrecombatPhase(PrecombatPhaseStep::Mana(Team::IT)),
                            _ => Phase::PrecombatPhase(PrecombatPhaseStep::Draft),
                        }
                    }
                    PrecombatPhaseStep::Draft => Phase::PrecombatPhase(PrecombatPhaseStep::PassPack),
                    PrecombatPhaseStep::PassPack =>  Phase::PrecombatPhase(PrecombatPhaseStep::Mana(Team::IT)),
                    PrecombatPhaseStep::Mana(Team::IT) => Phase::PrecombatPhase(PrecombatPhaseStep::Mana(Team::NIT)),
                    PrecombatPhaseStep::Mana(Team::NIT) => Phase::CombatPhaseA(CombatPhaseStep::Attack(Team::IT)),
                }
            }
            Phase::CombatPhaseA(step) => {
                match step {
                    CombatPhaseStep::Attack(Team::IT) => Phase::CombatPhaseA(CombatPhaseStep::AfterAttackPriorityWindow),
                    CombatPhaseStep::AfterAttackPriorityWindow => Phase::CombatPhaseA(CombatPhaseStep::Block(Team::NIT)),
                    CombatPhaseStep::Block(Team::NIT) => Phase::CombatPhaseA(CombatPhaseStep::AfterBlockPriorityWindow),
                    CombatPhaseStep::AfterBlockPriorityWindow => Phase::CombatPhaseA(CombatPhaseStep::Damage),
                    CombatPhaseStep::Damage => Phase::CombatPhaseA(CombatPhaseStep::AfterCombatPriorityWindow),
                    CombatPhaseStep::AfterCombatPriorityWindow => Phase::CombatPhaseB(CombatPhaseStep::Attack(Team::NIT)),
                    _ => { panic!("mismatched phase/step") }
                }
            }
            Phase::CombatPhaseB(step) => {
                match step {
                    CombatPhaseStep::Attack(Team::NIT) => Phase::CombatPhaseB(CombatPhaseStep::AfterAttackPriorityWindow),
                    CombatPhaseStep::AfterAttackPriorityWindow => Phase::CombatPhaseB(CombatPhaseStep::Block(Team::IT)),
                    CombatPhaseStep::Block(Team::IT) => Phase::CombatPhaseB(CombatPhaseStep::AfterBlockPriorityWindow),
                    CombatPhaseStep::AfterBlockPriorityWindow => Phase::CombatPhaseB(CombatPhaseStep::Damage),
                    CombatPhaseStep::Damage => Phase::CombatPhaseB(CombatPhaseStep::AfterCombatPriorityWindow),
                    CombatPhaseStep::AfterCombatPriorityWindow => Phase::MainPhase(MainPhaseStep::Regroup),
                    _ => { panic!("mismatched phase/step") }
                }
            }
            Phase::MainPhase(step) => {
                match step {
                    MainPhaseStep::Regroup => Phase::MainPhase(MainPhaseStep::Main(Team::IT)),
                    MainPhaseStep::Main(Team::IT) => Phase::MainPhase(MainPhaseStep::Main(Team::NIT)),
                    MainPhaseStep::Main(Team::NIT) => Phase::PrecombatPhase(PrecombatPhaseStep::Untap),
                }
            }
        }
    }

    pub fn is_attack(&self) -> bool {
        match self {
            Phase::CombatPhaseA(step) | Phase::CombatPhaseB(step) => {
                matches!(step, CombatPhaseStep::Attack(_))
            }
            _ => false,
        }
    }

    pub fn is_priority_window(&self) -> bool {
        match self {
            Phase::CombatPhaseA(step) | Phase::CombatPhaseB(step) => {
                matches!(step,
                    CombatPhaseStep::AfterAttackPriorityWindow |
                    CombatPhaseStep::AfterBlockPriorityWindow |
                    CombatPhaseStep::AfterCombatPriorityWindow
                )
            }
            _ => false,
        }
    }

    pub fn is_global_sync_step(&self) -> bool {
        match self {
            Phase::PrecombatPhase(step) => {
                matches!(step, PrecombatPhaseStep::PassPack)
            }
            _ => false,
        }
    }
    pub fn active_team(&self) -> Option<Team> {
        match self {
            Phase::PrecombatPhase(PrecombatPhaseStep::Mana(team)) => Some(*team),
            Phase::CombatPhaseA(CombatPhaseStep::Attack(team) | CombatPhaseStep::Block(team)) |
            Phase::CombatPhaseB(CombatPhaseStep::Attack(team) | CombatPhaseStep::Block(team)) => Some(*team),
            Phase::MainPhase(MainPhaseStep::Main(team)) => Some(*team),
            _ => None
        }
    }

    pub fn is_team_sync_step(&self) -> bool {
        match self {
            Phase::PrecombatPhase(step) => {
                matches!(step,
                    PrecombatPhaseStep::Mana(_)
                )
            }
            Phase::CombatPhaseA(step) | Phase::CombatPhaseB(step) => {
                matches!(step,
                    CombatPhaseStep::Attack(_) |
                    CombatPhaseStep::Block(_)
                )
            }
            Phase::MainPhase(step) => {
                matches!(step,
                    MainPhaseStep::Main(_)
                )
            }
        }
    }
}
