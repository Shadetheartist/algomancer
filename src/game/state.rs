use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Player {
    pub health: i32
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct State {
    pub step: i32,
    pub players: Vec<Player>
}

impl State {
    pub fn new() -> State {
        State {
            step: 0,
            players: Vec::new()
        }
    }

    pub fn get_hash_string(&self) -> String {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        format!("#{:x}", hasher.finish())
    }
}