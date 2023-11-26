use serde::{Deserialize, Serialize};
use crate::game::state::card::CardId;
use crate::game::state::error::StateError;
use crate::game::state::mutation::StateMutation;
use crate::game::state::player::PlayerId;
use crate::game::state::State;


pub enum Next<'a> {
    TransitionStep,
    PassPriority(PlayerId),
    ResolveEffect(&'a Effect)
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Effect {
    source: CardId
}


#[derive(Clone, Serialize, Deserialize, Debug)]
#[derive(Default)]
pub struct Stack {
    priority: Vec<PlayerId>,
    stack: Vec<Effect>
}



impl Stack {
    pub fn push_effect(&mut self, state: &State, effect: Effect){
        self.stack.push(effect);
        self.priority.extend(state.players().map(|p| p.id));
    }

    pub fn acting_player(&self) -> Option<PlayerId>{
        self.priority.last().copied()
    }

    pub fn push_priority(&mut self, player_id: PlayerId) {
        self.priority.push(player_id);
    }

    pub fn clear_priority(&mut self) {
        self.priority.clear();
    }

    /// returns the id of the next player in priority queue, or None if all players passed priority
    pub fn pass_priority(&mut self) -> PlayerId {
        if !self.priority.is_empty() {
            self.priority.pop().expect("a player id")
        } else {
            panic!("cannot pass priority, no one has priority")
        }
    }

    pub fn next(&self) -> Next {
        if self.priority.is_empty() {
            if self.stack.is_empty() {
                Next::TransitionStep
            } else {
                Next::ResolveEffect(self.stack.last().expect("an effect to resolve"))
            }
        } else {
            Next::PassPriority(self.acting_player().expect("a player with priority"))
        }
    }

    pub fn resolve_next_effect(&mut self) -> Option<Result<Vec<StateMutation>, StateError>> {
        let effect = self.stack.pop();
        effect.map(|effect| self.resolve_effect(effect))
    }

    fn resolve_effect(&self, _effect: Effect) -> Result<Vec<StateMutation>, StateError>{
        Ok(Vec::new())
    }
}
