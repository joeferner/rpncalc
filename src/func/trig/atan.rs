use anyhow::Result;

use crate::{
    func::{execute_unary, Func},
    state::RpnState,
    undo_action::UndoEvent,
};

pub struct ATanFunc {}

impl ATanFunc {
    pub fn new() -> Self {
        Self {}
    }
}

impl Func for ATanFunc {
    fn execute(&self, state: &mut RpnState) -> Result<Box<dyn UndoEvent>> {
        let angle_mode = state.angle_mode;
        execute_unary(state, |a| a.atan(angle_mode))
    }
}

#[cfg(test)]
mod test {
    use crate::{state::angle_mode::AngleMode, test_expr, test_unary_angle_func};

    #[test]
    fn test_atan() {
        test_unary_angle_func!(
            AngleMode::Radians,
            StackItem::Number(0.8660254037844386, 10),
            "atan",
            StackItem::Number(0.7137243789447656, 10)
        );
        test_unary_angle_func!(
            AngleMode::Degrees,
            StackItem::Number(0.8660254037844386, 10),
            "atan",
            StackItem::Number(40.89339464913091, 10)
        );
    }

    #[test]
    fn test_atan_expr() {
        test_expr!(
            "atan(0.8660254037844386)",
            StackItem::Number(40.89339464913091, 10)
        );
    }
}
