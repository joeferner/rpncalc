use anyhow::{anyhow, Result};

use crate::{
    state::RpnState,
    undo_action::{binary::BinaryFuncUndoEvent, UndoEvent},
};

use super::Func;

pub struct SubtractFunc {}

impl SubtractFunc {
    pub fn new() -> Self {
        Self {}
    }
}

impl Func for SubtractFunc {
    fn execute(&self, state: &mut RpnState) -> Result<Box<dyn UndoEvent>> {
        if state.stack.len() < 2 {
            return Err(anyhow!("Not enough arguments"));
        }
        let a = state.stack.peek(1).unwrap().clone();
        let b = state.stack.peek(0).unwrap().clone();
        let result = a.subtract(&b)?;
        state.stack.pop_n(2)?;
        state.stack.push(result.clone());
        Ok(Box::new(BinaryFuncUndoEvent::new(a, b, result)))
    }
}

#[cfg(test)]
mod test {
    use crate::test_binary_func;

    #[test]
    fn test_subtract() {
        test_binary_func!(
            StackItem::Number(1.0),
            StackItem::Number(2.0),
            "subtract",
            StackItem::Number(-1.0)
        );
    }
}
