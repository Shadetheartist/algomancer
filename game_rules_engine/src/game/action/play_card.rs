
use serde::{Deserialize, Serialize};
use crate::game::action::{Action, ActionTrait, ActionType};
use crate::game::db::CardPrototypeDatabase;
use crate::game::{Game};
use crate::game::state::card::{Card, CardId, CardType, FindCardResult, Timing};
use crate::game::state::card::CardType::{Resource, Unit};
use crate::game::state::error::{CardNotPlayableError,StateError};
use crate::game::state::error::CardNotPlayableError::{CannotCastANonSpellTokenPermanentFromPlay, CannotPlayMoreResources, CardDoesNotExist, CardLacksCorrectTiming, MustBePlayedFromHand, NotInPlayableStep, NotInPlayableZone};
use crate::game::state::mutation::StateMutation;
use crate::game::state::permanent::Permanent;
use crate::game::state::permanent::Permanent::SpellToken;
use crate::game::state::player::{Player, PlayerId};
use crate::game::state::progression::{MainPhaseStep, Phase, PrecombatPhaseStep, Team};
use crate::game::state::State;


#[derive(Hash, Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlayCardAction {
    pub card_id: CardId
}

impl PlayCardAction {

}

impl ActionTrait for PlayCardAction {
    fn generate_mutations(&self, state: &State, _db: &CardPrototypeDatabase, issuer: &Player) -> Result<Vec<StateMutation>, StateError> {
        match state.find_card(self.card_id)? {
            FindCardResult::InPlayerHand(player, _, _card) => {
                if player.id != issuer.id {
                    return Err(CardNotPlayableError::NotUnderPlayersControl(self.card_id).into())
                }
            }

            // must be grafting or modifying from discard
            FindCardResult::InPlayerDiscard(player, _, _) => {
                if player.id != issuer.id {
                    return Err(CardNotPlayableError::NotUnderPlayersControl(self.card_id).into())
                }

                todo!("card must be either grafting or augmenting")
            }

            // must be casting a spell token
            FindCardResult::AsPermanentInRegion(_, permanent) => {
                if let SpellToken {common, ..} = permanent {
                    if common.controller_player_id != issuer.id {
                        return Err(CardNotPlayableError::NotUnderPlayersControl(common.permanent_id).into())
                    }
                } else {
                    let id = match permanent {
                        Permanent::Unit { common, .. } => { common.permanent_id }
                        Permanent::Resource { common, .. } => { common.permanent_id }
                        Permanent::UnitToken { common, .. } => { common.permanent_id }
                        SpellToken { .. } => panic!("how did it come to this")
                    };
                    return Err(CardNotPlayableError::CannotCastANonSpellTokenPermanentFromPlay(id).into())
                }
            }

            FindCardResult::InPlayerDeck(_, _, _) |
            FindCardResult::InPlayerPack(_, _, _) |
            FindCardResult::InCommonDeck(_, _) |
            FindCardResult::AsPermanentInFormation(_, _, _) => {
                return Err(CardNotPlayableError::NotInPlayableZone(self.card_id).into())
            }
        };

        Ok(Vec::new())
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
            let player = region.sole_player();
            if player.team_id == state.initiative_team() {
                if let Phase::PrecombatPhase(PrecombatPhaseStep::Mana(Team::IT)) = region.step {}
                else {
                    continue
                }
            } else if let Phase::PrecombatPhase(PrecombatPhaseStep::Mana(Team::NIT)) = region.step {}
            else {
                continue
            }

            // assume single player per region at in the mana step
            let player = region.sole_player();

            for card in player.hand.iter() {
                let proto = db.prototypes.get(&card.prototype_id).expect("a card prototype");
                if let Unit(Timing::Haste) = proto.card_type {
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

    fn valid_play_resource(state: &State, db: &CardPrototypeDatabase) -> Vec<Action> {
        let mut actions : Vec<Action> = Vec::new();

        // during the mana phase, players can play up to two resources per turn

        for region in &state.regions {

            if let Phase::PrecombatPhase(PrecombatPhaseStep::Mana(_)) = region.step {} else {
                break
            }

            // assume single player per region at in the mana step
            let player = region.sole_player();

            if player.resources_played_this_turn >= 2 {
                break
            }

            for card in player.hand.iter() {
                let proto = db.prototypes.get(&card.prototype_id).expect("a card prototype");
                if let Resource(_) = proto.card_type {
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
                FindCardResult::AsPermanentInRegion(_, permanent) => {
                    match permanent {
                        Permanent::SpellToken { common, card_prototype_id } => {
                            (common.controller_player_id, *card_prototype_id, false, false, true)
                        }
                        _ => {
                            return Err(CannotCastANonSpellTokenPermanentFromPlay(card_id));
                        }
                    }
                }
                FindCardResult::AsPermanentInFormation(_, _, _) |
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
                Phase::PrecombatPhase(step) => {
                    match step {
                        PrecombatPhaseStep::Mana(_)  => {
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
                phase @ Phase::CombatPhaseA(_) |
                phase @ Phase::CombatPhaseB(_) => {
                    if !phase.is_priority_window() {
                        return Err(NotInPlayableStep(card_id));
                    }

                    match &proto.card_type {
                        CardType::Spell(timing) |
                        CardType::Unit(timing) => {
                            match timing {
                                Timing::Combat => {}
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

                Phase::MainPhase(step) => {
                    match step {
                        MainPhaseStep::Regroup => {
                            return Err(MustBePlayedFromHand(card_id));
                        }
                        MainPhaseStep::Main(_) => {}
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

        }


        Ok(state)
    }
}
