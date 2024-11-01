use anyhow::Result;

use crate::{
    func::Func,
    state::{angle_mode::AngleMode, RpnState},
    undo_action::UndoEvent,
};

use super::AngleUndoEvent;

pub struct RadiansFunc {}

impl RadiansFunc {
    pub fn new() -> Self {
        Self {}
    }
}

impl Func for RadiansFunc {
    fn execute(&self, state: &mut RpnState) -> Result<Box<dyn UndoEvent>> {
        let previous_mode = state.angle_mode;
        state.angle_mode = AngleMode::Radians;
        Ok(Box::new(AngleUndoEvent {
            previous_mode,
            new_mode: AngleMode::Radians,
        }))
    }

    fn name(&self) -> &str {
        "rad"
    }

    fn aliases(&self) -> Vec<&str> {
        vec![]
    }

    fn description(&self) -> &str {
        "The rad function sets the current angle mode to radians"
    }
}

#[cfg(test)]
mod test {
    use crate::{state::angle_mode::AngleMode, test_angle_mode_func};

    #[test]
    fn test_radians() {
        test_angle_mode_func!(AngleMode::Degrees, AngleMode::Radians, "rad");
    }
}
