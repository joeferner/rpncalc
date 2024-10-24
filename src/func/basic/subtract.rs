use anyhow::Result;

use crate::{
    func::{execute_binary, Func},
    state::RpnState,
    undo_action::UndoEvent,
};

pub struct SubtractFunc {}

impl SubtractFunc {
    pub fn new() -> Self {
        Self {}
    }
}

impl Func for SubtractFunc {
    fn execute(&self, state: &mut RpnState) -> Result<Box<dyn UndoEvent>> {
        execute_binary(state, |a, b| a.subtract(b))
    }
}

#[cfg(test)]
mod test {
    use crate::test_binary_func;

    #[test]
    fn test_subtract() {
        test_binary_func!(
            StackItem::Number(1.0, 10),
            StackItem::Number(2.0, 10),
            "subtract",
            StackItem::Number(-1.0, 10)
        );
    }
}
