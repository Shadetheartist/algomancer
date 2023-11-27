use std::collections::HashSet;
use std::hash::Hash;

use rand::prelude::SliceRandom;
use rand::RngCore;
use serde::{Deserialize, Serialize};

use crate::game::action::{Action, ActionTrait, ActionType};
use crate::game::db::{CardPrototype, CardPrototypeDatabase};

use crate::game::state::card::{Card, CardId};
use crate::game::state::card::CardType::Resource;
use crate::game::state::error::{InvalidActionError, StateError};
use crate::game::state::error::DraftError::{CardNotInHand, IncorrectNumberOfCardsDrafted, InvalidPackCard};
use crate::game::state::error::InvalidActionError::InvalidDraft;

use crate::game::state::mutation::{StateMutation};
use crate::game::state::mutation::create_pack::CreatePackMutation;
use crate::game::state::mutation::move_card::{MoveCardMutation, To};
use crate::game::state::mutation::phase_transition::PhaseTransitionMutation;
use crate::game::state::mutation::StaticStateMutation::{CreatePackForPlayer, MoveCard, PhaseTransition};
use crate::game::state::player::Player;
use crate::game::state::progression::Phase::PrecombatPhase;
use crate::game::state::progression::{Phase, PrecombatPhaseStep};

use crate::game::state::State;


#[derive(Hash, Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct DraftAction {
    pub cards_to_keep: Vec<CardId>,
}


impl DraftAction {

    fn validate(&self, _state: &State, db: &CardPrototypeDatabase, issuer: &Player) -> Result<(), StateError>{
        if issuer.hand.len() - self.cards_to_keep.len() != 10 {
            // enforce that there must be 10 cards remaining to create the next pack
            return Err(InvalidDraft(IncorrectNumberOfCardsDrafted).into());
        }

        // enforce that each card selected actually exists in the player's hand
        for card_id in &self.cards_to_keep {
            if !issuer.hand.iter().any(|c| c.card_id == *card_id) {
                return Err(InvalidDraft(CardNotInHand(*card_id)).into());
            }
        }

        let cards_for_pack: Vec<&Card> = issuer.hand.iter().filter(|c| !self.cards_to_keep.contains(&c.card_id)).collect();
        let _cards_for_pack2: Vec<&CardPrototype> = cards_for_pack.iter().map(|c| &db.prototypes[&c.prototype_id]).collect();

        // enforce that each card left for the pack is not a resource
        for card in cards_for_pack {
            let proto = &db.prototypes[&card.prototype_id];
            if let Resource(_) = proto.card_type {
                return Err(InvalidDraft(InvalidPackCard(card.card_id, "cannot put a resource into a pack")).into());
            }

        }
        Ok(())
    }
}

impl ActionTrait for DraftAction {
    fn generate_mutations(&self, state: &State, db: &CardPrototypeDatabase, issuer: &Player) -> Result<Vec<StateMutation>, StateError> {
        self.validate(state, db, issuer)?;

        let player_id = issuer.id;

        let mut mutations = Vec::new();
        let player = state.find_player(player_id)?;

        let cards_for_pack: Vec<&Card> = player.hand.iter().filter(|c| !self.cards_to_keep.contains(&c.card_id)).collect();
        if cards_for_pack.len() != 10 {
            return Err(InvalidActionError::InvalidDraft(IncorrectNumberOfCardsDrafted).into());
        }

        if player.pack.is_none() {
            mutations.push(StateMutation::Static(CreatePackForPlayer(CreatePackMutation{
                player_id: player.id
            })));
        }

        for card in cards_for_pack {
            let card_id = card.card_id;
            // all these mutations depend on the pack existing, it doesn't yet, but it will when applying these mutations.
            // so we need to look at that future state to get the pack's id, by using the Eval variant.
            let eval_mutation = StateMutation::Eval(Box::new(move |state| -> Result<Option<StateMutation>, StateError> {
                let player = state.find_player(player_id)?;
                Ok(Some(StateMutation::Static(MoveCard(MoveCardMutation{
                    from: player.hand.id,
                    to: To::Unordered(player.pack.as_ref().unwrap().id),
                    card_id,
                }))))
            }));

            mutations.push(eval_mutation);
        }

        let region = state.find_region_containing_player(player_id)?;
        mutations.push(StateMutation::Static(
            PhaseTransition(PhaseTransitionMutation{region_id: region.id, to_phase: region.step.get_next_phase(&state.game_mode) })
        ));

        // if all the other regions are in the pass pack step, and we just transitioned to it as
        // well, then all players are ready to receive their packs
        let all_other_regions_in_pass_pack_step = state.regions.iter().filter(|r| r.id !=  region.id).all(|r| {
            r.step == PrecombatPhase(PrecombatPhaseStep::PassPack)
        });

        // therefore all regions should move the the next step
        if all_other_regions_in_pass_pack_step {
            for r in &state.regions {
                mutations.push(state.generate_mutation_for_phase_transition(r.id));
            }
        }

        eprintln!("Player [{:?}] has selected their draft.", player_id);

        Ok(mutations)
    }

    fn get_valid(state: &State, db: &CardPrototypeDatabase) -> Vec<Action> {
        let mut actions = Vec::new();
        let pack_size = 10;

        let must_keep = |card: &&Card| {
            let proto = &db.prototypes[&card.prototype_id];
            matches!(proto.card_type, Resource(_))
        };

        for region in &state.regions {
            if let Phase::PrecombatPhase(PrecombatPhaseStep::Draft) = region.step {
                let player = region.sole_player();

                let must_keep_card_ids: Vec<CardId> = player.hand
                    .iter()
                    .filter(must_keep)
                    .map(|c| c.card_id)
                    .collect();

                let valid_card_ids: Vec<CardId> = player.hand
                    .iter()
                    .filter(|card| { !must_keep(card) })
                    .map(|card| card.card_id)
                    .collect();

                let num_cards_destined_for_hand = player.hand.len() - pack_size;
                let num_cards_which_cant_be_swapped = must_keep_card_ids.len();
                let num_draftable_options = num_cards_destined_for_hand - num_cards_which_cant_be_swapped;

                let performance_mode = true;
                let combinations = {
                    if performance_mode {
                        // this generates a random unique set of size `num_options` of combinations of cards
                        let num_options = 3;
                        let mut rng_clone = state.rand.clone();
                        random_unique_combinations(&mut rng_clone, &valid_card_ids, num_draftable_options, num_options)
                    } else {
                        // this generates an exhaustive list of combinations
                        combinations(valid_card_ids.as_slice(), num_draftable_options)
                    }
                };

                for combination in combinations {
                    let mut combined = must_keep_card_ids.clone();
                    combined.extend(combination);

                    actions.push(Action {
                        issuer_player_id: player.id,
                        action: ActionType::Draft(DraftAction {
                            cards_to_keep: combined,
                        }),
                    })
                }
            }
        }

        actions
    }
}


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
