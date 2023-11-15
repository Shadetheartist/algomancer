use crate::game::action::Action;
use crate::game::Game;
use crate::game::state::card::{FindCardResult, Timing};
use crate::game::state::card::CardType::{Resource, Unit};
use crate::game::state::error::StateError;
use crate::game::state::mutation::{StateMutation};
use crate::game::state::mutation::StaticStateMutation::{CreateCard, MoveCard};
use crate::game::state::region::RegionId;
use crate::game::state::resource::ResourceType;

impl Game {
    pub fn valid_mana_phase_actions(&self, region_id: RegionId) -> Vec<Action> {
        let mut actions : Vec<Action> = Vec::new();

        // quick check to discard any actions of regions where
        // the player does not currently have initiative
        let region = self.state.find_region(region_id).expect("a region");
        let player = region.sole_player();
        if !self.state.player_can_act(player.id) {
            return actions
        } else {
            // if the player can act, they can pass priority -
            // which moves to the next synchronised step when all players on the team pass priority
            actions.push(Action::PassPriority(player.id));
        }

        // during the mana phase, players can recycle any of their cards to gain a resource
        let mut recycle_actions = valid_recycle_actions(self, region_id);
        actions.append(&mut recycle_actions);

        // they may also play up to two resources per turn
        let mut play_resource_actions = valid_play_resource_actions(self, region_id);
        actions.append(&mut play_resource_actions);

        // they may also play cards with haste
        let mut play_resource_actions = valid_play_haste_actions(self, region_id);
        actions.append(&mut play_resource_actions);

        actions
    }

    pub fn generate_recycle_for_resource_mutations(&self, action: &Action) -> Result<Vec<StateMutation>, StateError> {
        if let Action::RecycleForResource { card_id, resource_type } = action {
            let card_id = *card_id;
            let mut mutations = Vec::new();
            let find_card_result = self.state.find_card(card_id)?;

            match find_card_result {
                FindCardResult::InPlayerHand(p, cc, _) |
                FindCardResult::InPlayerDiscard(p, cc, _) |
                FindCardResult::InPlayerDeck(p, cc, _) => {
                    let player_deck_id = p.deck(&self.state).id();
                    mutations.push(StateMutation::Static(MoveCard {
                        from_cc_id: cc.id(),
                        to_cc_id: player_deck_id,
                        card_id,
                        placement: None,
                    }));

                    let resource_prototype = self.cards_db.resource(*resource_type);
                    mutations.push(StateMutation::Static(CreateCard {
                        cc_id: p.hand.id(),
                        card_prototype_id: resource_prototype.prototype_id,
                    }));
                }
                FindCardResult::InCommonDeck(_, _) |
                FindCardResult::AsPermanentInRegion(_, _) |
                FindCardResult::AsPermanentInFormation(_, _, _) => {
                    return Err(StateError::InvalidRecycle)
                }
            }

            Ok(mutations)
        } else {
            panic!("only call this for Action::RecycleForResource")
        }
    }
}

fn valid_recycle_actions(game: &Game, region_id: RegionId) -> Vec<Action> {
    let mut actions : Vec<Action> = Vec::new();

    // during the mana phase, players can recycle any of their cards to gain a resource

    let region = game.state.find_region(region_id).expect("a region");
    let player = region.sole_player();

    for card in player.hand.iter() {

        let proto = game.cards_db.prototypes.get(&card.prototype_id).expect("a card prototype");
        if !proto.card_type.is_real() {
            continue
        }

        for resource_type in ResourceType::all() {
            actions.push(Action::RecycleForResource {
                card_id: card.card_id, resource_type
            })
        }
    }

    actions
}

fn valid_play_resource_actions(game: &Game, region_id: RegionId) -> Vec<Action> {
    let mut actions : Vec<Action> = Vec::new();

    // during the mana phase, players can play up to two resources per turn

    let region = game.state.find_region(region_id).expect("a region");
    let player = region.sole_player();

    if player.resources_played_this_turn >= 2 {
        return actions
    }

    for card in player.hand.iter() {
        let proto = game.cards_db.prototypes.get(&card.prototype_id).expect("a card prototype");
        if let Resource(_) = proto.card_type {
            actions.push(Action::PlayCard {
                card_id: card.card_id,
            })
        }
    }

    actions
}


fn valid_play_haste_actions(game: &Game, region_id: RegionId) -> Vec<Action> {
    let mut actions : Vec<Action> = Vec::new();

    // during the mana phase, players can play cards with haste

    let region = game.state.find_region(region_id).expect("a region");
    let player = region.sole_player();

    for card in player.hand.iter() {
        let proto = game.cards_db.prototypes.get(&card.prototype_id).expect("a card prototype");
        if let Unit(Timing::Haste) = proto.card_type {
            actions.push(Action::PlayCard {
                card_id: card.card_id,
            })
        }
    }

    actions
}