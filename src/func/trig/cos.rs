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
    use crate::{state::angle_mode::AngleMode, test_expr, test_unary_angle_func};

    #[test]
    fn test_cos() {
        test_unary_angle_func!(
            AngleMode::Degrees,
            StackItem::Number(30.0, 10),
            "cos",
            StackItem::Number((30.0_f64).to_radians().cos(), 10)
        );
    }

    #[test]
    fn test_cos_expr() {
        test_expr!(
            "cos(30)",
            StackItem::Number((30.0_f64).to_radians().cos(), 10)
        );
    }
}
