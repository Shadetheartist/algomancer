use std::cmp::Ordering;
use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use crate::game::action::attack::AttackAction;
use crate::game::action::draft::DraftAction;
use crate::game::action::pass_priority::PassPriorityAction;
use crate::game::action::play_card::PlayCardAction;
use crate::game::action::recycle_for_resource::RecycleForResourceAction;
use crate::game::db::CardPrototypeDatabase;
use crate::game::Game;
use crate::game::state::error::{StateError};
use crate::game::state::mutation::{StateMutation, StaticStateMutation};
use crate::game::state::player::{Player, PlayerId};
use crate::game::state::State;

mod draft;
mod pass_priority;
mod play_card;
mod recycle_for_resource;
mod attack;


pub trait ActionTrait: Sized {
    fn generate_mutations(&self, state: &State, db: &CardPrototypeDatabase, issuer: &Player) -> Result<Vec<StateMutation>, StateError>;
    fn get_valid(state: &State, db: &CardPrototypeDatabase) -> Vec<Action>;
}

#[derive(Hash, Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct Action {
    issuer_player_id: PlayerId,

    #[serde(flatten)]
    action: ActionType
}

impl PartialOrd for Action {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Action {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.action {
            ActionType::PassPriority(_) => {
                match other.action {
                    ActionType::PassPriority(_) => Ordering::Equal,
                    _ => Ordering::Less,
                }
            }
            _ => {
                Ordering::Equal
            }
        }
    }
}

#[derive(Hash, Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ActionType {
    PassPriority(PassPriorityAction),
    Draft(DraftAction),
    RecycleForResource(RecycleForResourceAction),
    PlayCard(PlayCardAction),
    Attack(AttackAction),
}

impl Action {
    fn generate_mutations(&self, game: &Game) -> Result<Vec<StateMutation>, StateError> {
        let issuer_player = game.state.find_player(self.issuer_player_id)?;
        match &self.action {
            ActionType::Draft(a) => a.generate_mutations(&game.state, &game.cards_db, issuer_player),
            ActionType::RecycleForResource(a) => a.generate_mutations(&game.state, &game.cards_db, issuer_player),
            ActionType::PlayCard(a) => a.generate_mutations(&game.state, &game.cards_db, issuer_player),
            ActionType::Attack(a) => a.generate_mutations(&game.state, &game.cards_db, issuer_player),
            ActionType::PassPriority(a) => a.generate_mutations(&game.state, &game.cards_db, issuer_player),
        }
    }
}



impl Game {
    pub fn apply_action(&mut self, action: Action) -> Result<Vec<StaticStateMutation>, StateError> {
        eprintln!("[{}] Applying Action [{:?}]", self.state.depth, &action);

        let mutations = action.generate_mutations(self)?;

        let mut static_mutations = Vec::new();

        if mutations.is_empty() {
            panic!("no mutations generated from action [{:?}]", action)
        }

        let mut next_state = self.state.clone();

        for mutation in mutations {
            let static_mutation = mutation.to_static(&next_state)?;
            for sub_mutation in static_mutation {
                next_state = next_state.mutate(&self.cards_db, &sub_mutation)?;
                static_mutations.push(sub_mutation);
            }
        }


        // just keep applying state based actions until there is nothing left to do
        loop {
            let num_starting_mutations = static_mutations.len();

            let state_based_mutations = next_state.generate_state_based_mutations();
            for mutation in state_based_mutations {
                let static_mutation = mutation.to_static(&next_state)?;
                for sub_mutation in static_mutation {
                    next_state = next_state.mutate(&self.cards_db, &sub_mutation)?;
                    static_mutations.push(sub_mutation);
                }
            }

            // stop when the number of static mutations didn't grow
            if num_starting_mutations == static_mutations.len() {
                break;
            }
        }

        for m in &static_mutations {
            eprintln!("- {:?}", &m);
        }

        self.action_history.push(action);

        next_state.depth += 1;
        self.state = next_state;

        Ok(static_mutations)
    }

    pub fn valid_actions(&self) -> HashSet<Action> {
        let mut actions = HashSet::new();

        actions.extend(PassPriorityAction::get_valid(&self.state, &self.cards_db));
        actions.extend(AttackAction::get_valid(&self.state, &self.cards_db));
        actions.extend(DraftAction::get_valid(&self.state, &self.cards_db));
        actions.extend(PlayCardAction::get_valid(&self.state, &self.cards_db));
        actions.extend(RecycleForResourceAction::get_valid(&self.state, &self.cards_db));

        actions
    }
}
