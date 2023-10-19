mod state;
mod effect;

type ObjectId = i32;

trait StateMutator {
    fn name(&self) -> &str;
    // not sure i like this pattern of preparation
    fn prepare(&self, state: &mut state::State) -> Self;
    fn explain(&self) -> String;
    fn mutate_state(&self, state: &mut state::State);
}

struct EffectHistoryEntry {
    effect: Box<effect::Effect>,
}

pub struct Game {
    // effect history is separate from the game state, so that we don't have to
    // consider the effect history in the state hash, this isn't a blockchain, thank god
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

    fn apply_effect(&mut self, effect: effect::Effect) {

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
    use super::effect::special::SpecialEffect;
    use super::effect::Effect;

    #[test]
    fn test_action_replay() {
        let seed = [0u8; 16];
        let num_players = 123;
        // apply effect to a game, each mutating its state somehow
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

        // after applying the effect in action replay to another game instance,
        // we should end at the same state
        assert_eq!(game.state.get_hash_string(), game2.state.get_hash_string());

        // apply the same effect to both games
        game.apply_effect(Effect::Damage { amount: 1, target: 1 });
        game2.apply_effect(Effect::Damage { amount: 1, target: 1 });

        // game state hashes should still be the same
        assert_eq!(game.state.get_hash_string(), game2.state.get_hash_string());
    }
}
