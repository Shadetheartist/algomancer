pub mod rng;
pub mod options;

pub struct Action {}

pub struct Database {}


#[derive(Clone, Hash, PartialEq, Eq)]
pub struct State {
    options: options::Options,
    rng: rng::GreRng
}

pub struct GameRulesEngine {
    state: State,
    database: Database,
    history: Vec<Action>
}

impl GameRulesEngine {

}