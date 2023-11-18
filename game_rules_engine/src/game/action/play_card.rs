use crate::game::action::Action;
use crate::game::Game;
use crate::game::state::card::{Card, CardId, CardType, FindCardResult, Timing};
use crate::game::state::error::{CardNotPlayableError,StateError};
use crate::game::state::error::CardNotPlayableError::{CannotCastANonSpellTokenPermanentFromPlay, CannotPlayMoreResources, CardDoesNotExist, CardLacksCorrectTiming, MustBePlayedFromHand, NotInPlayableStep, NotInPlayableZone};
use crate::game::state::mutation::StateMutation;
use crate::game::state::permanent::Permanent;
use crate::game::state::player::PlayerId;
use crate::game::state::progression::{MainPhaseStep, Phase, PrecombatPhaseStep};
use crate::game::state::State;

impl Game {

    pub fn generate_play_card_mutations(&self, action: &Action) -> Result<Vec<StateMutation>, StateError> {
        if let Action::PlayCard { card_id: _ } = action {
            let mutations = Vec::new();

            Ok(mutations)
        } else {
            panic!("only call this when the action is of the correct enum type")
        }
    }

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
                        PrecombatPhaseStep::ITMana |
                        PrecombatPhaseStep::NITMana => {
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
                        MainPhaseStep::ITMain => {}
                        MainPhaseStep::NITMain => {}
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
