use crate::game::action::Action;
use crate::game::Game;
use crate::game::state::State;
use crate::game::state::card::{CardId, CardType, FindCardResult, Timing};
use crate::game::state::permanent::Permanent;
use crate::game::state::player::{CardNotPlayableError, PlayerId, StateError};
use crate::game::state::player::CardNotPlayableError::{CardDoesNotExist, CardLacksCorrectTiming, NotInPlayableStep, NotInPlayableZone, WrongPlayer};
use crate::game::state::progression::{Phase, PrecombatPhaseStep};

impl Game {

    pub fn apply_play_card_action(&self, mut state: State, action: &Action) -> Result<State, StateError> {
        if let Action::PlayCard { card_id } = action {
            let card_id = *card_id;

            // if the card is in a playable zone, it will be associated with a player
            // the player will be in a region, and we need to get the region's step to process the action
            let (player_id, card) = {
                // the card is either in a player's hand or discard, cards are not playable from other zones
                // (except glimpse? which needs some sort of special play area in a region)
                let find_card_result = state.find_card(card_id).expect("a card");
                match find_card_result {
                    FindCardResult::InPlayerHand(player, card) => {
                        (player.player_id, card)
                    }
                    FindCardResult::InPlayerDiscard(player, card) => {
                        (player.player_id, card)
                    }
                    _ => return Err(StateError::CardNotPlayable(NotInPlayableZone)),
                }
            };

            // we need to know the region's step to determine how/which cards are actually playable
            let region_id = state.find_region_id_containing_player(player_id);
            let region = state.find_region(region_id).expect("a region");
            match region.step {
                Phase::PrecombatPhase(PrecombatPhaseStep::ITMana) |
                Phase::PrecombatPhase(PrecombatPhaseStep::NITMana) => {
                    // if the card is being played during this step,
                    // it must be either a resource or a haste card
                    let proto = self.cards_db.prototypes.get(&card.prototype_id).expect("a prototype");
                    match proto.card_type {
                        CardType::Resource(_) => {
                            state = self.player_play_card(state, player_id, card_id).expect("a card was played")
                        }
                        CardType::Unit(Timing::Haste) => {

                        },
                        _ => {
                            return Err(StateError::CardNotPlayable(CardLacksCorrectTiming))
                        }
                    }
                }
                Phase::CombatPhaseA(_) => {}
                Phase::CombatPhaseB(_) => {}
                _ => return Err(StateError::CardNotPlayable(NotInPlayableStep))
            }

            Ok(state)
        } else {
            panic!("only call this when the action is of the correct enum type")
        }
    }

    pub fn player_play_card(&self, mut state: State, player_id: PlayerId, card_id: CardId) -> Result<State, CardNotPlayableError> {

        let (prototype_id, in_hand) = {
            let find_card_result = state.find_card(card_id);

            // validate that this player can actually play the card
            if let None = find_card_result {
                return Err(CardDoesNotExist)
            }

            let find_card_result = find_card_result.expect("a result");

            match find_card_result {
                FindCardResult::InPlayerHand(player, card) => {
                    if player.player_id != player_id {
                        return Err(WrongPlayer);
                    }

                    (card.prototype_id, true)
                }
                FindCardResult::InPlayerDiscard(player, card) => {
                    if player.player_id != player_id {
                        return Err(WrongPlayer);
                    }

                    (card.prototype_id, false)
                }
                FindCardResult::InDeck(_, _) |
                FindCardResult::AsPermanent(_, _) => {
                    return Err(NotInPlayableZone);
                }
            }
        };

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
