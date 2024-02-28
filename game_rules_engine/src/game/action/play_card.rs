
use serde::{Deserialize, Serialize};
use crate::game::action::{Action, ActionTrait, ActionType};
use database::{CardPrototypeDatabase};
use crate::game::{Game};
use crate::game::state::card::{Card, CardId, FindCardResult };
use algocore::CardType;
use algocore::Timing;

use crate::game::state::error::{CardNotPlayableError,StateError};
use crate::game::state::error::CardNotPlayableError::{CannotPlayMoreResources, CardDoesNotExist, CardLacksCorrectTiming, MustBePlayedFromHand, NotInPlayableStep, NotInPlayableZone};
use crate::game::state::mutation::{StateMutation};

use crate::game::state::mutation::create_permanent::CreatePermanentMutation;
use crate::game::state::mutation::player_mutations::UpdatePlayerResourcesPlayedMutation;
use crate::game::state::mutation::remove_card::RemoveCardMutation;
use crate::game::state::permanent::{Permanent, PermanentCommon, PermanentId};

use crate::game::state::player::{Player, PlayerId};
use crate::game::state::progression::{DeploymentPhaseStep, Phase, PlanningPhaseStep};
use crate::game::state::State;
use crate::{sm_eval, sm_static};


#[derive(Hash, Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlayCardAction {
    pub card_id: CardId
}

impl PlayCardAction {

}

fn remove_card_mutation(card_id: CardId) -> StateMutation {
    sm_static!(RemoveCard, RemoveCardMutation{
        card_id
    })
}



impl ActionTrait for PlayCardAction {



    fn generate_mutations(&self, state: &State, db: &CardPrototypeDatabase, issuer: &Player) -> Result<Vec<StateMutation>, StateError> {
        match state.find_card(self.card_id)? {
            FindCardResult::InPlayerHand(player, _, card) => {
                if player.id != issuer.id {
                    return Err(CardNotPlayableError::NotUnderPlayersControl(self.card_id).into())
                }

                let mut mutations = Vec::new();

                let proto = db.prototypes.get(&card.prototype_id).expect("a card prototype");
                match &proto.card_type {
                    CardType::Resource(_) => {
                        mutations.push(remove_card_mutation(self.card_id));

                        // create the permanent
                        let player_id = player.id;
                        let prototype_id = proto.prototype_id;
                        let mutation = sm_eval!(move |next_state| {
                            let region_id = next_state.find_region_id_containing_player(player_id);
                            let permanent = Permanent::Resource {
                                card_prototype_id: prototype_id,
                                common: PermanentCommon {
                                    permanent_id: PermanentId(next_state.permanent_id_factory.peek()),
                                    controller_player_id: player_id
                                }
                            };

                            let mutation = sm_static!(CreatePermanent, CreatePermanentMutation{
                                region_id,
                                permanent
                            });

                            Ok(Some(mutation))
                        });
                        mutations.push(mutation);

                        let mutation = sm_static!(UpdatePlayerResourcesPlayed, UpdatePlayerResourcesPlayedMutation{
                            player_id: issuer.id,
                            new_value: issuer.resources_played_this_turn + 1
                        });
                        mutations.push(mutation);

                        Ok(mutations)
                    }

                    CardType::Unit(_) => {
                        mutations.push(remove_card_mutation(self.card_id));

                        // create the permanent
                        let player_id = player.id;
                        let prototype_id = proto.prototype_id;
                        let mutation = sm_eval!(move |next_state| {
                            let region_id = next_state.find_region_id_containing_player(player_id);
                            let permanent = Permanent::Resource {
                                card_prototype_id: prototype_id,
                                common: PermanentCommon {
                                    permanent_id: PermanentId(next_state.permanent_id_factory.peek()),
                                    controller_player_id: player_id
                                }
                            };

                            let mutation = sm_static!(CreatePermanent, CreatePermanentMutation{
                                region_id,
                                permanent
                            });

                            Ok(Some(mutation))
                        });

                        mutations.push(mutation);
                        Ok(mutations)
                    }
                    CardType::Spell(_) => { todo!("not yet supported"); }
                    CardType::UnitToken => { panic!("can't cast a unit token"); }
                    CardType::SpellToken => { todo!("not yet supported"); }
                    CardType::Meta(_) => { todo!("this is a meta card"); }
                }
            }

            // must be grafting or modifying from discard
            FindCardResult::InPlayerDiscard(player, _, _) => {
                if player.id != issuer.id {
                    return Err(CardNotPlayableError::NotUnderPlayersControl(self.card_id).into())
                }

                todo!("card must be either grafting or augmenting")
            }

            FindCardResult::InPlayerDeck(_, _, _) |
            FindCardResult::InPlayerPack(_, _, _) |
            FindCardResult::InCommonDeck(_, _) => {
                Err(CardNotPlayableError::NotInPlayableZone(self.card_id).into())
            }
        }
    }

