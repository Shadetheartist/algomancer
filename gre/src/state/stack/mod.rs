use thiserror::Error;
use crate::object::ObjectId;

#[derive(Error, Debug)]
pub enum StackError {
    #[error("nothing to resolve")]
    NothingToResolve
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Stack {
    data: Vec<ObjectId>
}

impl Default for Stack {
    fn default() -> Self {
        Self { data: Default::default() }
    }
}

impl Stack {
    pub fn push(&mut self, object_id: ObjectId) {
        self.data.push(object_id)
    }

    pub fn pop(&mut self) -> Result<ObjectId, StackError> {
        self.data.pop().ok_or(StackError::NothingToResolve)
    }
}

#[cfg(test)]
mod test {
    use crate::object::ObjectId;
    use super::{Stack};

    #[test]
    fn test_stack() {
        let mut stack = Stack::default();
        stack.push(ObjectId::default());
        assert_eq!(stack.data.len(), 1);
        let _ = stack.pop().unwrap();
        assert_eq!(stack.data.len(), 0);
    }
}