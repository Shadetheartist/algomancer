use crate::game::action::ActionValidationError;
use crate::game::state::card::CardId;
use crate::game::state::card_collection::CardCollectionId;
use crate::game::state::player::{PlayerId, TeamId};
use crate::game::state::region::RegionId;

#[derive(Debug)]
pub enum CardNotPlayableError {
    CardDoesNotExist,
    NotInPlayableZone,
    NotInPlayableStep,
    CardLacksCorrectTiming,
    CannotPlayMoreResources,
    MustBePlayedFromHand,
    CannotCastANonSpellTokenPermanentFromPlay,
}


#[derive(Debug)]
pub enum StateError {
    PlayerNotFound(PlayerId),
    RegionNotFound(RegionId),
    CardCollectionNotFound(CardCollectionId),
    CardNotFound(CardId),
    InvalidDraft,
    InvalidRecycle,
    NoPlayersOnTeam(TeamId),
    CardNotPlayable(CardNotPlayableError),
    MutationError,
    CannotDrawFromEmptyCollection,
    CannotDrawFromUnorderedSet,
    InvalidAction(ActionValidationError),
}
