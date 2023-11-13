use serde::{Deserialize, Serialize};

use crate::game::state::{GameMode, State};
use crate::game::state::region::RegionId;

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub enum Phase {
    PrecombatPhase(PrecombatPhaseStep),
    CombatPhaseA(CombatPhaseAStep),
    CombatPhaseB(CombatPhaseBStep),
    MainPhase(MainPhaseStep),
}

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub enum PrecombatPhaseStep {
    Untap,
    Draw,
    Draft,
    PassPack,
    ITMana,
    NITMana,
}

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub enum CombatPhaseAStep {
    ITAttack,
    AfterITAttackPriorityWindow,
    NITBlock,
    AfterNITBlockPriorityWindow,
    Damage,
    AfterCombatPriorityWindow,
}

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub enum CombatPhaseBStep {
    NITAttack,
    AfterNITAttackPriorityWindow,
    ITBlock,
    AfterITBlockPriorityWindow,
    Damage,
    AfterCombatPriorityWindow,
}

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub enum MainPhaseStep {
    Regroup,
    ITMain,
    NITMain,
}


impl Phase {

    #[allow(dead_code)]
    pub fn is_combat(&self) -> bool {
        match self {
            Phase::CombatPhaseA(_) => true,
            Phase::CombatPhaseB(_) => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    pub fn is_priority_window(&self) -> bool {
        match self {
            Phase::CombatPhaseA(step) => {
                match step {
                    CombatPhaseAStep::AfterITAttackPriorityWindow => true,
                    CombatPhaseAStep::AfterNITBlockPriorityWindow => true,
                    CombatPhaseAStep::AfterCombatPriorityWindow => true,
                    _ => false,
                }
            }
            Phase::CombatPhaseB(step) => {
                match step {
                    CombatPhaseBStep::AfterNITAttackPriorityWindow => true,
                    CombatPhaseBStep::AfterITBlockPriorityWindow => true,
                    CombatPhaseBStep::AfterCombatPriorityWindow => true,
                    _ => false,
                }
            }
            _ => false,
        }
    }

    // this returns the next phase & step given the current phase & step
    pub fn get_next_step(&self, game_mode: &GameMode) -> Phase {
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
}


impl State {
    pub fn transition_step_in_all_regions(mut self) -> Self {
        let region_ids : Vec<RegionId> = self.regions.iter().map(|r| r.region_id).collect();

        for region_id in region_ids {
            self = self.region_transition_to_next_step(region_id);
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use crate::game::state::GameMode;
    use crate::game::state::progression::{MainPhaseStep, Phase, PrecombatPhaseStep};

    #[test]
    fn test_phase_next() {
        let initial_phase = Phase::PrecombatPhase(PrecombatPhaseStep::Untap);
        let mode = &GameMode::new_player_mode();
        // there aren't nearly 100 steps in a round,
        // so if we get to the end of a round before the loop is over, the test is successful
        let mut phase = initial_phase.clone();
        for _ in 0..100 {
            eprintln!("{:?}", phase);
            phase = phase.get_next_step(mode);

            // we got from the beginning to the end of the loop, success!
            if phase == Phase::MainPhase(MainPhaseStep::NITMain) {
                return;
            }
        }

        // go one more
        phase = phase.get_next_step(mode);

        // we should be back to the initial phase
        assert_eq!(phase, initial_phase);
    }
}