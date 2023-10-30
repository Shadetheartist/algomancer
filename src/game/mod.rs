use serde::{Deserialize, Serialize};

use state::rng::AlgomancerRngSeed;

use crate::game::state::{effect, GameMode};
use crate::game::state::card::CardPrototypeDatabase;

pub mod state;
pub mod action;
pub mod game_builder;

pub struct GameOptions {
    pub seed: AlgomancerRngSeed,
    pub game_mode: GameMode,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub state: state::State,
    pub cards_db: CardPrototypeDatabase,
}

impl Game {

    // is_over returns true if there are are any living players on at least two teams
    pub fn is_over(&self) -> bool {
        let filtered = self.state.teams().into_iter().filter(|&t| !self.state.living_players_in_team(t).is_empty());
        let count = filtered.take(2).count();
        let result = count < 2;
        result
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

        // set current state to mutated clone of state
        self.state = state;

        // print resulting game hash
        println!(" -> {}", self.state.get_hash_string());
    }
}

#[cfg(test)]
mod tests {
    use crate::game::state::effect::EffectBuilder;
    use crate::game::state::GameMode;
    use crate::game::state::rng::AlgomancerRngSeed;

    use super::{Game, GameOptions};


    #[test]
    fn test_game_serialization() {
        let game_options = GameOptions {
            seed: AlgomancerRngSeed::default(),
            game_mode: GameMode::new_player_mode(),
        };

        let mut game = Game::new(&game_options).expect("game object");

        // use a random damage builder so that we are mutating the rand state
        // we need to make sure that it's serialized properly, so when we resume a game from
        // one of these states, the rng is seeded and offset the same way
        let rand_effect_builder = EffectBuilder::RandomDamage {min: 1, max: 100000, target: 1};

        // apply the affect to a game
        let effect = rand_effect_builder.build_effect(&mut game.state);
        game.apply_effect(effect);

        // serialize the game
        let json = serde_json::to_string_pretty(&game).expect("serialized game json");

        println!("{}", json);

        // deserialize the game
        let mut deserialized: Game = serde_json::from_str(json.as_str()).expect("deserialized game");

        assert_eq!(game.state, deserialized.state);

        // apply another random effect to both the original game and the deserialized one
        let effect = rand_effect_builder.build_effect(&mut game.state);
        game.apply_effect(effect);

        let effect = rand_effect_builder.build_effect(&mut deserialized.state);
        deserialized.apply_effect(effect);

        // the random number generator should be the same, and so the modified states should be equal
        assert_eq!(game.state, deserialized.state);

    }
}
