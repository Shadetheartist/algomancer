#![allow(dead_code)]

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use serde::Serialize;

trait StateMutator {
    fn name(&self) -> &str;
    fn explain(&self) -> String;
    fn mutate_state(&self, _: &mut State);
}

#[derive(Serialize, Clone)]
struct SpecialEffect {
    effect_number: i32
}

impl StateMutator for SpecialEffect {
    fn name(&self) -> &str {
        "Special Effect"
    }

    fn explain(&self) -> String {
        format!("Sets the game step to {}", self.effect_number)
    }

    fn mutate_state(&self, state: &mut State){
        state.step = self.effect_number
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct State {
    step: i32
}

impl State {
    fn new() -> State {
        State {
            step: 0
        }
    }

    fn get_hash_string(&self) -> String {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        format!("#{:x}", hasher.finish())
    }
}


struct Game<'a>  {
    mutation_history: Vec<Box<dyn StateMutator+'a>>,
    state: State
}

impl<'a> Game<'a> {
    fn new() -> Game<'a> {
        Game {
            mutation_history: Vec::new(),
            state: State::new()
        }
    }

    fn print_history(&self) {
        for mhi in &self.mutation_history {
            println!("Applied mutator \"{}\" ({})", mhi.name(), mhi.explain());
        }
    }

    fn apply_mutator(&mut self, mutator: impl StateMutator+Clone+'a) {
        println!("{} Applying mutator \"{}\" ({})", self.state.get_hash_string(), mutator.name(), mutator.explain());

        // clone state and apply the mutation
        let mut state = self.state.clone();

        // apply the mutation to the state clone
        mutator.mutate_state(&mut state);

        // store the mutation in history
        self.mutation_history.push(Box::new(mutator));

        // set current state to mutated clone of state
        self.state = state;
    }
}


fn main() {

    let mut game = Game::new();

    game.apply_mutator(SpecialEffect{ effect_number: 11 });
    game.apply_mutator(SpecialEffect{ effect_number: 9 });
    game.apply_mutator(SpecialEffect{ effect_number: 7 });
    game.apply_mutator(SpecialEffect{ effect_number: 5 });

    game.print_history()
}
