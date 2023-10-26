use crate::game::action::Action;
use crate::game::action::Action::Draft;
use crate::game::Game;
use crate::game::state::{GameMode, State};
use crate::game::state::card::CardId;
use crate::game::state::card::CardType::Resource;
use crate::game::state::pack::Pack;
use crate::game::state::player::PlayerId;


fn combinations<T: Clone>(items: &[T], k: usize) -> Vec<Vec<T>> {
    if k == 0 {
        return vec![vec![]];
    }

    if items.len() == 0 {
        return vec![];
    }

    let mut all_combinations = vec![];

    let (first_item, rest_items) = items.split_at(1);

    // Include the first item
    for mut combination in combinations(rest_items, k - 1) {
        combination.push(first_item[0].clone());
        all_combinations.push(combination);
    }

    // Exclude the first item
    all_combinations.extend(combinations(rest_items, k));

    all_combinations
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

        let combinations = combinations(card_ids.as_slice(), num_cards_to_draft);

        for combination in combinations {
            actions.push(Draft {
                player_id: player.player_id,
                cards_to_keep: combination,
            })
        }

        dbg!(actions.len());

        actions
    }

    pub fn apply_draft_action(&mut self, state: &mut State, action: &Action) {
        if let Action::Draft { player_id, cards_to_keep } = action {
            {
                let mut player_hand = state.player_hand_mut(*player_id);

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
            }

            match state.game_mode {
                GameMode::LiveDraft { .. } | GameMode::PreDraft { .. } | GameMode::TeamDraft { .. } => {}
                GameMode::Constructed { .. } => { todo!() }
            }

            let mut player = state.player_mut(*player_id).expect("a player");
            player.has_drafted = true;

            println!("Player [{:?}] has selected their draft.", *player_id);
        } else {
            panic!("action should have been draft")
        }
    }
}
