use crate::game::action::Action;
use crate::game::Game;
use crate::game::state::card::{CardId, CardType, FindCardResult, Timing};
use crate::game::state::permanent::Permanent;
use crate::game::state::player::{CardNotPlayableError, StateError};
use crate::game::state::player::CardNotPlayableError::{CannotPlayMoreResources, CardDoesNotExist, CardLacksCorrectTiming, MustBePlayedFromHand, NotInPlayableStep, NotInPlayableZone};
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
                    return Err(CardNotPlayable(err))
                }
            }

            Ok(state)
        } else {
            panic!("only call this when the action is of the correct enum type")
        }
    }



    pub fn play_card(&self, mut state: State, card_id: CardId) -> Result<State, CardNotPlayableError> {


        let (player_id, prototype_id, in_hand) = {
            let find_card_result = state.find_card(card_id);

            // validate that this player can actually play the card
            if let None = find_card_result {
                return Err(CardDoesNotExist)
            }

            let find_card_result = find_card_result.expect("a result");

            match find_card_result {
                FindCardResult::InPlayerHand(player, card) => {
                    (player.player_id, card.prototype_id, true)
                }
                FindCardResult::InPlayerDiscard(player, card) => {
                    (player.player_id, card.prototype_id, false)
                }
                FindCardResult::InDeck(_, _) |
                FindCardResult::AsPermanent(_, _) => {
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
                                        return Err(CannotPlayMoreResources)
                                    }
                                }

                                // haste cards are allowed during the mana step
                                CardType::Unit(Timing::Haste) => {},

                                // card can otherwise not be played during mana
                                _ => {
                                    return Err(CardLacksCorrectTiming)
                                }
                            }
                        },
                        _ => {
                            return Err(NotInPlayableStep)
                        }
                    }
                }
                phase @ Phase::CombatPhaseA(_) |
                phase @ Phase::CombatPhaseB(_) => {
                    if !phase.is_priority_window() {
                        return Err(NotInPlayableStep)
                    }

                    match &proto.card_type {
                        CardType::UnitToken => {}
                        CardType::SpellToken => {}
                        CardType::Spell(timing) |
                        CardType::Unit(timing) => {
                            match timing {
                                Timing::Combat => {}
                                Timing::Virus => {
                                    if !in_hand {
                                        return Err(MustBePlayedFromHand)
                                    }
                                }
                                _ => {
                                    return Err(CardLacksCorrectTiming)
                                }
                            }
                        }
                        _ => {
                            return Err(NotInPlayableStep)
                        }
                    }
                }

                Phase::MainPhase(step) => {
                    match step {
                        MainPhaseStep::Regroup => {
                            return Err(NotInPlayableStep)
                        }
                        MainPhaseStep::ITMain => {

                        }
                        MainPhaseStep::NITMain => {}
                    }
                }
            }
        }


        // create the permanent
        let proto = self.cards_db.prototypes.get(&prototype_id).expect("a prototype");
        let permanent = Permanent::from_card_prototype(proto, player_id, &mut state);

        // add the permanent to the region the player is currently in
        let region_id = state.find_region_id_containing_player(player_id);
        let region = state.find_region_mut(region_id).expect("a region");
        region.permanents.push(permanent);

        // remove the card from the player's hand or discard
        let player = state.find_player_mut(player_id).expect("a player");
        if in_hand {
            let card_idx = player.hand.cards.iter().position(|c| c.card_id == card_id).expect("a card in hand");
            player.hand.cards.remove(card_idx);
        } else {
            let card_idx = player.discard.cards.iter().position(|c| c.card_id == card_id).expect("a card in hand");
            player.discard.cards.remove(card_idx);
        }

        if let CardType::Resource(_) = proto.card_type {
            player.resources_played_this_turn += 1;
        }

        Ok(state)
    }
}
