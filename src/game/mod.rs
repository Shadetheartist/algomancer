use crate::game::state::AlgomancerRngSeed;

pub mod state;
mod effect;
mod card;
mod zone;
mod player;
mod progression;
mod resource;

type ObjectId = i32;

struct EffectHistoryEntry {
    effect: Box<effect::Effect>,
}

#[derive(Clone)]
pub enum PlayMode {
    FFA,
    Teams
}

pub struct Game {

    play_mode: PlayMode,

    // effect history is separate from the game state, so that we don't have to
    // consider the effect history in the state hash, this isn't a blockchain, thank god
    effect_history: Vec<EffectHistoryEntry>,
    state: state::State,
}

pub struct GameOptions {
    pub seed: AlgomancerRngSeed,
    pub num_players: i32,
    pub play_mode: PlayMode,
}

impl Game {
    pub fn new(options: &GameOptions) -> Game {
        let mut game = Game {
            play_mode: options.play_mode.clone(),
            effect_history: Vec::new(),
            state: state::State::new(options.seed),
        };

        for _ in 0..options.num_players {
            game.state.players.push(state::Player { health: 30 });
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

        print!("Applying mutator \"{}\" ({}) {}", effect.name(), effect.explain(), self.state.get_hash_string());

        // apply the mutation to the state clone
        effect.mutate_state(&mut state);

        if self.state == state {
            // this probably shouldn't actually panic,
            // since there are ways an effect is potentially nullified
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
    use super::{Game, GameOptions, PlayMode};
    use super::effect::special::SpecialEffect;
    use super::effect::Effect;
    use super::state::AlgomancerRngSeed;

    #[test]
    fn test_action_replay() {
        let game_options = GameOptions{
            seed: AlgomancerRngSeed::default(),
            num_players: 8,
            play_mode: PlayMode::Teams,
        };

        // apply effect to a game, each mutating its state somehow
        let mut game = Game::new(&game_options);
        game.apply_effect(Effect::Special(SpecialEffect { effect_number: 11 }));
        game.apply_effect(Effect::Heal { amount: 3, target: 1 });
        game.apply_effect(Effect::Damage { amount: 5, target: 1 });

        // use the action history from game 1 on game 2
        let mut game2 = Game::new(&game_options);
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

    #[test]
    fn test_serialization() {
        let game_options = GameOptions{
            seed: AlgomancerRngSeed::default(),
            num_players: 8,
            play_mode: PlayMode::Teams,
        };

        // apply effect to a game, each mutating its state somehow
        let mut game = Game::new(&game_options);
        game.apply_effect(Effect::Special(SpecialEffect { effect_number: 11 }));
        game.apply_effect(Effect::Heal { amount: 3, target: 1 });
        game.apply_effect(Effect::Damage { amount: 5, target: 1 });

        // use the action history from game 1 on game 2
        let mut game2 = Game::new(&game_options);
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
