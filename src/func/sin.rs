use anyhow::Result;

use crate::{state::RpnState, undo_action::UndoEvent};

use super::{execute_unary, Func};

pub struct SinFunc {}

impl SinFunc {
    pub fn new() -> Self {
        Self {}
    }
}

impl Func for SinFunc {
    fn execute(&self, state: &mut RpnState) -> Result<Box<dyn UndoEvent>> {
        let angle_mode = state.angle_mode;
        execute_unary(state, |a| a.sin(angle_mode))
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
            "sin",
            StackItem::Number(0.01745240643728351, 10)
        );
    }
}
