use serde::{Deserialize, Serialize};

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum Phase {
    PrecombatPhase(PrecombatPhaseStep),
    CombatPhaseA(CombatPhaseAStep),
    CombatPhaseB(CombatPhaseBStep),
    MainPhase(MainPhaseStep),
}

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum PrecombatPhaseStep {
    Untap,
    Draw,
    Draft,
    ITMana,
    NITMana,
}

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum CombatPhaseAStep {
    ITPrepareFormation,
    ITAttack,
    AfterITAttackPriorityWindow,
    NITBlock,
    AfterNITBlockPriorityWindow,
    Damage,
    AfterCombatPriorityWindow,
}

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum CombatPhaseBStep {
    NITPrepareFormation,
    NITAttack,
    AfterNITAttackPriorityWindow,
    ITBlock,
    AfterITBlockPriorityWindow,
    Damage,
    AfterCombatPriorityWindow,
}

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum MainPhaseStep {
    Regroup,
    ITMain,
    NITMain,
}


impl Phase {
    pub fn is_combat(&self) -> bool {
        match self {
            Phase::CombatPhaseA(_) => true,
            Phase::CombatPhaseB(_) => true,
            _ => false,
        }
    }

    pub fn is_priority_window(&self) -> bool {
        match self {
            Phase::CombatPhaseA(step) => {
                match step {
                    CombatPhaseAStep::AfterITAttackPriorityWindow => true,
                    CombatPhaseAStep::AfterNITBlockPriorityWindow => true,
                    CombatPhaseAStep::AfterCombatPriorityWindow => true,
                    _ => false,
                }
            },
            Phase::CombatPhaseB(step) => {
                match step {
                    CombatPhaseBStep::AfterNITAttackPriorityWindow => true,
                    CombatPhaseBStep::AfterITBlockPriorityWindow => true,
                    CombatPhaseBStep::AfterCombatPriorityWindow => true,
                    _ => false,
                }
            },
            _ => false,
        }
    }

    // this returns the next phase & step given the current phase & step
    pub fn next(&self) -> Phase {
        match self {
            Phase::PrecombatPhase(step) => {
                match step {
                    PrecombatPhaseStep::Untap => {
                        Phase::PrecombatPhase(PrecombatPhaseStep::Draw)
                    }
                    PrecombatPhaseStep::Draw => {
                        Phase::PrecombatPhase(PrecombatPhaseStep::Draft)
                    }
                    PrecombatPhaseStep::Draft => {
                        Phase::PrecombatPhase(PrecombatPhaseStep::ITMana)
                    }
                    PrecombatPhaseStep::ITMana => {
                        Phase::PrecombatPhase(PrecombatPhaseStep::NITMana)
                    }
                    PrecombatPhaseStep::NITMana => {
                        Phase::CombatPhaseA(CombatPhaseAStep::ITPrepareFormation)
                    }
                }
            }
            Phase::CombatPhaseA(step) => {
                match step {
                    CombatPhaseAStep::ITPrepareFormation => {
                        Phase::CombatPhaseA(CombatPhaseAStep::ITAttack)
                    }
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
                        Phase::CombatPhaseB(CombatPhaseBStep::NITPrepareFormation)
                    }
                }
            }
            Phase::CombatPhaseB(step) => {
                match step {
                    CombatPhaseBStep::NITPrepareFormation => {
                        Phase::CombatPhaseB(CombatPhaseBStep::NITAttack)
                    }
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
}

#[cfg(test)]
mod tests {
    use crate::game::state::progression::{MainPhaseStep, Phase, PrecombatPhaseStep};

    #[test]
    fn test_phase_next(){

        // there aren't nearly 100 steps in a round,
        // so if we get to the end of a round before the loop is over, the test is successful
        let mut phase = Phase::PrecombatPhase(PrecombatPhaseStep::Untap);
        for _ in 0..100 {
            println!("{:?}", phase);
            phase = phase.next();

            // we got from the beginning to the end of the loop, success!
            if phase == Phase::MainPhase(MainPhaseStep::NITMain) {
                return;
            }
        }

        assert!(false);
    }
}