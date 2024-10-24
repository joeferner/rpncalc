use anyhow::Result;

use crate::{
    func::Func,
    state::{AngleMode, RpnState},
    undo_action::UndoEvent,
};

use super::AngleUndoEvent;

pub struct DegreesFunc {}

impl DegreesFunc {
    pub fn new() -> Self {
        Self {}
    }
}

impl Func for DegreesFunc {
    fn execute(&self, state: &mut RpnState) -> Result<Box<dyn UndoEvent>> {
        let previous_mode = state.angle_mode;
        state.angle_mode = AngleMode::Radians;
        Ok(Box::new(AngleUndoEvent {
            previous_mode,
            new_mode: AngleMode::Degrees,
        }))
    }
}

#[cfg(test)]
mod test {
    use crate::{state::AngleMode, test_angle_mode_func};

    #[test]
    fn test_sin() {
        test_angle_mode_func!(AngleMode::Radians, AngleMode::Degrees, "deg");
    }
}
