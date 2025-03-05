use crate::ability::Ability;
use crate::database::Database;
use crate::object::{Object, ObjectId};
use crate::permanent::PermanentId;
use crate::state::{State, StateError};

impl State {
    pub(crate) fn state_based_actions(&mut self, db: &Database) -> Result<(), StateError>{

        while let Some(event) = self.event_queue.pop() {

        }

        Ok(())
    }

    fn permanents_with_triggered_abilities(&self, db: &Database) -> Result<Vec<PermanentId>, StateError> {

        self.permanents().filter_map(|p| {
            let obj_id: ObjectId = p.id.into();
            let obj = self.object(&obj_id);
            if let Err(err) = obj {
                return Some(Err(err.into()))
            }
            let obj = obj.unwrap();

            if let Object::Permanent { permanent } = obj {
                let card_data = db.card_data(&permanent.card_ref);
                if let Err(err) = card_data {
                    return Some(Err(err.into()))
                }
                let card_data = card_data.unwrap();

                let triggered_abilities = card_data.triggered_abilities();
                if triggered_abilities.count() > 0 {
                    return Some(Ok(permanent.id));
                }
            }
            None
        }).collect()
    }
}
