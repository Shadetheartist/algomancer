use crate::game::state::formation::FormationError::InvalidPosition;
use crate::game::state::formation::FormationPos::{BackRow, FrontRow};
use crate::game::state::formation::RemoveError::NothingToRemove;
use crate::game::state::permanent::Permanent;

#[derive(Debug)]
pub struct Formation {
    top_row: Vec<Option<Permanent>>,
    bot_row: Vec<Option<Permanent>>,
}

#[derive(Copy, Clone, Debug)]
pub enum FormationPos {
    FrontRow(usize),
    BackRow(usize),
}

#[derive(Debug)]
pub enum InsertError {
    CellIsOccupied,
    FrontCellIsEmpty,
}

#[derive(Debug)]
pub enum RemoveError {
    NothingToRemove,
}

#[derive(Debug)]
pub enum FormationError {
    InvalidPosition,
    InsertError(InsertError),
    RemoveError(RemoveError),
}

/// a formation has two fixed rows, and infinite columns
/// empty columns are created on either side of a column containing a permanent
impl<'a> Formation {
    pub fn new() -> Formation {
        Formation {
            top_row: vec![None],
            bot_row: vec![None],
        }
    }

    pub fn get_at(&'a self, pos: FormationPos) -> Result<Option<&'a Permanent>, FormationError> {
        match pos {
            FrontRow(col) => {
                if col >= self.top_row.len() {
                    Err(InvalidPosition)
                } else {
                    Ok(self.top_row[col].as_ref())
                }
            }
            BackRow(col) => {
                if col >= self.bot_row.len() {
                    Err(InvalidPosition)
                } else {
                    Ok(self.bot_row[col].as_ref())
                }
            }
        }
    }


    pub fn insert_at(&mut self, pos: FormationPos, permanent: Permanent) -> Result<(), FormationError> {

        // validate the insert
        {
            let cell = self.get_at(pos)?;
            if let Some(_) = cell {
                return Err(FormationError::InsertError(InsertError::CellIsOccupied));
            }

            // if an insert is being attempted on the back row
            if let BackRow(col) = &pos {
                // and there is no permanent in the front row
                let front_cell = self.get_at(FrontRow(*col))?;
                if let None = front_cell {
                    // cannot insert into the back row if there is not already a permanent
                    // in the front row of this col
                    return Err(FormationError::InsertError(InsertError::FrontCellIsEmpty));
                }
            }
        }

        // insert is valid

        match pos {
            FrontRow(col) => {
                self.top_row[col] = Some(permanent);

                // since new columns must start on the first row, we check here to make sure
                // that there is column on either side of this one, if not, we make one

                // if the insertion happened on the rightmost column,
                // we need to create a new column to the right (aka push)
                if col == self.top_row.len() - 1 {
                    self.top_row.push(None);
                    self.bot_row.push(None);
                }

                // if the leftmost column was inserted into, we need to create a column at -1,
                // since we can't do that, we need to shift everything over one
                // inserting here is always going to make a new column to the left
                if col == 0 {
                    self.top_row.insert(0, None);
                    self.bot_row.insert(0, None);
                }
            }
            BackRow(col) => {
                self.bot_row[col] = Some(permanent);
            }
        }

        Ok(())
    }

    pub fn remove_at(&mut self, pos: FormationPos) -> Result<Permanent, FormationError> {
        let cell = self.get_at(pos)?;
        if let None = cell {
            return Err(FormationError::RemoveError(NothingToRemove))
        }

        match pos {
            FrontRow(col) => {
                // if a unit is removed from the front, the unit behind it moves up to replace it
                // (works even if the replacement is None)
                let replacement = std::mem::replace(&mut self.bot_row[col], None);
                let permanent = std::mem::replace(&mut self.top_row[col], replacement);
                let permanent = permanent.expect("a permanent");

                // units being removed from the front row may leave an empty column which is not on the edge.
                // these empty columns are invalid and must be collapsed.
                if col != 0 && col != self.top_row.len()-1 {
                    // re-check the top cell, if it's None, then the bottom cell is also None.
                    // and therefore the column needs to be removed entirely
                    if self.top_row[col] == None {
                        self.top_row.remove(col);
                        self.bot_row.remove(col);
                    }
                }

                // if we remove everything, there will be exactly two empty columns remaining
                // at this point we should reset to the initial state by deleting one of the columns
                if self.top_row.len() == 2 {
                    self.top_row.remove(1);
                    self.bot_row.remove(1);
                }

                Ok(permanent)
            }
            BackRow(col) => {
                let permanent = std::mem::replace(&mut self.bot_row[col], None);
                let permanent = permanent.expect("a permanent");

                // units being removed from the back row should never cause a collapse
                Ok(permanent)
            }
        }
    }


    pub fn print(&self) {
        fn print_row(row: &Vec<Option<Permanent>>) {
            for p in row {
                match p {
                    None => {
                        print!("◻️️");
                    }
                    Some(_) => {
                        print!("◼");
                    }
                }
            }
            println!();
        }

        print_row(&self.top_row);
        print_row(&self.bot_row);
        println!();
    }
}

#[cfg(test)]
mod tests {
    use crate::game::state::card::{CardPrototypeId};
    use crate::game::state::formation::{Formation, FormationPos};
    use crate::game::state::permanent::{Permanent, PermanentCommon, PermanentId};
    use crate::game::state::player::PlayerId;

    #[test]
    fn test_formation() {
        let mut formation = Formation::new();

        fn fake_permanent() -> Permanent {
            Permanent::UnitToken {
                common: PermanentCommon {
                    permanent_id: PermanentId(1),
                    controller_player_id: PlayerId(1),
                },
                card_prototype_id: CardPrototypeId(1),
            }
        }

        for _ in 0..3 {
            formation.print();

            formation.insert_at(FormationPos::FrontRow(0), fake_permanent()).expect("inserted permanent");
            formation.insert_at(FormationPos::FrontRow(0), fake_permanent()).expect("inserted permanent");
            formation.insert_at(FormationPos::FrontRow(0), fake_permanent()).expect("inserted permanent");

            formation.print();

            formation.insert_at(FormationPos::BackRow(1), fake_permanent()).expect("inserted permanent");

            formation.print();

            formation.remove_at(FormationPos::FrontRow(1)).expect("inserted permanent");

            formation.print();

            formation.remove_at(FormationPos::FrontRow(1)).expect("inserted permanent");

            formation.print();

            formation.remove_at(FormationPos::FrontRow(1)).expect("inserted permanent");

            formation.print();

            formation.remove_at(FormationPos::FrontRow(1)).expect("inserted permanent");

            formation.print();
        }


    }
}