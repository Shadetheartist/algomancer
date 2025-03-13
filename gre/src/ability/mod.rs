use serde::{Deserialize, Serialize};
use crate::effect::Effect;
use crate::event::Event;

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum Ability {
    OneShot(OneShotAbility),
    Triggered(TriggeredAbility),
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct OneShotAbility {
    pub(crate) effect: Effect
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct TriggeredAbility {
    pub(crate) triggers: Vec<Event>,
    pub(crate) ability: OneShotAbility
}