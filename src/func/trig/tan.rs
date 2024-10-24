use anyhow::Result;

use crate::{
    func::{execute_unary, Func},
    state::RpnState,
    undo_action::UndoEvent,
};

pub struct TanFunc {}

impl TanFunc {
    pub fn new() -> Self {
        Self {}
    }
}

impl Func for TanFunc {
    fn execute(&self, state: &mut RpnState) -> Result<Box<dyn UndoEvent>> {
        let angle_mode = state.angle_mode;
        execute_unary(state, |a| a.tan(angle_mode))
    }
}

#[cfg(test)]
mod test {
    use crate::{state::AngleMode, test_unary_angle_func};

    #[test]
    fn test_sin() {
        test_unary_angle_func!(
            AngleMode::Degrees,
            StackItem::Number(2.0, 10),
            "tan",
            StackItem::Number(0.03492076949174773, 10)
        );
    }
}
