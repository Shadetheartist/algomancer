use crate::game::action::Action;
use crate::game::Game;
use crate::game::state::card::{Card, CardType, FindCardResult};
use crate::game::state::player::StateError;
use crate::game::state::region::RegionId;
use crate::game::state::resource::ResourceType;
use crate::game::state::State;


fn valid_recycle_actions(game: &Game, region_id: RegionId) -> Vec<Action> {
    let mut actions : Vec<Action> = Vec::new();

    // during the mana phase, players can recycle any of their cards to gain a resource

    let region = game.state.find_region(region_id).expect("a region");
    let player = region.sole_player();

    for card in player.hand.cards.iter() {

        let proto = game.cards_db.prototypes.get(&card.prototype_id).expect("a card prototype");
        if proto.card_type.is_real() == false {
            continue
        }

        for resource_type in ResourceType::all() {
            actions.push(Action::RecycleForResource {
                card_id: card.card_id, resource_type:
                resource_type
            })
        }
    }

    actions
}


impl Game {
    pub fn valid_mana_phase_actions(&self, region_id: RegionId) -> Vec<Action> {
        let mut actions : Vec<Action> = Vec::new();


        // quick check to discard any actions of regions where
        // the player does not currently have initiative
        let region = self.state.find_region(region_id).expect("a region");
        let player = region.sole_player();
        if self.state.player_can_act(player.player_id) == false {
            return actions
        }

        let mut recycle_actions = valid_recycle_actions(self, region_id);
        actions.append(&mut recycle_actions);

        // during the mana phase, players can recycle any of their cards to gain a resource
        // they may also play up to two resources per turn
        // they may also play cards with haste

        // if the player can act, they can pass priority -
        // which moves to the next synchronised step when all players on the team pass priority

        if !player.passed_priority {
            actions.push(Action::PassPriority(player.player_id));
        }

        actions
    }

    pub fn apply_recycle_for_resource_action(&mut self, mut state: State, action: &Action) -> Result<State, StateError> {

        if let Action::RecycleForResource { card_id, resource_type } = action {
            let card_id = *card_id;

            let player_id = {
                let find_card_result = self.state.find_card(card_id).expect("a card");
                match find_card_result {
                    FindCardResult::InPlayerHand(player, card) => {
                        player.player_id
                    }
                    _ => return Err(StateError::InvalidRecycle),
                }
            };

            state.player_recycle_card(player_id, card_id);

            let resource_card = Card::from_resource_type(&self.cards_db, &mut state, *resource_type);

            state.find_player_mut(player_id).expect("a player").hand.cards.push(resource_card);

            Ok(state)
        } else {
            panic!("only call this when the action is of the correct enum type")
        }
    }
}