    fn get_valid(state: &State, db: &CardPrototypeDatabase) -> Vec<Action> {
        let mut actions = Vec::new();

        actions.extend(Self::valid_play_resource(state, db));
        actions.extend(Self::valid_play_haste(state, db));

        actions
    }

}

impl PlayCardAction {

    fn valid_play_haste(state: &State, db: &CardPrototypeDatabase) -> Vec<Action> {
        let mut actions : Vec<Action> = Vec::new();

        for region in &state.regions {

            if let Phase::PlanningPhase(PlanningPhaseStep::Haste(_)) = region.step {} else {
                continue
            }

            // assume single player per region at in the haste step
            let player = region.sole_player();

            // player must be on the team with active initiative
            if let Some(active_team_id) = region.active_team_id(state) {
                if player.team_id != active_team_id {
                    continue
                }
            }

            for card in player.hand.iter() {
                let proto = db.prototypes.get(&card.prototype_id).expect("a card prototype");
                match proto.card_type {
                    CardType::Unit(Timing::Haste) |
                    CardType::Spell(Timing::Haste) => {
                        actions.push(Action {
                            issuer_player_id: player.id,
                            action: ActionType::PlayCard(PlayCardAction {
                                card_id: card.card_id,
                            })
                        })
                    }
                    _ => {}
                }
            }
        }

        actions
    }

    fn valid_play_resource(state: &State, db: &CardPrototypeDatabase) -> Vec<Action> {
        let mut actions : Vec<Action> = Vec::new();

        // during the mana phase, players can play up to two resources per turn

        for region in &state.regions {

            if let Phase::PlanningPhase(PlanningPhaseStep::Mana(_)) = region.step {} else {
                continue
            }

            // assume single player per region at in the mana step
            let player = region.sole_player();

            // can't play more than two resources per round
            if player.resources_played_this_turn >= 2 {
                continue
            }

            // player must be on the team with active initiative
            if let Some(active_team_id) = region.active_team_id(state) {
                if player.team_id != active_team_id {
                    continue
                }
            }

            for card in player.hand.iter() {
                let proto = db.prototypes.get(&card.prototype_id).expect("a card prototype");
                if let CardType::Resource(_) = proto.card_type {
                    actions.push(Action {
                        issuer_player_id: player.id,
                        action: ActionType::PlayCard(PlayCardAction {
                            card_id: card.card_id,
                        })
                    })
                }
            }
        }

        actions
    }
}


impl Game {

