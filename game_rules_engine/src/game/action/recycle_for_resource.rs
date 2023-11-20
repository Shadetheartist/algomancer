use serde::{Deserialize, Serialize};
use crate::game::action::{Action, ActionTrait, ActionType};
use crate::game::db::CardPrototypeDatabase;
use crate::game::state::card::{CardId, FindCardResult};
use crate::game::state::error::{InvalidActionError, StateError};
use crate::game::state::mutation::create_card::CreateCardMutation;
use crate::game::state::mutation::move_card::{MoveCardMutation, Placement, To};
use crate::game::state::mutation::StateMutation;
use crate::game::state::mutation::StaticStateMutation::{CreateCard, MoveCard};
use crate::game::state::player::Player;
use crate::game::state::progression::{Phase, PrecombatPhaseStep};
use crate::game::state::resource::ResourceType;
use crate::game::state::State;

#[derive(Hash, Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]

pub struct RecycleForResourceAction {
    pub card_id: CardId,
    pub resource_type: ResourceType
}

impl ActionTrait for RecycleForResourceAction {
    fn generate_mutations(&self, state: &State, db: &CardPrototypeDatabase, _issuer: &Player) -> Result<Vec<StateMutation>, StateError> {
        let mut mutations = Vec::new();
        let find_card_result = state.find_card(self.card_id)?;

        match find_card_result {
            FindCardResult::InPlayerHand(p, cc, _) => {
                let player_deck_id = p.deck(state).id;
                mutations.push(StateMutation::Static(MoveCard(MoveCardMutation{
                    from: cc.id,
                    to: To::Ordered(player_deck_id, Placement::OnBottom),
                    card_id: self.card_id,
                })));

                let resource_prototype = db.resource(self.resource_type);
                mutations.push(StateMutation::Static(CreateCard(CreateCardMutation{
                    card_collection_id: p.hand.id,
                    card_prototype_id: resource_prototype.prototype_id,
                })));
            }
            FindCardResult::InPlayerPack(_, _, _) |
            FindCardResult::InPlayerDiscard(_, _, _) |
            FindCardResult::InPlayerDeck(_, _, _) |
            FindCardResult::InCommonDeck(_, _) |
            FindCardResult::AsPermanentInRegion(_, _) |
            FindCardResult::AsPermanentInFormation(_, _, _) => {
                return Err(InvalidActionError::InvalidRecycle.into())
            }
        }

        Ok(mutations)
    }

    fn get_valid(state: &State, db: &CardPrototypeDatabase) -> Vec<Action> {
        let mut actions = Vec::new();

        // during the mana phase, players can recycle any of their cards to gain a resource

        for region in &state.regions {
            if let Phase::PrecombatPhase(PrecombatPhaseStep::ITMana | PrecombatPhaseStep::NITMana) = region.step {} else {
                continue
            }

            let player = region.sole_player();

            for card in player.hand.iter() {

                let proto = db.prototypes.get(&card.prototype_id).expect("a card prototype");
                if !proto.card_type.is_real() {
                    continue
                }

                for resource_type in ResourceType::all() {
                    actions.push(Action{
                        issuer_player_id: player.id,
                        action: ActionType::RecycleForResource(RecycleForResourceAction{
                            card_id: card.card_id,
                            resource_type,
                        })
                    })
                }
            }
        }


        actions
    }
}

