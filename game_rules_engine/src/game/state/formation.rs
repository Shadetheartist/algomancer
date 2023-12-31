use serde::{Deserialize, Serialize};

use crate::game::state::formation::FormationError::{AttackingFormationMustBeCommitted, FormationAlreadyCommitted, InvalidPosition};
use crate::game::state::formation::FormationPos::{BackRow, FrontRow};
use crate::game::state::formation::RemoveError::NothingToRemove;
use crate::game::state::player::PlayerId;

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub struct FormationId(pub usize);

#[derive(Hash, Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct Formation<T> {
    formation_id: FormationId,
    owner_player_id: PlayerId,
    padding_cells_enabled: bool,
    committed: bool,
    top_row: Vec<Option<T>>,
    bot_row: Vec<Option<T>>,
}


#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
    FormationAlreadyCommitted,
    AttackingFormationMustBeCommitted,
}

/// a formation has two fixed rows, and infinite columns
/// when unlocked, empty columns exist on either side of a column containing a permanent,
/// to facilitate inserting a permanent into 'template cells'
impl<'a, T> Formation<T> {
    pub fn new(id: FormationId, owner_player_id: PlayerId) -> Formation<T> {
        Formation {
            formation_id: id,
            owner_player_id,
            padding_cells_enabled: true,
            committed: false,
            top_row: vec![None],
            bot_row: vec![None],
        }
    }

    /// reintroduces padding cells and allows modification until locked.
    pub fn enable_padding_cells(&mut self) -> Result<(), FormationError>{
        if self.committed {
            return Err(FormationAlreadyCommitted)
        }

        if self.padding_cells_enabled {
            return Ok(());
        }

        // add column padding
        if self.top_row.is_empty() {
            self.top_row.insert(0, None);
            self.bot_row.insert(0, None);
        } else {
            self.top_row.insert(0, None);
            self.bot_row.insert(0, None);

            self.top_row.push(None);
            self.bot_row.push(None);
        }

        self.padding_cells_enabled = true;

        Ok(())
    }

    /// removes padding cells and stops any modification until unlocked.
    pub fn disable_padding_cells(&mut self) -> Result<(), FormationError>{
        if self.committed {
            return Err(FormationAlreadyCommitted)
        }

        if !self.padding_cells_enabled {
            return Ok(());
        }

        // remove column padding
        self.top_row.remove(0);
        self.bot_row.remove(0);


        // if, after removing the left padding, there are still two or more columns,
        // remove the right column. This avoids panics when an empty formation is locked.
        if self.top_row.len() >= 2 {
            let l = self.top_row.len() - 1;
            self.top_row.remove(l);
            self.bot_row.remove(l);
        }

        self.padding_cells_enabled = false;

        Ok(())
    }

    pub fn cells_iter<'b>(&'b self) -> Box<dyn Iterator<Item=&'b T> + 'b> {
        let top_iter = self.top_row.iter().filter_map(|cell| cell.as_ref());
        let bot_iter = self.bot_row.iter().filter_map(|cell| cell.as_ref());
        Box::new(top_iter.chain(bot_iter))
    }

    pub fn get_at(&'a self, pos: FormationPos) -> Result<Option<&'a T>, FormationError> {
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


    pub fn insert_at(&mut self, pos: FormationPos, permanent: T) -> Result<(), FormationError> {
        if self.committed {
            return Err(FormationAlreadyCommitted)
        }

        // validate the insert
        {
            let cell = self.get_at(pos)?;
            if cell.is_some() {
                return Err(FormationError::InsertError(InsertError::CellIsOccupied));
            }

            // if an insert is being attempted on the back row
            if let BackRow(col) = &pos {
                // and there is no permanent in the front row
                let front_cell = self.get_at(FrontRow(*col))?;
                if front_cell.is_none() {
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

                // introduce new padding cells, if needed
                if self.padding_cells_enabled {
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
            }
            BackRow(col) => {
                self.bot_row[col] = Some(permanent);
            }
        }

        Ok(())
    }

    pub fn remove_at(&mut self, pos: FormationPos) -> Result<T, FormationError> {
        if self.committed {
            return Err(FormationAlreadyCommitted)
        }

        let cell = self.get_at(pos)?;
        if cell.is_none() {
            return Err(FormationError::RemoveError(NothingToRemove));
        }

        match pos {
            FrontRow(col) => {
                // if a unit is removed from the front, the unit behind it moves up to replace it
                // (works even if the replacement is None)
                let replacement = self.bot_row[col].take();
                let item = std::mem::replace(&mut self.top_row[col], replacement);
                let item = item.expect("a permanent");

                // locked formations don't have their columns automatically removed
                if self.padding_cells_enabled {
                    // units being removed from the front row may leave an empty column which is not on the edge.
                    // these empty columns are invalid and must be collapsed.
                    if col != 0 && col != self.top_row.len() - 1 {
                        // re-check the top cell, if it's None, then the bottom cell is also None.
                        // and therefore the column needs to be removed entirely
                        if self.top_row[col].is_none() {
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
                }

                Ok(item)
            }
            BackRow(col) => {
                let item = self.bot_row[col].take();
                let item = item.expect("a permanent");

                // units being removed from the back row should never cause a collapse
                Ok(item)
            }
        }
    }

    pub fn commit(&mut self) -> Result<(), FormationError>{
        self.disable_padding_cells()?;
        self.committed = true;
        Ok(())
    }

    pub fn print(&self) {
        print_row(&self.top_row);
        print_row(&self.bot_row);
    }

    pub fn rprint(&self) {
        print_row(&self.bot_row);
        print_row(&self.top_row);
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct DefensiveFormation<T> {
    pub attacking_formation_id: FormationId,
    pub formation: Formation<T>,
}

impl <T> DefensiveFormation<T> {
    pub fn from_attacking_formation<A>(id: FormationId, owner_player_id: PlayerId, attacking_formation: &Formation<A>) -> Result<DefensiveFormation<T>, FormationError> {
        if !attacking_formation.committed {
            return Err(AttackingFormationMustBeCommitted);
        }

        // fill defensive formation with cells to match each column in the offensive formation
        let defensive_formation = DefensiveFormation {
            attacking_formation_id: attacking_formation.formation_id,
            formation: Formation {
                formation_id: id,
                owner_player_id,
                padding_cells_enabled: false, // defensive formations start locked
                committed: false,
                top_row: attacking_formation.top_row.iter().map(|_| None).collect(),
                bot_row: attacking_formation.top_row.iter().map(|_| None).collect(),
            },
        };

        Ok(defensive_formation)
    }
}

fn print_row<T>(row: &Vec<Option<T>>) {
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


#[cfg(test)]
mod tests {
    use database::CardPrototypeId;
    use crate::game::state::formation::{DefensiveFormation, Formation, FormationId, FormationPos};
    use crate::game::state::permanent::{Permanent, PermanentCommon, PermanentId};
    use crate::game::state::player::PlayerId;

    fn fake_permanent() -> Permanent {
        Permanent::UnitToken {
            common: PermanentCommon {
                permanent_id: PermanentId(1),
                controller_player_id: PlayerId(1),
            },
            card_prototype_id: CardPrototypeId(1),
        }
    }

    #[test]
    fn test_formation() {
        let mut formation = Formation::new(FormationId(1), PlayerId(1));

        formation.print();

        formation.insert_at(FormationPos::FrontRow(0), fake_permanent()).expect("inserted permanent");
        formation.print();
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

        formation.insert_at(FormationPos::FrontRow(0), fake_permanent()).expect("inserted permanent");
        formation.insert_at(FormationPos::FrontRow(0), fake_permanent()).expect("inserted permanent");
        formation.insert_at(FormationPos::FrontRow(0), fake_permanent()).expect("inserted permanent");
        formation.print();

        formation.remove_at(FormationPos::FrontRow(2)).expect("inserted permanent");
        formation.print();
    }

    #[test]
    fn test_defensive_formation() {
        let mut formation = Formation::new(FormationId(1), PlayerId(1));

        formation.insert_at(FormationPos::FrontRow(0), fake_permanent()).expect("inserted permanent");
        formation.insert_at(FormationPos::FrontRow(0), fake_permanent()).expect("inserted permanent");
        formation.insert_at(FormationPos::FrontRow(0), fake_permanent()).expect("inserted permanent");
        formation.insert_at(FormationPos::BackRow(2), fake_permanent()).expect("inserted permanent");
        formation.commit().expect("a committed formation");

        let mut defensive_formation = DefensiveFormation::from_attacking_formation(FormationId(2), PlayerId(2), &formation).expect("a defensive formation");

        defensive_formation.formation.insert_at(FormationPos::FrontRow(0), fake_permanent()).expect("inserted permanent");
        formation.rprint();
        defensive_formation.formation.print();

    }
}