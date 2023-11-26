
use serde::{Deserialize, Serialize};

use crate::game::state::{GameMode};

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub enum Phase {
    PrecombatPhase(PrecombatPhaseStep),
    CombatPhaseA(CombatPhaseAStep),
    CombatPhaseB(CombatPhaseBStep),
    MainPhase(MainPhaseStep),
}

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub enum PrecombatPhaseStep {
    Untap,
    Draw,
    Draft,
    PassPack,
    ITMana,
    NITMana,
}

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub enum CombatPhaseAStep {
    ITAttack,
    AfterITAttackPriorityWindow,
    NITBlock,
    AfterNITBlockPriorityWindow,
    Damage,
    AfterCombatPriorityWindow,
}

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub enum CombatPhaseBStep {
    NITAttack,
    AfterNITAttackPriorityWindow,
    ITBlock,
    AfterITBlockPriorityWindow,
    Damage,
    AfterCombatPriorityWindow,
}

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub enum MainPhaseStep {
    Regroup,
    ITMain,
    NITMain,
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
                            GameMode::Constructed { .. } => Phase::PrecombatPhase(PrecombatPhaseStep::ITMana),
                            _ => Phase::PrecombatPhase(PrecombatPhaseStep::Draft),
                        }
                    }
                    PrecombatPhaseStep::Draft => {
                        Phase::PrecombatPhase(PrecombatPhaseStep::PassPack)
                    }
                    PrecombatPhaseStep::PassPack => {
                        Phase::PrecombatPhase(PrecombatPhaseStep::ITMana)
                    }
                    PrecombatPhaseStep::ITMana => {
                        Phase::PrecombatPhase(PrecombatPhaseStep::NITMana)
                    }
                    PrecombatPhaseStep::NITMana => {
                        Phase::CombatPhaseA(CombatPhaseAStep::ITAttack)
                    }
                }
            }
            Phase::CombatPhaseA(step) => {
                match step {
                    CombatPhaseAStep::ITAttack => {
                        Phase::CombatPhaseA(CombatPhaseAStep::AfterITAttackPriorityWindow)
                    }
                    CombatPhaseAStep::AfterITAttackPriorityWindow => {
                        Phase::CombatPhaseA(CombatPhaseAStep::NITBlock)
                    }
                    CombatPhaseAStep::NITBlock => {
                        Phase::CombatPhaseA(CombatPhaseAStep::AfterNITBlockPriorityWindow)
                    }
                    CombatPhaseAStep::AfterNITBlockPriorityWindow => {
                        Phase::CombatPhaseA(CombatPhaseAStep::Damage)
                    }
                    CombatPhaseAStep::Damage => {
                        Phase::CombatPhaseA(CombatPhaseAStep::AfterCombatPriorityWindow)
                    }
                    CombatPhaseAStep::AfterCombatPriorityWindow => {
                        Phase::CombatPhaseB(CombatPhaseBStep::NITAttack)
                    }
                }
            }
            Phase::CombatPhaseB(step) => {
                match step {
                    CombatPhaseBStep::NITAttack => {
                        Phase::CombatPhaseB(CombatPhaseBStep::AfterNITAttackPriorityWindow)
                    }
                    CombatPhaseBStep::AfterNITAttackPriorityWindow => {
                        Phase::CombatPhaseB(CombatPhaseBStep::ITBlock)
                    }
                    CombatPhaseBStep::ITBlock => {
                        Phase::CombatPhaseB(CombatPhaseBStep::AfterITBlockPriorityWindow)
                    }
                    CombatPhaseBStep::AfterITBlockPriorityWindow => {
                        Phase::CombatPhaseB(CombatPhaseBStep::Damage)
                    }
                    CombatPhaseBStep::Damage => {
                        Phase::CombatPhaseB(CombatPhaseBStep::AfterCombatPriorityWindow)
                    }
                    CombatPhaseBStep::AfterCombatPriorityWindow => {
                        Phase::MainPhase(MainPhaseStep::Regroup)
                    }
                }
            }
            Phase::MainPhase(step) => {
                match step {
                    MainPhaseStep::Regroup => {
                        Phase::MainPhase(MainPhaseStep::ITMain)
                    }
                    MainPhaseStep::ITMain => {
                        Phase::MainPhase(MainPhaseStep::NITMain)
                    }
                    MainPhaseStep::NITMain => {
                        Phase::PrecombatPhase(PrecombatPhaseStep::Untap)
                    }
                }
            }
        }
    }

    pub fn is_attack(&self) -> bool {
        match self {
            Phase::CombatPhaseA(step) => {
                matches!(step,
                    CombatPhaseAStep::ITAttack
                )
            }
            Phase::CombatPhaseB(step) => {
                matches!(step,
                    CombatPhaseBStep::NITAttack
                )
            }
            _ => false,
        }
    }

    pub fn is_priority_window(&self) -> bool {
        match self {
            Phase::CombatPhaseA(step) => {
                matches!(step,
                    CombatPhaseAStep::AfterITAttackPriorityWindow |
                    CombatPhaseAStep::AfterNITBlockPriorityWindow |
                    CombatPhaseAStep::AfterCombatPriorityWindow
                )
            }
            Phase::CombatPhaseB(step) => {
                matches!(step,
                    CombatPhaseBStep::AfterNITAttackPriorityWindow |
                    CombatPhaseBStep::AfterITBlockPriorityWindow |
                    CombatPhaseBStep::AfterCombatPriorityWindow
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

    pub fn is_team_sync_step(&self) -> bool {
        match self {
            Phase::PrecombatPhase(step) => {
                matches!(step,
                    PrecombatPhaseStep::ITMana |
                    PrecombatPhaseStep::NITMana
                )
            }
            Phase::CombatPhaseA(step) => {
                matches!(step,
                    CombatPhaseAStep::ITAttack |
                    CombatPhaseAStep::NITBlock
                )
            }
            Phase::CombatPhaseB(step) => {
                matches!(step,
                    CombatPhaseBStep::NITAttack |
                    CombatPhaseBStep::ITBlock
                )
            }
            Phase::MainPhase(step) => {
                matches!(step,
                    MainPhaseStep::ITMain |
                    MainPhaseStep::NITMain
                )
            }
        }
    }
}
