use serde::{Deserialize, Serialize};
use crate::effect::Effect;
use crate::event::Event;

#[derive(Debug, Hash, Serialize, Deserialize)]
pub enum Ability {
    OneShot(OneShotAbility),
    Triggered(TriggeredAbility),
}

#[derive(Debug, Hash, Serialize, Deserialize)]
pub struct OneShotAbility {
    pub(crate) effect: Effect
}

#[derive(Debug, Hash, Serialize, Deserialize)]
pub struct TriggeredAbility {
    pub(crate) triggers: Vec<Event>,
    pub(crate) ability: OneShotAbility
}