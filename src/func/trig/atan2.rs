use anyhow::Result;

use crate::{
    func::{execute_binary, Func},
    state::RpnState,
    undo_action::UndoEvent,
};

pub struct ATan2Func {}

impl ATan2Func {
    pub fn new() -> Self {
        Self {}
    }
}

impl Func for ATan2Func {
    fn execute(&self, state: &mut RpnState) -> Result<Box<dyn UndoEvent>> {
        let angle_mode = state.angle_mode;
        execute_binary(state, |a, b| a.atan2(b, angle_mode))
    }
}

#[cfg(test)]
mod test {
    use crate::{state::angle_mode::AngleMode, test_binary_angle_func, test_expr};

    #[test]
    fn test_atan2() {
        test_binary_angle_func!(
            AngleMode::Radians,
            StackItem::Number(1.0, 10),
            StackItem::Number(2.0, 10),
            "atan2",
            StackItem::Number(0.4636476090008061, 10)
        );
        test_binary_angle_func!(
            AngleMode::Degrees,
            StackItem::Number(1.0, 10),
            StackItem::Number(2.0, 10),
            "atan2",
            StackItem::Number(26.56505117707799, 10)
        );
    }

    #[test]
    fn test_atan2_expr() {
        test_expr!("atan2(1, 2)", StackItem::Number(26.56505117707799, 10));
    }
}
