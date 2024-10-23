use anyhow::anyhow;

use crate::stack::item::StackItem;

use super::UndoEvent;

#[derive(Debug)]
pub struct PopUndoEvent {
    stack_item: StackItem,
}

impl PopUndoEvent {
    pub fn new(stack_item: StackItem) -> Self {
        Self { stack_item }
    }
}

impl UndoEvent for PopUndoEvent {
    fn undo(&self, state: &mut crate::state::RpnState) -> anyhow::Result<()> {
        state.stack.push(self.stack_item.clone());
        Ok(())
    }

    fn redo(&self, state: &mut crate::state::RpnState) -> anyhow::Result<()> {
        if state.stack.len() < 1 {
            return Err(anyhow!(
                "unexpected state for redo, expected item on the stack but found none"
            ));
        }
        let found_result = state.stack.peek(0).unwrap();
        if *found_result != self.stack_item {
            return Err(anyhow!(
                "unexpected state for redo, expected item on the stack to be the same as the result"
            ));
        }

        state.stack.pop();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{stack::item::StackItem, state::RpnState};

    #[test]
    fn test_pop_undo_redo() {
        let mut state = RpnState::new();
        state.push_str("1").unwrap();
        state.pop().unwrap();
        assert_eq!(state.stack.len(), 0);

        // test undo
        state.undo().unwrap();
        assert_eq!(state.stack.len(), 1);
        let answer = state.stack.peek(0).unwrap();
        assert_eq!(*answer, StackItem::Number { value: 1.0 });

        // test redo
        state.redo().unwrap();
        assert_eq!(state.stack.len(), 0);
    }
}
