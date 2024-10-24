use anyhow::Result;

use crate::{
    func::{execute_unary, Func},
    state::RpnState,
    undo_action::UndoEvent,
};

pub struct CosFunc {}

impl CosFunc {
    pub fn new() -> Self {
        Self {}
    }
}

impl Func for CosFunc {
    fn execute(&self, state: &mut RpnState) -> Result<Box<dyn UndoEvent>> {
        let angle_mode = state.angle_mode;
        execute_unary(state, |a| a.cos(angle_mode))
    }
}

#[cfg(test)]
mod test {
    use crate::{state::AngleMode, test_unary_angle_func};

    #[test]
    fn test_sin() {
        test_unary_angle_func!(
            AngleMode::Degrees,
            StackItem::Number(1.0, 10),
            "cos",
            StackItem::Number(0.9998476951563913, 10)
        );
    }
}
