use anyhow::Result;

use crate::{
    state::RpnState,
    undo_action::UndoEvent,
};

use super::{execute_binary, Func};

pub struct MultiplyFunc {}

impl MultiplyFunc {
    pub fn new() -> Self {
        Self {}
    }
}

impl Func for MultiplyFunc {
    fn execute(&self, state: &mut RpnState) -> Result<Box<dyn UndoEvent>> {
        execute_binary(state, |a, b| a.multiply(&b))
    }
}

#[cfg(test)]
mod test {
    use crate::test_binary_func;

    #[test]
    fn test_multiply() {
        test_binary_func!(
            StackItem::Number(2.0),
            StackItem::Number(3.0),
            "multiply",
            StackItem::Number(6.0)
        );
    }
}
