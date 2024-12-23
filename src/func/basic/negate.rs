use anyhow::Result;

use crate::{
    func::{execute_unary, Func},
    state::RpnState,
    undo_action::UndoEvent,
};

pub struct NegateFunc {}

impl NegateFunc {
    pub fn new() -> Self {
        Self {}
    }
}

impl Func for NegateFunc {
    fn execute(&self, state: &mut RpnState) -> Result<Box<dyn UndoEvent>> {
        execute_unary(state, |a| a.negate())
    }

    fn name(&self) -> &str {
        "neg"
    }

    fn aliases(&self) -> Vec<&str> {
        vec![]
    }

    fn description(&self) -> &str {
        "The unary negation operator produces the negative of its operand."
    }
}

#[cfg(test)]
mod test {
    use crate::test_expr;

    #[test]
    fn test_negate_expr() {
        test_expr!("-30", StackItem::Number(-30.0, 10));
    }
}
