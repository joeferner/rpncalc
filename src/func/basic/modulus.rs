use anyhow::Result;

use crate::{
    func::{execute_binary, Func},
    state::RpnState,
    undo_action::UndoEvent,
};

pub struct ModulusFunc {}

impl ModulusFunc {
    pub fn new() -> Self {
        Self {}
    }
}

impl Func for ModulusFunc {
    fn execute(&self, state: &mut RpnState) -> Result<Box<dyn UndoEvent>> {
        execute_binary(state, |a, b| a.modulus(b))
    }
}

#[cfg(test)]
mod test {
    use crate::{test_binary_func, test_expr};

    #[test]
    fn test_modulus() {
        test_binary_func!(
            StackItem::Number(42.0, 10),
            StackItem::Number(8.0, 10),
            "mod",
            StackItem::Number(42.0 % 8.0, 10)
        );
    }

    #[test]
    fn test_modulus_by_zero() {
        test_binary_func!(
            StackItem::Number(1.0, 10),
            StackItem::Number(0.0, 10),
            "mod",
            StackItem::Undefined
        );
    }

    #[test]
    fn test_modulus_expr() {
        test_expr!("42 % 8", StackItem::Number(42.0 % 8.0, 10));
    }
}
