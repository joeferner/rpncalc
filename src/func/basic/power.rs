use anyhow::Result;

use crate::{
    func::{execute_binary, Func},
    state::RpnState,
    undo_action::UndoEvent,
};

pub struct PowerFunc {}

impl PowerFunc {
    pub fn new() -> Self {
        Self {}
    }
}

impl Func for PowerFunc {
    fn execute(&self, state: &mut RpnState) -> Result<Box<dyn UndoEvent>> {
        execute_binary(state, |a, b| a.pow(b))
    }

    fn name(&self) -> &str {
        "pow"
    }

    fn aliases(&self) -> Vec<&str> {
        vec!["^"]
    }

    fn description(&self) -> &str {
        "The pow function returns the value of a base raised to a power."
    }
}

#[cfg(test)]
mod test {
    use crate::{test_binary_func, test_expr};

    #[test]
    fn test_subtract() {
        test_binary_func!(
            StackItem::Number(2.0, 10),
            StackItem::Number(3.0, 10),
            "pow",
            StackItem::Number(2.0_f64.powf(3.0), 10)
        );
    }

    #[test]
    fn test_subtract_expr() {
        test_expr!("2^3", StackItem::Number(2.0_f64.powf(3.0), 10));
    }
}
