use anyhow::Result;

use crate::{
    func::{execute_unary, Func},
    state::RpnState,
    undo_action::UndoEvent,
};

pub struct ACosFunc {}

impl ACosFunc {
    pub fn new() -> Self {
        Self {}
    }
}

impl Func for ACosFunc {
    fn execute(&self, state: &mut RpnState) -> Result<Box<dyn UndoEvent>> {
        let angle_mode = state.angle_mode;
        execute_unary(state, |a| a.acos(angle_mode))
    }
}

#[cfg(test)]
mod test {
    use crate::{state::angle_mode::AngleMode, test_expr, test_unary_angle_func};

    #[test]
    fn test_acos() {
        test_unary_angle_func!(
            AngleMode::Radians,
            StackItem::Number(0.8660254037844386, 10),
            "acos",
            StackItem::Number(0.5235987755982989, 10)
        );
        test_unary_angle_func!(
            AngleMode::Degrees,
            StackItem::Number(0.8660254037844386, 10),
            "acos",
            StackItem::Number(30.000000000000004, 10)
        );
    }

    #[test]
    fn test_acos_expr() {
        test_expr!(
            "acos(0.8660254037844386)",
            StackItem::Number(30.000000000000004, 10)
        );
    }
}
