use anyhow::{anyhow, Result};

use crate::{
    state::RpnState,
    undo_action::{binary::BinaryFuncUndoEvent, UndoEvent},
};

use super::Func;

pub struct AddFunc {}

impl AddFunc {
    pub fn new() -> Self {
        Self {}
    }
}

impl Func for AddFunc {
    fn execute(&self, state: &mut RpnState) -> Result<Box<dyn UndoEvent>> {
        if state.stack.len() < 2 {
            return Err(anyhow!("Not enough arguments"));
        }
        let a = state.stack.peek(1).unwrap().clone();
        let b = state.stack.peek(0).unwrap().clone();
        let result = a.add(&b)?;
        state.stack.pop_n(2)?;
        state.stack.push(result.clone());
        Ok(Box::new(BinaryFuncUndoEvent::new(a, b, result)))
    }
}

#[cfg(test)]
mod tests {
    use crate::{stack::item::StackItem, state::RpnState};

    #[test]
    fn test_add() {
        let mut state = RpnState::new();
        state.push_str("1").unwrap();
        state.push_str("2").unwrap();
        state.push_str("add").unwrap();
        assert_eq!(state.stack.len(), 1);
        let answer = state.stack.peek(0).unwrap();
        assert_eq!(*answer, StackItem::Number { value: 3.0 });

        // test undo
        state.undo().unwrap();
        assert_eq!(2, state.stack.len());
        assert_eq!(
            *state.stack.peek(0).unwrap(),
            StackItem::Number { value: 2.0 }
        );
        assert_eq!(
            *state.stack.peek(1).unwrap(),
            StackItem::Number { value: 1.0 }
        );

        // test redo
        state.redo().unwrap();
        assert_eq!(state.stack.len(), 1);
        let answer = state.stack.peek(0).unwrap();
        assert_eq!(*answer, StackItem::Number { value: 3.0 });
    }
}
