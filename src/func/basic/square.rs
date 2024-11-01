use anyhow::Result;

use crate::{
    func::{execute_unary, Func},
    stack::item::StackItem,
    state::RpnState,
    undo_action::UndoEvent,
};

pub struct SquareFunc {}

impl SquareFunc {
    pub fn new() -> Self {
        Self {}
    }
}

impl Func for SquareFunc {
    fn execute(&self, state: &mut RpnState) -> Result<Box<dyn UndoEvent>> {
        execute_unary(state, |a| a.pow(&StackItem::Number(2.0, 10)))
    }

    fn name(&self) -> &str {
        "sq"
    }

    fn aliases(&self) -> Vec<&str> {
        vec![]
    }

    fn description(&self) -> &str {
        "The square (sq) function returns the square of a number (x^2)."
    }
}

#[cfg(test)]
mod test {
    use crate::{test_expr, test_unary_func};

    #[test]
    fn test_square() {
        test_unary_func!(
            StackItem::Number(5.0, 10),
            "sq",
            StackItem::Number(5.0_f64.powf(2.0), 10)
        );
    }

    #[test]
    fn test_square_expr() {
        test_expr!("sq(5)", StackItem::Number(5.0_f64.powf(2.0), 10));
    }
}
