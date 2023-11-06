use crate::game::action::Action;
use crate::game::Game;
use crate::game::state::card::{Card, CardId, CardType, FindCardResult, Timing};
use crate::game::state::permanent::Permanent;
use crate::game::state::player::{CardNotPlayableError, PlayerId, StateError};
use crate::game::state::player::CardNotPlayableError::{CannotCastANonSpellTokenPermanentFromPlay, CannotPlayMoreResources, CardDoesNotExist, CardLacksCorrectTiming, MustBePlayedFromHand, NotInPlayableStep, NotInPlayableZone};
use crate::game::state::player::StateError::CardNotPlayable;
use crate::game::state::progression::{MainPhaseStep, Phase, PrecombatPhaseStep};
use crate::game::state::State;

impl Game {
    pub fn apply_play_card_action(&self, mut state: State, action: &Action) -> Result<State, StateError> {
        if let Action::PlayCard { card_id } = action {
            let card_id = *card_id;

            match self.play_card(state, card_id) {
                Ok(_state) => {
                    state = _state
                }
                Err(err) => {
                    return Err(CardNotPlayable(err));
                }
            }

            Ok(state)
        } else {
            panic!("only call this when the action is of the correct enum type")
        }
    }


    pub fn play_card(&self, mut state: State, card_id: CardId) -> Result<State, CardNotPlayableError> {

        // collect info about the card, or discover it doesn't exist or it's not in a playable zone
        let (player_id, prototype_id, in_hand, in_discard, in_play) = {
            let find_card_result = state.find_card(card_id);

            // validate that this player can actually play the card
            if let None = find_card_result {
                return Err(CardDoesNotExist);
            }

            let find_card_result = find_card_result.expect("a result");

            match find_card_result {
                FindCardResult::InPlayerHand(player, card) => {
                    (player.player_id, card.prototype_id, true, false, false)
                }
                FindCardResult::InPlayerDiscard(player, card) => {
                    (player.player_id, card.prototype_id, false, true, false)
                }
                FindCardResult::AsPermanent(_, permanent) => {
                    match permanent {
                        Permanent::SpellToken { common, card_prototype_id } => {
                            (common.controller_player_id, *card_prototype_id, false, false, true)
                        }
                        _ => {
                            return Err(CannotCastANonSpellTokenPermanentFromPlay);
                        }
                    }
                }
                FindCardResult::InDeck(_, _) => {
                    return Err(NotInPlayableZone);
                }
            }
        };

        // check the timing requirements
        {
            let region_id = state.find_region_id_containing_player(player_id);
            let region = state.find_region(region_id).expect("a region");
            let proto = self.cards_db.prototypes.get(&prototype_id).expect("a prototype");

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
                                        return Err(CannotPlayMoreResources);
                                    }
                                }

                                // haste cards are allowed during the mana step
                                CardType::Unit(Timing::Haste) => {}

                                // card can otherwise not be played during mana
                                _ => {
                                    return Err(CardLacksCorrectTiming);
                                }
                            }
                        }
                        _ => {
                            return Err(NotInPlayableStep);
                        }
                    }
                }
                phase @ Phase::CombatPhaseA(_) |
                phase @ Phase::CombatPhaseB(_) => {
                    if !phase.is_priority_window() {
                        return Err(NotInPlayableStep);
                    }

                    match &proto.card_type {
                        CardType::Spell(timing) |
                        CardType::Unit(timing) => {
                            match timing {
                                Timing::Combat => {}
                                Timing::Virus => {
                                    if !in_hand {
                                        return Err(MustBePlayedFromHand);
                                    }
                                }
                                _ => {
                                    return Err(CardLacksCorrectTiming);
                                }
                            }
                        }
                        _ => {
                            return Err(NotInPlayableStep);
                        }
                    }
                }

                Phase::MainPhase(step) => {
                    match step {
                        MainPhaseStep::Regroup => {
                            return Err(NotInPlayableStep);
                        }
                        MainPhaseStep::ITMain => {}
                        MainPhaseStep::NITMain => {}
                    }
                }
            }
        }

        fn remove_card(state: &mut State, player_id: PlayerId, card_id: CardId, in_hand: bool, in_discard: bool, in_play: bool) {
            // remove the card from the player's hand or discard
            let player = state.find_player_mut(player_id).expect("a player");
            if in_hand {
                let card_idx = player.hand.cards.iter().position(|c| c.card_id == card_id).expect("a card in hand");
                player.hand.cards.remove(card_idx);
            } else if in_discard {
                let card_idx = player.discard.cards.iter().position(|c| c.card_id == card_id).expect("a card in hand");
                player.discard.cards.remove(card_idx);
            } else if in_play {
                let region = state.find_region_containing_player_mut(player_id);
                let permanent_idx = region.permanents.iter().position(|p| {
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
                        region.permanents.remove(idx);
                    }
                }
            } else {
                panic!("card has to be somewhere")
            }
        }

        // create the permanent
        let proto = self.cards_db.prototypes.get(&prototype_id).expect("a prototype");

        match proto.card_type {
            CardType::UnitToken  => {}
            CardType::SpellToken |
            CardType::Unit(_) |
            CardType::Resource(_) => {
                let permanent = Permanent::from_card_prototype(proto, player_id, &mut state);

                // add the permanent to the region the player is currently in
                let region_id = state.find_region_id_containing_player(player_id);
                let region = state.find_region_mut(region_id).expect("a region");
                region.permanents.push(permanent);

                remove_card(&mut state, player_id, card_id, in_hand, in_discard, in_play);

                // special case for resource, need to increment counter
                if let CardType::Resource(_) = proto.card_type {
                    let player = state.find_player_mut(player_id).expect("a player");
                    player.resources_played_this_turn += 1;
                }
            }
            CardType::Spell(_) => {
                // spells just get cast
                remove_card(&mut state, player_id, card_id, in_hand, in_discard, in_play);
            }
        }


        Ok(state)
    }
}
