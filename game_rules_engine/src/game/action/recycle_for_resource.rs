use serde::{Deserialize, Serialize};
use crate::game::action::{Action, ActionTrait, ActionType};
use database::CardPrototypeDatabase;
use crate::game::state::card::{Card, CardId, FindCardResult};
use crate::game::state::error::{InvalidActionError, StateError};
use crate::game::state::mutation::create_card::CreateCardMutation;
use crate::game::state::mutation::move_card::{MoveCardMutation, Placement, To};
use crate::game::state::mutation::StateMutation;
use crate::game::state::mutation::StaticStateMutation::{MoveCard};
use crate::game::state::player::Player;
use crate::game::state::progression::{Phase, PlanningPhaseStep, Team};
use algocore::ResourceType;
use crate::game::state::State;
use crate::{sm_eval, sm_static};

#[derive(Hash, Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct RecycleForResourceAction {
    pub card_id: CardId,
    pub resource_type: ResourceType,
}

impl ActionTrait for RecycleForResourceAction {
    fn generate_mutations(&self, state: &State, db: &CardPrototypeDatabase, _issuer: &Player) -> Result<Vec<StateMutation>, StateError> {
        let mut mutations = Vec::new();
        let find_card_result = state.find_card(self.card_id)?;
        match find_card_result {
            FindCardResult::InPlayerHand(p, cc, _) => {
                let player_deck_id = p.deck(state).id;
                mutations.push(StateMutation::Static(MoveCard(MoveCardMutation {
                    from: cc.id,
                    to: To::Ordered(player_deck_id, Placement::OnBottom),
                    card_id: self.card_id,
                })));

                let player_hand_id = p.hand.id;
                let prototype_id = db.resource(self.resource_type).prototype_id;
                mutations.push(sm_eval!(move |next_state| {
                    let mutation = sm_static!(CreateCard, CreateCardMutation {
                        card_collection_id: player_hand_id,
                        card: Card {
                            card_id: CardId(next_state.next_card_id),
                            prototype_id
                        }
                    });
                    Ok(Some(mutation))
                }));
            }
            FindCardResult::InPlayerPack(_, _, _) |
            FindCardResult::InPlayerDiscard(_, _, _) |
            FindCardResult::InPlayerDeck(_, _, _) |
            FindCardResult::InCommonDeck(_, _) => {
                return Err(InvalidActionError::InvalidRecycle.into());
            }
        }

        Ok(mutations)
    }

    fn get_valid(state: &State, db: &CardPrototypeDatabase) -> Vec<Action> {
        let mut actions = Vec::new();

        // during the mana phase, players can recycle any of their cards to gain a resource

        for region in &state.regions {
            let player = region.sole_player();
            if player.team_id == state.initiative_team() {
                if let Phase::PlanningPhase(PlanningPhaseStep::Mana(Team::IT)) = region.step {} else {
                    continue;
                }
            } else if let Phase::PlanningPhase(PlanningPhaseStep::Mana(Team::NIT)) = region.step {} else {
                continue;
            }


            for card in player.hand.iter() {
                let proto = db.prototypes.get(&card.prototype_id).expect("a card prototype");
                if !proto.card_type.is_real() {
                    continue;
                }

                for resource_type in ResourceType::all() {
                    actions.push(Action {
                        issuer_player_id: player.id,
                        action: ActionType::RecycleForResource(RecycleForResourceAction {
                            card_id: card.card_id,
                            resource_type,
                        }),
                    })
                }
            }
        }


        actions
    }
}

