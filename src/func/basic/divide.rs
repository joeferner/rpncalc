use anyhow::Result;

use crate::{
    func::{execute_binary, Func},
    state::RpnState,
    undo_action::UndoEvent,
};

pub struct DivideFunc {}

impl DivideFunc {
    pub fn new() -> Self {
        Self {}
    }
}

impl Func for DivideFunc {
    fn execute(&self, state: &mut RpnState) -> Result<Box<dyn UndoEvent>> {
        execute_binary(state, |a, b| a.divide(b))
    }
}

#[cfg(test)]
mod test {
    use crate::{test_binary_func, test_expr};

    #[test]
    fn test_divide() {
        test_binary_func!(
            StackItem::Number(1.0, 10),
            StackItem::Number(2.0, 10),
            "divide",
            StackItem::Number(1.0 / 2.0, 10)
        );
    }

    #[test]
    fn test_divide_by_zero() {
        test_binary_func!(
            StackItem::Number(1.0, 10),
            StackItem::Number(0.0, 10),
            "divide",
            StackItem::Undefined
        );
    }

    #[test]
    fn test_divide_expr() {
        test_expr!("1 / 2", StackItem::Number(1.0 / 2.0, 10));
    }
}
