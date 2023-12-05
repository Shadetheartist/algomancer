
use serde::{Deserialize, Serialize};

use crate::game::state::{GameMode};



#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
#[serde(tag = "phase")]
pub enum Phase {
    PlanningPhase(PlanningPhaseStep),
    BattlePhaseA(BattlePhaseStep),
    BattlePhaseB(BattlePhaseStep),
    DeploymentPhase(DeploymentPhaseStep),
}

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
#[serde(tag = "step")]
pub enum PlanningPhaseStep {
    Refresh,
    Draw,
    Draft,
    PassPack,
    Mana(Team),
    Haste(Team),
}

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
#[serde(tag = "step")]
pub enum BattlePhaseStep {
    Attack(Team),
    AfterAttackPriorityWindow,
    Block(Team),
    AfterBlockPriorityWindow,
    Damage,
    AfterCombatPriorityWindow,
}


#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
#[serde(tag = "step")]
pub enum DeploymentPhaseStep {
    Regroup,
    Deployment(Team),
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
            Phase::PlanningPhase(step) => {
                match step {
                    PlanningPhaseStep::Refresh => {
                        Phase::PlanningPhase(PlanningPhaseStep::Draw)
                    }
                    PlanningPhaseStep::Draw => {
                        match &game_mode {
                            // skip the draft step in constructed
                            GameMode::Constructed { .. } => Phase::PlanningPhase(PlanningPhaseStep::Mana(Team::IT)),
                            _ => Phase::PlanningPhase(PlanningPhaseStep::Draft),
                        }
                    }
                    PlanningPhaseStep::Draft => Phase::PlanningPhase(PlanningPhaseStep::PassPack),
                    PlanningPhaseStep::PassPack =>  Phase::PlanningPhase(PlanningPhaseStep::Mana(Team::IT)),
                    PlanningPhaseStep::Mana(Team::IT) => Phase::PlanningPhase(PlanningPhaseStep::Mana(Team::NIT)),
                    PlanningPhaseStep::Mana(Team::NIT) => Phase::PlanningPhase(PlanningPhaseStep::Haste(Team::IT)),
                    PlanningPhaseStep::Haste(Team::IT) => Phase::PlanningPhase(PlanningPhaseStep::Haste(Team::NIT)),
                    PlanningPhaseStep::Haste(Team::NIT) => Phase::BattlePhaseA(BattlePhaseStep::Attack(Team::IT)),
                }
            }
            Phase::BattlePhaseA(step) => {
                match step {
                    BattlePhaseStep::Attack(Team::IT) => Phase::BattlePhaseA(BattlePhaseStep::AfterAttackPriorityWindow),
                    BattlePhaseStep::AfterAttackPriorityWindow => Phase::BattlePhaseA(BattlePhaseStep::Block(Team::NIT)),
                    BattlePhaseStep::Block(Team::NIT) => Phase::BattlePhaseA(BattlePhaseStep::AfterBlockPriorityWindow),
                    BattlePhaseStep::AfterBlockPriorityWindow => Phase::BattlePhaseA(BattlePhaseStep::Damage),
                    BattlePhaseStep::Damage => Phase::BattlePhaseA(BattlePhaseStep::AfterCombatPriorityWindow),
                    BattlePhaseStep::AfterCombatPriorityWindow => Phase::BattlePhaseB(BattlePhaseStep::Attack(Team::NIT)),
                    _ => { panic!("mismatched phase/step") }
                }
            }
            Phase::BattlePhaseB(step) => {
                match step {
                    BattlePhaseStep::Attack(Team::NIT) => Phase::BattlePhaseB(BattlePhaseStep::AfterAttackPriorityWindow),
                    BattlePhaseStep::AfterAttackPriorityWindow => Phase::BattlePhaseB(BattlePhaseStep::Block(Team::IT)),
                    BattlePhaseStep::Block(Team::IT) => Phase::BattlePhaseB(BattlePhaseStep::AfterBlockPriorityWindow),
                    BattlePhaseStep::AfterBlockPriorityWindow => Phase::BattlePhaseB(BattlePhaseStep::Damage),
                    BattlePhaseStep::Damage => Phase::BattlePhaseB(BattlePhaseStep::AfterCombatPriorityWindow),
                    BattlePhaseStep::AfterCombatPriorityWindow => Phase::DeploymentPhase(DeploymentPhaseStep::Regroup),
                    _ => { panic!("mismatched phase/step") }
                }
            }
            Phase::DeploymentPhase(step) => {
                match step {
                    DeploymentPhaseStep::Regroup => Phase::DeploymentPhase(DeploymentPhaseStep::Deployment(Team::IT)),
                    DeploymentPhaseStep::Deployment(Team::IT) => Phase::DeploymentPhase(DeploymentPhaseStep::Deployment(Team::NIT)),
                    DeploymentPhaseStep::Deployment(Team::NIT) => Phase::PlanningPhase(PlanningPhaseStep::Refresh),
                }
            }
        }
    }

    pub fn is_attack(&self) -> bool {
        match self {
            Phase::BattlePhaseA(step) | Phase::BattlePhaseB(step) => {
                matches!(step, BattlePhaseStep::Attack(_))
            }
            _ => false,
        }
    }

    pub fn is_priority_window(&self) -> bool {
        match self {
            Phase::BattlePhaseA(step) | Phase::BattlePhaseB(step) => {
                matches!(step,
                    BattlePhaseStep::AfterAttackPriorityWindow |
                    BattlePhaseStep::AfterBlockPriorityWindow |
                    BattlePhaseStep::AfterCombatPriorityWindow
                )
            }
            _ => false,
        }
    }

    pub fn is_global_sync_step(&self) -> bool {
        match self {
            Phase::PlanningPhase(step) => {
                matches!(step, PlanningPhaseStep::PassPack)
            }
            _ => false,
        }
    }
    pub fn active_team(&self) -> Option<Team> {
        match self {
            Phase::PlanningPhase(PlanningPhaseStep::Mana(team) | PlanningPhaseStep::Haste(team)) |
            Phase::BattlePhaseA(BattlePhaseStep::Attack(team) | BattlePhaseStep::Block(team)) |
            Phase::BattlePhaseB(BattlePhaseStep::Attack(team) | BattlePhaseStep::Block(team)) |
            Phase::DeploymentPhase(DeploymentPhaseStep::Deployment(team)) => Some(*team),
            _ => None
        }
    }

    pub fn is_team_sync_step(&self) -> bool {
        match self {
            Phase::PlanningPhase(step) => {
                matches!(step,
                    PlanningPhaseStep::Mana(_) |
                    PlanningPhaseStep::Haste(_)
                )
            }
            Phase::BattlePhaseA(step) | Phase::BattlePhaseB(step) => {
                matches!(step,
                    BattlePhaseStep::Attack(_) |
                    BattlePhaseStep::Block(_)
                )
            }
            Phase::DeploymentPhase(step) => {
                matches!(step,
                    DeploymentPhaseStep::Deployment(_)
                )
            }
        }
    }
}
