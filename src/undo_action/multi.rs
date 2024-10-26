use anyhow::Result;

use crate::state::RpnState;

use super::UndoEvent;

#[derive(Debug)]
pub struct MultiUndoEvent {
    undos: Vec<Box<dyn UndoEvent>>,
}

impl MultiUndoEvent {
    pub fn new(undos: Vec<Box<dyn UndoEvent>>) -> Self {
        Self { undos }
    }
}

impl UndoEvent for MultiUndoEvent {
    fn undo(&self, state: &mut RpnState) -> Result<()> {
        for undo in self.undos.iter().rev() {
            undo.undo(state)?;
        }
        Ok(())
    }

    fn redo(&self, state: &mut RpnState) -> Result<()> {
        for undo in self.undos.iter() {
            undo.redo(state)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{stack::item::StackItem, state::RpnState};

    #[test]
    pub fn test_multi_undo() {
        let mut state = RpnState::new().unwrap();
        state.push_str("2 + 3 * 4").unwrap();
        assert_eq!(
            &StackItem::Number(2.0 + 3.0 * 4.0, 10),
            state.stack.peek(0).unwrap()
        );

        // undo
        state.undo().unwrap();
        assert_eq!(0, state.stack.len());

        // redo
        state.redo().unwrap();
        assert_eq!(
            &StackItem::Number(2.0 + 3.0 * 4.0, 10),
            state.stack.peek(0).unwrap()
        );
    }
}
