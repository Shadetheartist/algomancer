use crate::player::PlayerId;

#[derive(Clone, Hash, PartialEq, Eq, Default)]
pub struct Priority(pub(crate) Option<PlayerId>);