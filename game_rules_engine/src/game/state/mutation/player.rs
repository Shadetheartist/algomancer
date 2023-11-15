use crate::game::state::{GameMode, State};
use crate::game::state::card::CardId;
use crate::game::state::card_collection::CardCollection;
use crate::game::state::error::StateError;
use crate::game::state::player::{Player, PlayerId};

impl State {
    pub fn find_player_mut(&mut self, player_id: PlayerId) -> Result<&mut Player, StateError> {
        let find_result = self.players_mut().into_iter().find(|p| p.id == player_id);
        match find_result {
            None => {
                Err(StateError::PlayerNotFound(player_id))
            }
            Some(player) => {
                Ok(player)
            }
        }
    }

    pub fn players_mut(&mut self) -> Vec<&mut Player> {
        self.regions.iter_mut().flat_map(|r| &mut r.players).collect()
    }


    pub fn player_draw_n_cards(&mut self, player_id: PlayerId, n: usize){

        let deck = self.get_deck_for_player(player_id).expect("a deck");
        let mut cards = Vec::new();
        for _ in 0..n {
            let card = deck.draw().expect("a card");
            cards.push(card);
        }

        let player = self.find_player_mut(player_id).expect("a player");
        for card in cards {
            player.hand.add(card);
        }
    }


    pub fn player_recycle_card(&mut self, player_id: PlayerId, card_id: CardId){

        // remove the card from the player's hand
        let card = {
            let player = self.find_player_mut(player_id).expect("a player");
            player.hand.remove(card_id).expect("a card was removed")
        };

        // add the removed card to the bottom of the deck
        let deck = self.get_deck_for_player(player_id).expect("a deck");
        deck.add(card);
    }

    pub fn get_deck_for_player(&mut self, player_id: PlayerId) -> Result<&mut CardCollection, StateError> {
        match &self.game_mode {
            GameMode::LiveDraft { .. } => {
                if let Some(common_deck) = &mut self.common_deck {
                    Ok(common_deck)
                } else {
                    panic!("player is supposed to draw from the common deck in live-draft, but it doesn't exist");
                }
            },
            GameMode::PreDraft { .. } | GameMode::Constructed { .. } => {
                let player = self.find_player_mut(player_id).expect("player");
                if let Some(player_deck) = player.deck.as_mut() {
                    Ok(player_deck)
                } else {
                    panic!("player is supposed to draw from their own deck in pre-draft & constructed, but it doesn't exist");
                }
            },
            GameMode::TeamDraft { .. } => {
                // weird, this needs a common deck per team i guess
                todo!("need to implement team draft, which deck the player is drawing from")
            }
        }
    }
}