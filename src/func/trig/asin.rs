use anyhow::Result;

use crate::{
    func::{execute_unary, Func},
    state::RpnState,
    undo_action::UndoEvent,
};

pub struct ASinFunc {}

impl ASinFunc {
    pub fn new() -> Self {
        Self {}
    }
}

impl Func for ASinFunc {
    fn execute(&self, state: &mut RpnState) -> Result<Box<dyn UndoEvent>> {
        let angle_mode = state.angle_mode;
        execute_unary(state, |a| a.asin(angle_mode))
    }

    fn name(&self) -> &str {
        "asin"
    }

    fn aliases(&self) -> Vec<&str> {
        vec![]
    }

    fn description(&self) -> &str {
        "The asin function returns the inverse sine (in the current angle mode) of a number."
    }
}

#[cfg(test)]
mod test {
    use crate::{state::angle_mode::AngleMode, test_expr, test_unary_angle_func};

    #[test]
    fn test_asin() {
        test_unary_angle_func!(
            AngleMode::Radians,
            StackItem::Number(0.8660254037844386, 10),
            "asin",
            StackItem::Number(1.0471975511965976, 10)
        );
        test_unary_angle_func!(
            AngleMode::Degrees,
            StackItem::Number(0.8660254037844386, 10),
            "asin",
            StackItem::Number(59.99999999999999, 10)
        );
    }

    #[test]
    fn test_asin_expr() {
        test_expr!(
            "asin(0.8660254037844386)",
            StackItem::Number(59.99999999999999, 10)
        );
    }
}
