use serde::{Deserialize, Serialize};
use state::rng::AlgomancerRngSeed;
use crate::game::state::player::{Player, PlayerId};
use crate::game::state::{DeckMode, effect, PlayMode};
use crate::game::state::team::{Team, TeamId};

pub mod state;
pub mod action;


#[derive(Serialize, Deserialize, Debug)]
struct EffectHistoryEntry {
    effect: Box<effect::Effect>,
}

pub struct GameOptions {
    pub seed: AlgomancerRngSeed,
    pub num_players: usize,
    pub play_mode: PlayMode,
    pub deck_mode: DeckMode,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    // effect history is separate from the game state, so that we don't have to
    // consider the effect history in the state hash, this isn't a blockchain, thank god
    effect_history: Vec<EffectHistoryEntry>,
    state: state::State,
}

impl Game {
    pub fn new(options: &GameOptions) -> Result<Game, &str> {
        let mut game = Game {
            effect_history: Vec::new(),
            state: state::State::new(options.seed, &options.play_mode, &options.deck_mode),
        };

        // game basically supports 1 or two teams, where 1 is ffa, and 2 is team play
        let num_teams = 2;
        let players_per_team = options.num_players / num_teams;
        if players_per_team * num_teams != options.num_players {
            return Err("odd number of players not allowed since they won't fit nicely in teams");
        }

        for t in 0..num_teams {
            let team_id = t + 1;

            game.state.teams.push(Team{
                id: TeamId(team_id),
                passed_priority: false,
                has_priority: false,
                has_initiative: false,
            });

            for p in 0..players_per_team {
                let player_seat = p + t * num_teams;
                let player_id = PlayerId(player_seat + 1);
                game.state.players.push(Player::new(player_id, player_seat, TeamId(team_id)));
            }
        }

        Ok(game)
    }

    pub fn print_history(&self) {
        println!();
        println!("Action History ({})", self.effect_history.len());
        for (idx, effect) in self.effect_history.iter().enumerate() {
            println!("\t{idx} Applied mutator \"{}\" ({})", effect.effect.name(), effect.effect.explain());
        }
    }

    // is_over returns true if there are are any living players on at least two teams
    pub fn is_over(&self) -> bool {
        let filtered = self.state.teams.iter().filter(|&t| !self.state.living_players_in_team(t.id).is_empty());
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
    use crate::game::state::effect::EffectBuilder;
    use crate::game::state::DeckMode;
    use super::{Game, GameOptions, PlayMode};
    use super::state::effect::special::SpecialEffect;
    use super::state::effect::Effect;
    use crate::game::state::rng::AlgomancerRngSeed;
    use crate::game::state::team::TeamId;

    #[test]
    fn test_is_over() {
        let game_options = GameOptions {
            seed: AlgomancerRngSeed::default(),
            num_players: 4,
            play_mode: PlayMode::Teams,
            deck_mode: DeckMode::CommonDeck,
        };

        // apply effect to a game, each mutating its state somehow
        let mut game = Game::new(&game_options).expect("game object");
        let res = game.is_over();
        assert_eq!(res, false);

        for mut p in game.state.players_in_team_mut(TeamId(1)) {
            p.is_alive = false
        }

        let res = game.is_over();
        assert_eq!(res, true);
    }

    #[test]
    fn test_action_replay() {
        let game_options = GameOptions {
            seed: AlgomancerRngSeed::default(),
            num_players: 8,
            play_mode: PlayMode::Teams,
            deck_mode: DeckMode::CommonDeck,
        };

        // apply effect to a game, each mutating its state somehow
        let mut game = Game::new(&game_options).expect("game object");
        game.apply_effect(Effect::Special(SpecialEffect { effect_number: 11 }));
        game.apply_effect(Effect::Heal { amount: 3, target: 1 });
        game.apply_effect(Effect::Damage { amount: 5, target: 1 });

        // use the action history from game 1 on game 2
        let mut game2 = Game::new(&game_options).expect("game object");
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
    fn test_game_serialization() {
        let game_options = GameOptions {
            seed: AlgomancerRngSeed::default(),
            num_players: 2,
            play_mode: PlayMode::Teams,
            deck_mode: DeckMode::CommonDeck,
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
