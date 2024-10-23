use anyhow::{anyhow, Result};

use crate::{
    state::RpnState,
    undo_action::{binary::BinaryFuncUndoEvent, UndoEvent},
};

use super::Func;

pub struct DivideFunc {}

impl DivideFunc {
    pub fn new() -> Self {
        Self {}
    }
}

impl Func for DivideFunc {
    fn execute(&self, state: &mut RpnState) -> Result<Box<dyn UndoEvent>> {
        if state.stack.len() < 2 {
            return Err(anyhow!("Not enough arguments"));
        }
        let a = state.stack.peek(1).unwrap().clone();
        let b = state.stack.peek(0).unwrap().clone();
        let result = a.divide(&b)?;
        state.stack.pop_n(2)?;
        state.stack.push(result.clone());
        Ok(Box::new(BinaryFuncUndoEvent::new(a, b, result)))
    }
}

#[cfg(test)]
mod test {
    use crate::test_binary_func;

    #[test]
    fn test_divide() {
        test_binary_func!(
            StackItem::Number(1.0),
            StackItem::Number(2.0),
            "divide",
            StackItem::Number(0.5)
        );
    }

    #[test]
    fn test_divide_by_zero() {
        test_binary_func!(
            StackItem::Number(1.0),
            StackItem::Number(0.0),
            "divide",
            StackItem::Undefined
        );
    }
}
