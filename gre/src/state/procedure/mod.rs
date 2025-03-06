use serde::{Deserialize, Serialize};

#[derive(Eq, Hash, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
#[serde(tag = "phase")]
pub enum Phase {
    PlanningPhase(PlanningPhaseStep),
    BattlePhaseA(BattlePhaseStep),
    BattlePhaseB(BattlePhaseStep),
    DeploymentPhase(DeploymentPhaseStep),
}

impl Default for Phase {
    fn default() -> Self {
        Phase::PlanningPhase(PlanningPhaseStep::Refresh)
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
#[serde(tag = "step")]
pub enum PlanningPhaseStep {
    Refresh,
    Draw,
    Draft,
    Mana,
    Haste,
}

#[derive(Eq, Hash, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
#[serde(tag = "step")]
pub enum BattlePhaseStep {
    Attack(Team),
    Block(Team),
    Combat,
}


#[derive(Eq, Hash, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
#[serde(tag = "step")]
pub enum DeploymentPhaseStep {
    Regroup,
    Deployment(Team),
}

#[derive(Eq, Hash, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
#[serde(tag = "team")]
pub enum Team {
    IT,
    NIT,
}

const TEAM_PROCEDURE: &[Phase] = &[
    Phase::PlanningPhase(PlanningPhaseStep::Refresh),
    Phase::PlanningPhase(PlanningPhaseStep::Draw),
    Phase::PlanningPhase(PlanningPhaseStep::Draft),
    Phase::PlanningPhase(PlanningPhaseStep::Mana),
    Phase::PlanningPhase(PlanningPhaseStep::Haste),

    Phase::BattlePhaseA(BattlePhaseStep::Attack(Team::IT)),
    Phase::BattlePhaseA(BattlePhaseStep::Block(Team::NIT)),
    Phase::BattlePhaseA(BattlePhaseStep::Combat),

    Phase::BattlePhaseB(BattlePhaseStep::Attack(Team::NIT)),
    Phase::BattlePhaseB(BattlePhaseStep::Block(Team::IT)),
    Phase::BattlePhaseB(BattlePhaseStep::Combat),

    Phase::DeploymentPhase(DeploymentPhaseStep::Regroup),
    Phase::DeploymentPhase(DeploymentPhaseStep::Deployment(Team::IT)),
    Phase::DeploymentPhase(DeploymentPhaseStep::Deployment(Team::NIT)),
];
