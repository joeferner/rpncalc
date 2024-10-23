use anyhow::{anyhow, Result};

use crate::{stack::item::StackItem, state::RpnState};

use super::UndoEvent;

#[derive(Debug)]
pub struct BinaryFuncUndoEvent {
    a: StackItem,
    b: StackItem,
    result: StackItem,
}

impl BinaryFuncUndoEvent {
    pub fn new(a: StackItem, b: StackItem, result: StackItem) -> Self {
        Self { a, b, result }
    }
}

impl UndoEvent for BinaryFuncUndoEvent {
    fn undo(&self, state: &mut RpnState) -> Result<()> {
        if state.stack.len() < 1 {
            return Err(anyhow!(
                "unexpected state for undo, expected item on the stack but found none"
            ));
        }
        let found_result = state.stack.peek(0).unwrap();
        if *found_result != self.result {
            return Err(anyhow!(
                "unexpected state for undo, expected item on the stack to be the same as the result"
            ));
        }

        state.stack.pop();
        state.stack.push(self.a.clone());
        state.stack.push(self.b.clone());

        Ok(())
    }

    fn redo(&self, state: &mut RpnState) -> Result<()> {
        if state.stack.len() < 2 {
            return Err(anyhow!(
                "unexpected state for redo, expected at least 2 items on the stack"
            ));
        }
        let found_a = state.stack.peek(1).unwrap();
        let found_b = state.stack.peek(0).unwrap();

        if *found_a != self.a {
            return Err(anyhow!(
                "unexpected state for redo, expected item on the stack to be the same as argument 0"
            ));
        }

        if *found_b != self.b {
            return Err(anyhow!(
                "unexpected state for redo, expected item on the stack to be the same as argument 1"
            ));
        }

        state.stack.pop_n(2)?;
        state.stack.push(self.result.clone());

        Ok(())
    }
}
