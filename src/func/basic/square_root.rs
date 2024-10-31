use anyhow::Result;

use crate::{
    func::{execute_unary, Func},
    state::RpnState,
    undo_action::UndoEvent,
};

pub struct SquareRootFunc {}

impl SquareRootFunc {
    pub fn new() -> Self {
        Self {}
    }
}

impl Func for SquareRootFunc {
    fn execute(&self, state: &mut RpnState) -> Result<Box<dyn UndoEvent>> {
        execute_unary(state, |a| a.sqrt())
    }
}

#[cfg(test)]
mod test {
    use crate::{test_expr, test_unary_func};

    #[test]
    fn test_square_root() {
        test_unary_func!(
            StackItem::Number(5.0, 10),
            "sqrt",
            StackItem::Number(5.0_f64.sqrt(), 10)
        );
    }

    #[test]
    fn test_square_root_expr() {
        test_expr!("sqrt(5)", StackItem::Number(5.0_f64.sqrt(), 10));
    }
}
