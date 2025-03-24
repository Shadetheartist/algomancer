use crate::database::Database;
use crate::state::object::{Object, ObjectId};
use crate::state::permanent::PermanentId;
use crate::state::{State, StateError};

impl State {
    pub(crate) fn state_based_actions(&mut self, db: &Database) -> Result<(), StateError>{

        let permanents_with_triggered_abilities = self.permanents_with_triggered_abilities(db)?;
        while let Some(event) = self.event_queue.pop() {
            for permanent_id in &permanents_with_triggered_abilities {
                let permanent = self.permanent_mut(permanent_id)?;
                let card_data = db.card_data(&permanent.card_ref)?;
                for ability in card_data.triggered_abilities() {
                    println!("{:?}", ability);
                }
            }
        }

        Ok(())
    }

    fn permanents_with_triggered_abilities(&self, db: &Database) -> Result<Vec<PermanentId>, StateError> {
        let mut permanents = Vec::new();
        for p in self.permanents() {
            let obj_id: ObjectId = p.id.into();
            let obj = self.object(&obj_id)?;

            if let Object::Permanent { permanent } = obj {
                let card_data = db.card_data(&permanent.card_ref)?;
                if card_data.triggered_abilities().next().is_some() {
                    permanents.push(permanent.id)
                }
            }
        }
        Ok(permanents)
    }
}
