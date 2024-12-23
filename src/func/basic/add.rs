use anyhow::Result;

use crate::{
    func::{execute_binary, Func},
    state::RpnState,
    undo_action::UndoEvent,
};

pub struct AddFunc {}

impl AddFunc {
    pub fn new() -> Self {
        Self {}
    }
}

impl Func for AddFunc {
    fn execute(&self, state: &mut RpnState) -> Result<Box<dyn UndoEvent>> {
        execute_binary(state, |a, b| a.add(b))
    }

    fn name(&self) -> &str {
        "add"
    }

    fn aliases(&self) -> Vec<&str> {
        vec!["+"]
    }

    fn description(&self) -> &str {
        "The addition (+) operator produces the sum of numeric operands or string concatenation."
    }
}

#[cfg(test)]
mod test {
    use crate::{test_binary_func, test_expr};

    #[test]
    fn test_add() {
        test_binary_func!(
            StackItem::Number(1.0, 10),
            StackItem::Number(2.0, 10),
            "add",
            StackItem::Number(1.0 + 2.0, 10)
        );
    }

    #[test]
    fn test_add_expr() {
        test_expr!("1 + 2", StackItem::Number(1.0 + 2.0, 10));
    }
}
