use std::collections::HashSet;
use std::hash::Hash;

use rand::prelude::SliceRandom;
use rand::RngCore;

use crate::game::action::Action;
use crate::game::Game;
use crate::game::state::card::{Card, CardId};
use crate::game::state::card::CardType::Resource;
use crate::game::state::card_collection::CardCollection;
use crate::game::state::error::StateError;
use crate::game::state::mutation::{StateMutation, StaticStateMutation};
use crate::game::state::mutation::StaticStateMutation::{CreatePackForPlayer, MoveCard, PhaseTransition};
use crate::game::state::progression::Phase::PrecombatPhase;
use crate::game::state::progression::PrecombatPhaseStep;
use crate::game::state::region::RegionId;
use crate::game::state::State;

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
    pub fn valid_drafts(&self, region_id: RegionId) -> Vec<Action> {
        let player_id = self.state.find_region(region_id).expect("a region").sole_player().player_id;

        let mut actions = Vec::new();

        let player = self.state.find_player(player_id).expect("a player");
        let card_ids: Vec<CardId> = player.hand.iter()
            .filter(|card| {
                let proto = &self.cards_db.prototypes[&card.prototype_id];
                if let Resource(_) = proto.card_type {
                    return false
                }
                true
            })
            .map(|card| card.card_id)
            .collect();


        let num_cards_to_draft = {
            if player.hand.len() >= 10 {
                player.hand.len() - 10
            } else {
                0
            }
        };


        let performance_mode = true;
        let combinations = {
            if performance_mode {
                // this generates a random unique set of size `num_options` of combinations of cards
                let num_options = 3;
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

        actions
    }

    pub fn generate_draft_mutations(&self, action: &Action) -> Result<Vec<StateMutation>, StateError> {
        if let Action::Draft { player_id, cards_to_keep } = action {
            let player_id = *player_id;
            let mut mutations = Vec::new();
            let state = &self.state;
            let player = state.find_player(player_id)?;

            let cards_for_pack: Vec<&Card> = player.hand.iter().filter(|c| !cards_to_keep.contains(&c.card_id)).collect();
            if cards_for_pack.len() != 10 {
                return Err(StateError::InvalidDraft)
            }

            if let None = &player.pack {
                mutations.push(StateMutation::Static(CreatePackForPlayer { player_id: player.player_id }));
            }

            for card in cards_for_pack {
                let card_id = card.card_id;
                let eval_mutation = StateMutation::Eval(Box::new(move |state| -> Result<StaticStateMutation, StateError> {
                    let player = state.find_player(player_id)?;
                    Ok(MoveCard {
                        from_cc_id: player.hand.id(),
                        to_cc_id: player.pack.as_ref().unwrap().id(),
                        card_id: card_id,
                    })
                }));

                mutations.push(eval_mutation);
            }

            let region_id = state.find_region_id_containing_player(player_id);
            mutations.push(StateMutation::Static(PhaseTransition { region_id }));

            // if all the other regions are in the pass pack step, and we just transitioned to it as
            // well, then all players are ready to receive their packs
            let all_other_regions_in_pass_pack_step = state.regions.iter().all(|r| {
                r.region_id != region_id && r.step == PrecombatPhase(PrecombatPhaseStep::PassPack)
            });

            // therefore all regions should move the the next step
            if all_other_regions_in_pass_pack_step {
                for r in &state.regions {
                    mutations.push(StateMutation::Static(PhaseTransition { region_id: r.region_id }));
                }
            }

            eprintln!("Player [{:?}] has selected their draft.", player_id);

            Ok(mutations)

        } else {
            panic!("action should have been draft")
        }
    }

    pub fn apply_draft_action(&self, mut state: State, action: &Action) -> Result<State, StateError> {
        if let Action::Draft { player_id, cards_to_keep } = action {
            let cards_for_pack = {
                let player = &mut state.find_player_mut(*player_id)?;

                let mut cards_for_hand = Vec::new();
                let mut cards_for_pack = Vec::new();

                let card_ids: Vec<CardId> = player.hand.iter().map(|c| c.card_id).collect();
                for c_id in card_ids {
                    let card = player.hand.remove(c_id).expect("a card to have been removed");
                    if cards_to_keep.contains(&card.card_id) {
                        cards_for_hand.push(card);
                    } else {
                        cards_for_pack.push(card);
                    }
                }

                if cards_for_pack.len() != 10 {
                    return Err(StateError::InvalidDraft)
                }

                for card in cards_for_hand {
                    player.hand.add(card);
                }

                cards_for_pack
            };

            let player = state.find_player_mut(*player_id).expect("a player");
            match player.pack.as_mut() {
                None => {
                    player.pack = Some(CardCollection::new_pack(player.player_id))
                }
                Some(player_pack) => {
                    for card in cards_for_pack {
                        player_pack.add(card);
                    }
                }
            }

            let region_id = state.find_region_id_containing_player(*player_id);
            state = state.region_transition_to_next_step(region_id);

            let all_players_passed_packs = state.regions.iter().all(|r| {
                r.step == PrecombatPhase(PrecombatPhaseStep::PassPack)
            });

            if all_players_passed_packs {
                state = state.transition_step_in_all_regions();
            }

            eprintln!("Player [{:?}] has selected their draft.", *player_id);


            Ok(state)

        } else {
            panic!("action should have been draft")
        }
    }
}
