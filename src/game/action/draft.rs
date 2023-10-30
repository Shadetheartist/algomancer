use std::collections::HashSet;
use std::hash::Hash;

use rand::prelude::SliceRandom;
use rand::RngCore;

use crate::game::action::Action;
use crate::game::Game;
use crate::game::state::{GameMode, State};
use crate::game::state::card::CardId;
use crate::game::state::card::CardType::Resource;
use crate::game::state::pack::Pack;
use crate::game::state::player::PlayerId;

fn combinations<T: Clone>(items: &[T], k: usize) -> Vec<Vec<T>> {
    let n = items.len();
    if k > n {
        return Vec::new();
    }

    // Compute the number of combinations using the formula n choose k
    let capacity = (n - k + 1..=n).product::<usize>() / (1..=k).product::<usize>();
    let mut result = Vec::with_capacity(capacity);

    // Initialize the first combination
    let mut indices: Vec<usize> = (0..k).collect();

    loop {
        result.push(indices.iter().map(|&i| items[i].clone()).collect());

        // Generate the next combination in lexicographic order
        let mut i = k as isize - 1;
        while i >= 0 {
            indices[i as usize] += 1;
            if indices[i as usize] < n - (k - 1 - i as usize) {
                for j in i + 1..k as isize {
                    indices[j as usize] = indices[(j - 1) as usize] + 1;
                }
                break;
            }
            i -= 1;
        }

        // All combinations generated
        if i < 0 {
            break;
        }
    }

    result
}

fn random_unique_combinations<T: Clone + Ord + Hash, R: RngCore>(rng: &mut R, input: &[T], k: usize, n: usize) -> Vec<Vec<T>> {
    let mut combinations = Vec::new();
    let mut seen_combinations = HashSet::new();

    for _ in 0..n {
        let mut new_combination;
        loop {
            let mut shuffled: Vec<T> = input.to_vec();
            shuffled.shuffle(rng);
            new_combination = shuffled[0..k].to_vec();
            new_combination.sort();  // To make it easier to check for duplicates
            if !seen_combinations.contains(&new_combination) {
                break;
            }
        }
        seen_combinations.insert(new_combination.clone());
        combinations.push(new_combination);
    }

    combinations
}

impl Game {
    pub fn valid_drafts(&self, player_id: PlayerId) -> Vec<Action> {

        let mut actions = Vec::new();

        let player = self.state.player(player_id).expect("a player");
        let card_ids: Vec<CardId> = player.hand.cards.iter()
            .filter(|card| {
                let proto = &self.cards_db.prototypes[&card.prototype_id];
                proto.card_type != Resource
            })
            .map(|card| card.card_id)
            .collect();


        let num_cards_to_draft = {
            if player.hand.cards.len() >= 10 {
                player.hand.cards.len() - 10
            } else {
                0
            }
        };


        let performance_mode = true;
        let combinations = {
            if performance_mode {
                // this generates a random unique set of size `num_options` of combinations of cards
                let num_options = 128;
                let mut rng_clone = self.state.rand.rng.clone();
                random_unique_combinations(&mut rng_clone, &card_ids, num_cards_to_draft, num_options)
            } else {
                // this generates an exhaustive list of combinations
                combinations(card_ids.as_slice(), num_cards_to_draft)
            }
        };

        for combination in combinations {
            actions.push(Action::Draft {
                player_id: player.player_id,
                cards_to_keep: combination,
            })
        }

        dbg!(actions.len());

        actions
    }

    pub fn apply_draft_action(&mut self, mut state: State, action: &Action) -> State {
        if let Action::Draft { player_id, cards_to_keep } = action {
            let player_hand = state.player_hand_mut(*player_id);

            let mut cards_for_hand = Vec::new();
            let mut cards_for_pack = Vec::new();

            for _ in 0..player_hand.cards.len() {
                let card = player_hand.cards.remove(0);
                if cards_to_keep.contains(&card.card_id) {
                    cards_for_hand.push(card);
                } else {
                    cards_for_pack.push(card);
                }
            }

            if cards_for_pack.len() != 10 {
                panic!("there must always be 10 cards in the pack")
            }

            for card in cards_for_hand {
                player_hand.cards.push(card);
            }

            let player = state.player_mut(*player_id).expect("a player");
            match player.pack.as_mut() {
                None => {
                    player.pack = Some(Pack { cards: cards_for_pack })
                }
                Some(player_pack) => {
                    for card in cards_for_pack {
                        player_pack.cards.push(card);
                    }
                }
            }

            let region_id = state.region_id_containing_player(*player_id);
            state = state.transition_to_next_step(region_id);

            match state.game_mode {
                GameMode::LiveDraft { .. } | GameMode::PreDraft { .. } | GameMode::TeamDraft { .. } => {}
                GameMode::Constructed { .. } => { todo!() }
            }

            println!("Player [{:?}] has selected their draft.", *player_id);

            state

        } else {
            panic!("action should have been draft")
        }
    }
}
