use serde::{Deserialize, Serialize};
use crate::game::state::team::Initiative;

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
    AfterITAttackPriorityWindow(Initiative),
    NITBlock,
    AfterNITBlockPriorityWindow(Initiative),
    Damage,
    AfterCombatPriorityWindow(Initiative),
}

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum CombatPhaseBStep {
    NITPrepareFormation,
    NITAttack,
    AfterNITAttackPriorityWindow(Initiative),
    ITBlock,
    AfterITBlockPriorityWindow(Initiative),
    Damage,
    AfterCombatPriorityWindow(Initiative),
}

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum MainPhaseStep {
    Regroup,
    ITMain,
    NITMain,
}

impl Phase {
    pub fn transition_next(&mut self) {
        let mut next = self.next();
        *self = next;
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
                        // on turn 1 and num_players+1, recycle the draft packs
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
                        Phase::CombatPhaseA(CombatPhaseAStep::AfterITAttackPriorityWindow(Initiative::Initiative))
                    }
                    CombatPhaseAStep::AfterITAttackPriorityWindow(Initiative::Initiative) => {
                        Phase::CombatPhaseA(CombatPhaseAStep::AfterITAttackPriorityWindow(Initiative::NonInitiative))
                    }
                    CombatPhaseAStep::AfterITAttackPriorityWindow(Initiative::NonInitiative) => {
                        Phase::CombatPhaseA(CombatPhaseAStep::NITBlock)
                    }
                    CombatPhaseAStep::NITBlock => {
                        Phase::CombatPhaseA(CombatPhaseAStep::AfterNITBlockPriorityWindow(Initiative::Initiative))
                    }
                    CombatPhaseAStep::AfterNITBlockPriorityWindow(Initiative::Initiative) => {
                        Phase::CombatPhaseA(CombatPhaseAStep::AfterNITBlockPriorityWindow(Initiative::NonInitiative))
                    }
                    CombatPhaseAStep::AfterNITBlockPriorityWindow(Initiative::NonInitiative) => {
                        Phase::CombatPhaseA(CombatPhaseAStep::Damage)
                    }
                    CombatPhaseAStep::Damage => {
                        Phase::CombatPhaseA(CombatPhaseAStep::AfterCombatPriorityWindow(Initiative::Initiative))
                    }
                    CombatPhaseAStep::AfterCombatPriorityWindow(Initiative::Initiative) => {
                        Phase::CombatPhaseA(CombatPhaseAStep::AfterCombatPriorityWindow(Initiative::NonInitiative))
                    }
                    CombatPhaseAStep::AfterCombatPriorityWindow(Initiative::NonInitiative) => {
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
                        Phase::CombatPhaseB(CombatPhaseBStep::AfterNITAttackPriorityWindow(Initiative::Initiative))
                    }
                    CombatPhaseBStep::AfterNITAttackPriorityWindow(Initiative::Initiative) => {
                        Phase::CombatPhaseB(CombatPhaseBStep::AfterNITAttackPriorityWindow(Initiative::NonInitiative))
                    }
                    CombatPhaseBStep::AfterNITAttackPriorityWindow(Initiative::NonInitiative) => {
                        Phase::CombatPhaseB(CombatPhaseBStep::ITBlock)
                    }
                    CombatPhaseBStep::ITBlock => {
                        Phase::CombatPhaseB(CombatPhaseBStep::AfterITBlockPriorityWindow(Initiative::Initiative))
                    }
                    CombatPhaseBStep::AfterITBlockPriorityWindow(Initiative::Initiative) => {
                        Phase::CombatPhaseB(CombatPhaseBStep::AfterITBlockPriorityWindow(Initiative::NonInitiative))
                    }
                    CombatPhaseBStep::AfterITBlockPriorityWindow(Initiative::NonInitiative) => {
                        Phase::CombatPhaseB(CombatPhaseBStep::Damage)
                    }
                    CombatPhaseBStep::Damage => {
                        Phase::CombatPhaseB(CombatPhaseBStep::AfterCombatPriorityWindow(Initiative::Initiative))
                    }
                    CombatPhaseBStep::AfterCombatPriorityWindow(Initiative::Initiative) => {
                        Phase::CombatPhaseB(CombatPhaseBStep::AfterCombatPriorityWindow(Initiative::NonInitiative))
                    }
                    CombatPhaseBStep::AfterCombatPriorityWindow(Initiative::NonInitiative) => {
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

        let initial_phase = Phase::PrecombatPhase(PrecombatPhaseStep::Untap);

        // there aren't nearly 100 steps in a round,
        // so if we get to the end of a round before the loop is over, the test is successful
        let mut phase = initial_phase.clone();
        for _ in 0..100 {
            println!("{:?}", phase);
            phase = phase.next();

            // we got from the beginning to the end of the loop, success!
            if phase == Phase::MainPhase(MainPhaseStep::NITMain) {
                return;
            }
        }

        // go one more
        phase = phase.next();

        // we should be back to the initial phase
        assert_eq!(phase, initial_phase);
    }
}