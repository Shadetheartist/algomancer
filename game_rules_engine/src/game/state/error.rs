use crate::game::state::card::CardId;
use crate::game::state::card_collection::CardCollectionId;
use crate::game::state::player::{PlayerId};
use crate::game::state::region::RegionId;
use thiserror::Error;
use crate::game::state::permanent::PermanentId;

#[derive(Error, Debug)]
#[error("card not playable")]
pub enum CardNotPlayableError {
    CardDoesNotExist(CardId),
    NotUnderPlayersControl(CardId),
    NotInPlayableZone(CardId),
    NotInPlayableStep(CardId),
    CardLacksCorrectTiming(CardId),
    CannotPlayMoreResources(CardId),
    MustBePlayedFromHand(CardId),
    CannotCastANonSpellTokenPermanentFromPlay(PermanentId),
}

#[derive(Error, Debug)]
#[error("entity not found error")]
pub enum EntityNotFoundError {
    #[error("player [{0}] not found in state")]
    Player(PlayerId),

    #[error("region [{0}] not found in state")]
    Region(RegionId),

    #[error("card collection [{0}] not found in state")]
    CardCollection(CardCollectionId),

    #[error("card [{0}] not found in state")]
    Card(CardId),
}

#[derive(Error, Debug)]
#[error("card collection error")]
pub enum CardCollectionError {
    #[error("card collection [{0}] is empty, and cannot be drawn from")]
    CannotDrawFromEmptyCollection(CardCollectionId),

    #[error("card collection [{0}] is not being used properly due to its order/unordered-ness")]
    UnexpectedOrdering(CardCollectionId),
}


#[derive(Error, Debug)]
#[error("draft error")]
pub enum DraftError {
    IncorrectNumberOfCardsDrafted,
    CardNotInHand(CardId),
    InvalidPackCard(CardId, &'static str),
}


#[derive(Error, Debug)]
#[error("invalid action")]
pub enum InvalidActionError {
    InvalidDraft(DraftError),
    InvalidRecycle,
}


#[derive(Error, Debug)]
#[error("state error")]
pub enum StateError {
    #[error("entity not found")]
    EntityNotFound(#[from] EntityNotFoundError),

    #[error("error relating to card collection")]
    CardCollection(#[from] CardCollectionError),

    #[error("the card [{0:?}] is not playable")]
    CardNotPlayable(#[from] CardNotPlayableError),

    #[error("action is invalid")]
    InvalidAction(#[from] InvalidActionError),

    #[error("player [{0:?}] cannot pass priority")]
    CannotPassPriority(PlayerId),

    #[error("some other error")]
    Other
}
