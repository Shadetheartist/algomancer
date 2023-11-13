use serde::{Deserialize, Serialize};
use crate::game::state::card::Card;
use crate::game::state::player::PlayerId;
use crate::game::state::State;


#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub struct CardCollectionId(pub [char; 6]);

impl CardCollectionId {
    pub fn from_string(str: &str) -> CardCollectionId {
        let mut char_array: [char; 6] = Default::default();
        for (i, c) in str.chars().enumerate() {
            char_array[i] = c;
        }
        CardCollectionId(char_array)
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct  CardCollection {
    pub id: CardCollectionId,
    pub cards: Vec<Card>,
}

impl CardCollection {

    pub fn new_hand(player_id: PlayerId) -> CardCollection {
        let id_string = format!("p_{:0>2}_h", player_id.0);
        CardCollection {
            id: CardCollectionId::from_string(id_string.as_str()),
            cards: Vec::new(),
        }
    }

    pub fn new_discard(player_id: PlayerId) -> CardCollection {
        let id_string = format!("p_{:0>2}_d", player_id.0);
        CardCollection {
            id: CardCollectionId::from_string(id_string.as_str()),
            cards: Vec::new(),
        }
    }

    pub fn new_pack(player_id: PlayerId) -> CardCollection {
        let id_string = format!("p_{:0>2}_p", player_id.0);
        CardCollection {
            id: CardCollectionId::from_string(id_string.as_str()),
            cards: Vec::new(),
        }
    }

}

impl State {
    pub fn find_card_collection(&self, id: CardCollectionId) -> Option<&CardCollection> {
        if let Some(player) = self.players().iter().find(|p| p.hand.id == id) {
            return Some(&player.hand)
        }

        if let Some(player) = self.players().iter().find(|p| p.discard.id == id) {
            return Some(&player.discard)
        }

        None
    }
}