use anyhow::Result;

use crate::{
    state::RpnState,
    undo_action::UndoEvent,
};

use super::{execute_binary, Func};

pub struct DivideFunc {}

impl DivideFunc {
    pub fn new() -> Self {
        Self {}
    }
}

impl Func for DivideFunc {
    fn execute(&self, state: &mut RpnState) -> Result<Box<dyn UndoEvent>> {
        execute_binary(state, |a, b| a.divide(&b))
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
