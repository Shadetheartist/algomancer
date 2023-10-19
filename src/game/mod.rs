pub mod state;

use serde::{Deserialize, Serialize};

type ObjectId = i32;

#[derive(Serialize, Deserialize, Clone)]
enum Effect {
    RandomDamage { target: ObjectId, min: i32, max: i32, prepared_amount: i32 },
    Damage { target: ObjectId, amount: i32 },
    Heal { target: ObjectId, amount: i32 },
    Special(SpecialEffect),
}

trait StateMutator {
    fn name(&self) -> &str;
    fn prepare(&self, state: &mut state::State) -> Self;
    fn explain(&self) -> String;
    fn mutate_state(&self, state: &mut state::State);
}

#[derive(Clone, Serialize, Deserialize)]
struct SpecialEffect {
    effect_number: i32,
}

impl StateMutator for SpecialEffect {
    fn name(&self) -> &str {
        "Special"
    }

    fn prepare(&self, state: &mut state::State) -> Self {
        SpecialEffect {
            effect_number: self.effect_number
        }
    }

    fn explain(&self) -> String {
        format!("Sets the game step to {}", self.effect_number)
    }

    fn mutate_state(&self, state: &mut state::State) {
        state.step = self.effect_number
    }
}

impl StateMutator for Effect {

    fn name(&self) -> &str {
        match self {
            Effect::Special { .. } => "Special",
            Effect::Damage { .. } => "Damage",
            Effect::RandomDamage { .. } => "Random Damage",
            Effect::Heal { .. } => "Heal"
        }
    }

    fn prepare(&self, state: &mut state::State) -> Effect {
        match self {
            Effect::RandomDamage { min, max, target, .. } => {
                let amount = state.rand.gen_range(*min..*max);
                let effect = Effect::Damage { amount, target: *target };
                effect
            },
            _ => self.clone()
        }
    }

    fn explain(&self) -> String {
        match self {
            Effect::Special(effect) => format!("Sets the game step to {}", effect.effect_number),
            Effect::Damage { amount, .. } => format!("Deal {} Damage", amount),
            // design issue - the random value should probably be resolved before this part
            Effect::RandomDamage { prepared_amount, max, min, .. } => format!("Deals Between {} and {} Damage [{}]", min, max, prepared_amount),
            Effect::Heal { amount, .. } => format!("Heal {} Damage", amount),
        }
    }

    fn mutate_state(&self, state: &mut state::State) {
        match self {
            Effect::Special(effect) => effect.mutate_state(state),
            Effect::Heal { amount, .. } => state.step -= amount,
            Effect::RandomDamage { prepared_amount, .. } => {
                state.step -= prepared_amount
            },
            Effect::Damage { amount, .. } => state.step += amount,
        }
    }
}


struct EffectHistoryEntry {
    effect: Box<Effect>
}

pub struct Game {
    effect_history: Vec<EffectHistoryEntry>,
    state: state::State,
}

impl Game {
    pub fn new(seed: [u8; 16], num_players: i32) -> Game {
        let mut game = Game {
            effect_history: Vec::new(),
            state: state::State::new(seed),
        };

        for _ in 0..num_players {
            game.state.players.push(state::Player { health: 20 });
        }

        game
    }

    pub fn print_history(&self) {
        println!();
        println!("Action History ({})", self.effect_history.len());
        for (idx, effect) in self.effect_history.iter().enumerate() {
            println!("\t{idx} Applied mutator \"{}\" ({})", effect.effect.name(), effect.effect.explain());
        }
    }

    fn apply_effect(&mut self, effect: Effect) {

        // clone state and apply the mutation
        let mut state = self.state.clone();

        println!("Preparing Effect \"{}\"", effect.name());

        let effect = effect.prepare(&mut self.state);

        print!("Applying mutator \"{}\" ({}) {}", effect.name(), effect.explain(), self.state.get_hash_string());

        // apply the mutation to the state clone
        effect.mutate_state(&mut state);

        if self.state == state {
            panic!("effect [{}] did not mutate the state", effect.name())
        }

        // store the mutation in history
        self.effect_history.push(EffectHistoryEntry {
            effect: Box::new(effect),
        });

        // set current state to mutated clone of state
        self.state = state;

        // print resulting game hash
        println!(" -> {}", self.state.get_hash_string());
    }
}

#[cfg(test)]
mod tests {
    use super::Game;
    use super::Effect;
    use super::SpecialEffect;

    #[test]
    fn test_action_replay() {
        let seed = [0u8; 16];
        let num_players = 123;
        // apply effects to a game, each mutating its state somehow
        let mut game = Game::new(seed, num_players);
        game.apply_effect(Effect::Special(SpecialEffect { effect_number: 11 }));
        game.apply_effect(Effect::Heal { amount: 3, target: 1 });
        game.apply_effect(Effect::Damage { amount: 5, target: 1 });
        game.apply_effect(Effect::RandomDamage { max: 10, min: 1, target: 1, prepared_amount: 0 });

        // use the action history from game 1 on game 2
        let mut game2 = Game::new(seed, num_players);
        for entry in game.effect_history.iter() {
            let effect = *entry.effect.clone();
            game2.apply_effect(effect)
        }

        // after applying the effects in action replay to another game instance,
        // we should end at the same state
        assert_eq!(game.state.get_hash_string(), game2.state.get_hash_string());

        // apply the same effect to both games
        game.apply_effect(Effect::Damage { amount: 1, target: 1 });
        game2.apply_effect(Effect::Damage { amount: 1, target: 1 });


        // if they
    }
}
