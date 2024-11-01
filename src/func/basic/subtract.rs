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

    fn name(&self) -> &str {
        "subtract"
    }

    fn aliases(&self) -> Vec<&str> {
        vec!["-"]
    }

    fn description(&self) -> &str {
        "The subtraction (-) operator subtracts the two operands, producing their difference."
    }
}

#[cfg(test)]
mod test {
    use crate::{test_binary_func, test_expr};

    #[test]
    fn test_subtract() {
        test_binary_func!(
            StackItem::Number(1.0, 10),
            StackItem::Number(2.0, 10),
            "subtract",
            StackItem::Number(1.0 - 2.0, 10)
        );
    }

    #[test]
    fn test_subtract_expr() {
        test_expr!("1 - 2", StackItem::Number(1.0 - 2.0, 10));
    }
}
