use anyhow::Result;

use crate::{
    func::{execute_unary, Func},
    stack::item::StackItem,
    state::RpnState,
    undo_action::UndoEvent,
};

pub struct InverseFunc {}

impl InverseFunc {
    pub fn new() -> Self {
        Self {}
    }
}

impl Func for InverseFunc {
    fn execute(&self, state: &mut RpnState) -> Result<Box<dyn UndoEvent>> {
        execute_unary(state, |a| StackItem::Number(1.0, 10).divide(a))
    }

    fn name(&self) -> &str {
        "inv"
    }

    fn aliases(&self) -> Vec<&str> {
        vec![]
    }

    fn description(&self) -> &str {
        "The inverse operator produces the inverse (1/x) of its operand."
    }
}

#[cfg(test)]
mod test {
    use crate::{test_expr, test_unary_func};

    #[test]
    fn test_inverse() {
        test_unary_func!(
            StackItem::Number(5.0, 10),
            "inv",
            StackItem::Number(1.0 / 5.0, 10)
        );
    }

    #[test]
    fn test_inverse_by_zero() {
        test_unary_func!(StackItem::Number(0.0, 10), "inv", StackItem::Undefined);
    }

    #[test]
    fn test_inverse_expr() {
        test_expr!("inv(5)", StackItem::Number(1.0 / 5.0, 10));
    }
}