    // this probably should be decomposed into play_card_from_hand / play_spell_token / etc.
    pub fn play_card(&self, mut state: State, card_id: CardId) -> Result<State, CardNotPlayableError> {

        // collect info about the card, or discover it doesn't exist or it's not in a playable zone
        let (player_id, prototype_id, in_hand, in_discard, in_play) = {
            let find_card_result = match state.find_card(card_id) {
                Ok(r) => {
                    r
                }
                Err(_) => {
                    return Err(CardDoesNotExist(card_id))
                }
            };

            match find_card_result {
                FindCardResult::InPlayerHand(player, _, card) => {
                    (player.id, card.prototype_id, true, false, false)
                }
                FindCardResult::InPlayerDiscard(player, _,  card) => {
                    (player.id, card.prototype_id, false, true, false)
                }
                FindCardResult::InCommonDeck(_, _) |
                FindCardResult::InPlayerPack(_, _, _) |
                FindCardResult::InPlayerDeck(_, _, _) => {
                    return Err(NotInPlayableZone(card_id));
                }
            }
        };

        let proto = self.cards_db.prototypes.get(&prototype_id).expect("a prototype");

        // check the timing requirements
        {
            let region_id = state.find_region_id_containing_player(player_id);
            let region = state.find_region(region_id).expect("a region");

            match &region.step {
                Phase::PlanningPhase(step) => {
                    match step {
                        PlanningPhaseStep::Mana(_)  => {
                            match proto.card_type {

                                // up to two resource cards are allowed during the mana step
                                CardType::Resource(_) => {
                                    let player = state.find_player(player_id).expect("a player");
                                    if player.resources_played_this_turn >= 2 {
                                        return Err(CannotPlayMoreResources(card_id));
                                    }
                                }

                                // haste cards are allowed during the mana step
                                CardType::Unit(Timing::Haste) => {}

                                // card can otherwise not be played during mana
                                _ => {
                                    return Err(CardLacksCorrectTiming(card_id));
                                }
                            }
                        }
                        _ => {
                            return Err(NotInPlayableStep(card_id));
                        }
                    }
                }
                phase @ Phase::BattlePhaseA(_) |
                phase @ Phase::BattlePhaseB(_) => {
                    if !phase.is_priority_window() {
                        return Err(NotInPlayableStep(card_id));
                    }

                    match &proto.card_type {
                        CardType::Spell(timing) |
                        CardType::Unit(timing) => {
                            match timing {
                                Timing::Battle => {}
                                Timing::Virus => {
                                    if !in_hand {
                                        return Err(MustBePlayedFromHand(card_id));
                                    }
                                }
                                _ => {
                                    return Err(MustBePlayedFromHand(card_id));
                                }
                            }
                        }
                        _ => {
                            return Err(MustBePlayedFromHand(card_id));
                        }
                    }
                }

                Phase::DeploymentPhase(step) => {
                    match step {
                        DeploymentPhaseStep::Regroup => {
                            return Err(MustBePlayedFromHand(card_id));
                        }
                        DeploymentPhaseStep::Deployment(_) => {}
                    }
                }
            }
        }

        fn remove_card(state: &mut State, player_id: PlayerId, card_id: CardId, in_hand: bool, in_discard: bool, in_play: bool) -> Option<Card> {
            // remove the card from the player's hand or discard
            let player = state.find_player_mut(player_id).expect("a player");
            if in_hand {
                Some(player.hand.remove(card_id).expect("card removed"))
            } else if in_discard {
                Some(player.discard.remove(card_id).expect("card removed"))
            } else if in_play {
                let region = state.find_region_containing_player_mut(player_id);
                let permanent_idx = region.unformed_permanents.iter().position(|p| {
                    match p {
                        Permanent::SpellToken { common, .. }  => {
                            // can find permanents be casting the CardId to a PermanentId
                            // maybe some better way, but this isn't too bad
                            common.permanent_id.0 == card_id.0
                        }
                        _ => {
                            panic!("can't cast a non-token permanent from play")
                        }
                    }
                });
                match permanent_idx {
                    None => {
                        panic!("permanent not found in play")
                    }
                    Some(idx) => {
                        region.unformed_permanents.remove(idx);
                        None

                    }
                }
            } else {
                panic!("card has to be somewhere")
            }
        }

        match proto.card_type {
            CardType::Unit(_)  => {

                let card = remove_card(&mut state, player_id, card_id, in_hand, in_discard, in_play).expect("a card");

                let permanent = Permanent::from_unit_card(card, player_id, &mut state, &self.cards_db);

                // add the permanent to the region the player is currently in
                let region_id = state.find_region_id_containing_player(player_id);
                let region = state.find_region_mut(region_id).expect("a region");
                region.unformed_permanents.push(permanent);

                // special case for resource, need to increment counter
                if let CardType::Resource(_) = proto.card_type {
                    let player = state.find_player_mut(player_id).expect("a player");
                    player.resources_played_this_turn += 1;
                }
            }


            CardType::SpellToken => {
                todo!("what types make sense to cast here?");
            }

            CardType::Spell(_) => {
                // spells just get cast
                remove_card(&mut state, player_id, card_id, in_hand, in_discard, in_play);
            }



            CardType::Resource(_) |
            CardType::UnitToken => {
                todo!("does this even make sense?")
            }

            CardType::Meta(_) => {
                todo!("this is a meta card type");
            }
        }


        Ok(state)
    }
}
