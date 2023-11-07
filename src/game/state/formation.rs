use serde::{Deserialize, Serialize};
use crate::game::state::formation::FormationError::{FormationMustBeLocked, InvalidPosition};
use crate::game::state::formation::FormationPos::{BackRow, FrontRow};
use crate::game::state::formation::RemoveError::NothingToRemove;
use crate::game::state::permanent::Permanent;
use crate::game::state::player::PlayerId;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct Formation {
    owner_player_id: PlayerId,
    is_locked: bool,
    top_row: Vec<Option<Permanent>>,
    bot_row: Vec<Option<Permanent>>,
}


#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
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
    FormationMustBeLocked,
}

/// a formation has two fixed rows, and infinite columns
/// when unlocked, empty columns exist on either side of a column containing a permanent,
/// to facilitate inserting a permanent into 'template cells'
impl<'a> Formation {
    pub fn new(owner_player_id: PlayerId) -> Formation {
        Formation {
            owner_player_id: owner_player_id,
            is_locked: false,
            top_row: vec![None],
            bot_row: vec![None],
        }
    }

    /// reintroduces padding cells and allows modification until locked.
    pub fn unlock(&mut self) {
        if !self.is_locked {
            return;
        }

        // add column padding
        if self.top_row.len() == 0 {
            self.top_row.insert(0, None);
            self.bot_row.insert(0, None);
        } else {
            self.top_row.insert(0, None);
            self.bot_row.insert(0, None);

            self.top_row.push(None);
            self.bot_row.push(None);
        }

        self.is_locked = false;
    }

    /// removes padding cells and stops any modification until unlocked.
    pub fn lock(&mut self) {
        if self.is_locked {
            return;
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

        self.is_locked = true;
    }

    pub fn permanents_iter<'b>(&'b self) -> Box<dyn Iterator<Item=&'b Permanent> + 'b> {
        let top_iter = self.top_row.iter().filter(|cell| cell.is_some()).map(|cell| cell.as_ref().expect("a permanent"));
        let bot_iter = self.bot_row.iter().filter(|cell| cell.is_some()).map(|cell| cell.as_ref().expect("a permanent"));
        Box::new(top_iter.chain(bot_iter))
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

                // if unlocked, introduce new padding cells, if needed
                if !self.is_locked {
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

    pub fn remove_at(&mut self, pos: FormationPos, collapse: bool) -> Result<Permanent, FormationError> {


        let cell = self.get_at(pos)?;
        if let None = cell {
            return Err(FormationError::RemoveError(NothingToRemove));
        }

        match pos {
            FrontRow(col) => {
                // if a unit is removed from the front, the unit behind it moves up to replace it
                // (works even if the replacement is None)
                let replacement = std::mem::replace(&mut self.bot_row[col], None);
                let permanent = std::mem::replace(&mut self.top_row[col], replacement);
                let permanent = permanent.expect("a permanent");

                // locked formations don't have their columns automatically removed
                if !self.is_locked {
                    // units being removed from the front row may leave an empty column which is not on the edge.
                    // these empty columns are invalid and must be collapsed.
                    if col != 0 && col != self.top_row.len() - 1 {
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
        print_row(&self.top_row);
        print_row(&self.bot_row);
        println!();
    }
}

pub struct DefensiveFormation<'f> {
    attacking_formation: &'f Formation,
    formation: Formation,
}

impl<'f> DefensiveFormation<'f> {
    pub fn from_attacking_formation(owner_player_id: PlayerId, attacking_formation: &Formation) -> Result<DefensiveFormation, FormationError> {
        if !attacking_formation.is_locked {
            return Err(FormationMustBeLocked);
        }

        // fill defensive formation with cells to match each column in the offensive formation
        let defensive_row: Vec<Option<Permanent>> = attacking_formation.top_row.iter().map(|_| None).collect();
        let defensive_formation = DefensiveFormation {
            attacking_formation,
            formation: Formation {
                owner_player_id: owner_player_id,
                is_locked: true, // defensive formations start locked
                top_row: defensive_row.clone(),
                bot_row: defensive_row,
            },
        };

        Ok(defensive_formation)
    }

    pub fn print(&self) {
        print_row(&self.attacking_formation.bot_row);
        print_row(&self.attacking_formation.top_row);
        print_row(&self.formation.top_row);
        print_row(&self.formation.bot_row);

        println!();
    }
}

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


#[cfg(test)]
mod tests {
    use crate::game::state::card::{CardPrototypeId};
    use crate::game::state::formation::{DefensiveFormation, Formation, FormationPos};
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
        let mut formation = Formation::new(PlayerId(1));

        formation.print();

        formation.insert_at(FormationPos::FrontRow(0), fake_permanent()).expect("inserted permanent");
        formation.insert_at(FormationPos::FrontRow(0), fake_permanent()).expect("inserted permanent");
        formation.insert_at(FormationPos::FrontRow(0), fake_permanent()).expect("inserted permanent");

        formation.print();

        formation.insert_at(FormationPos::BackRow(1), fake_permanent()).expect("inserted permanent");

        formation.print();

        formation.remove_at(FormationPos::FrontRow(1), true).expect("inserted permanent");

        formation.print();

        formation.remove_at(FormationPos::FrontRow(1), true).expect("inserted permanent");

        formation.print();

        formation.remove_at(FormationPos::FrontRow(1), true).expect("inserted permanent");

        formation.print();

        formation.remove_at(FormationPos::FrontRow(1), true).expect("inserted permanent");

        formation.print();

        formation.insert_at(FormationPos::FrontRow(0), fake_permanent()).expect("inserted permanent");
        formation.insert_at(FormationPos::FrontRow(0), fake_permanent()).expect("inserted permanent");
        formation.insert_at(FormationPos::FrontRow(0), fake_permanent()).expect("inserted permanent");
        formation.print();

        formation.remove_at(FormationPos::FrontRow(2), true).expect("inserted permanent");
        formation.print();
    }

    #[test]
    fn test_defensive_formation() {
        let mut formation = Formation::new(PlayerId(1));
        formation.insert_at(FormationPos::FrontRow(0), fake_permanent()).expect("inserted permanent");
        formation.insert_at(FormationPos::FrontRow(0), fake_permanent()).expect("inserted permanent");
        formation.insert_at(FormationPos::FrontRow(0), fake_permanent()).expect("inserted permanent");
        formation.insert_at(FormationPos::BackRow(2), fake_permanent()).expect("inserted permanent");
        formation.lock();

        let mut defensive_formation = DefensiveFormation::from_attacking_formation(PlayerId(2), &formation).expect("a defensive formation");

        defensive_formation.formation.insert_at(FormationPos::FrontRow(0), fake_permanent()).expect("inserted permanent");
        defensive_formation.print();

    }
}