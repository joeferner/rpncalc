use anyhow::{anyhow, Result};

use crate::{stack::item::StackItem, state::RpnState};

use super::UndoEvent;

#[derive(Debug)]
pub struct UnaryFuncUndoEvent {
    a: StackItem,
    result: StackItem,
}

impl UnaryFuncUndoEvent {
    pub fn new(a: StackItem, result: StackItem) -> Self {
        Self { a, result }
    }
}

impl UndoEvent for UnaryFuncUndoEvent {
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

        Ok(())
    }

    fn redo(&self, state: &mut RpnState) -> Result<()> {
        if state.stack.len() < 1 {
            return Err(anyhow!(
                "unexpected state for redo, expected at least 1 items on the stack"
            ));
        }
        let found_a = state.stack.peek(0).unwrap();

        if *found_a != self.a {
            return Err(anyhow!(
                "unexpected state for redo, expected item on the stack to be the same as argument 0"
            ));
        }

        state.stack.pop_n(1)?;
        state.stack.push(self.result.clone());

        Ok(())
    }
}
