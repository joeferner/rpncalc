use anyhow::{anyhow, Result};

use crate::{
    state::{angle_mode::AngleMode, RpnState},
    undo_action::UndoEvent,
};

pub mod cos;
pub mod degrees;
pub mod radians;
pub mod sin;
pub mod tan;

#[derive(Debug)]
pub struct AngleUndoEvent {
    previous_mode: AngleMode,
    new_mode: AngleMode,
}

impl UndoEvent for AngleUndoEvent {
    fn undo(&self, state: &mut RpnState) -> Result<()> {
        if state.angle_mode != self.new_mode {
            return Err(anyhow!(
                "expected current mode to be {:?} but was {:?}",
                self.new_mode,
                state.angle_mode
            ));
        }
        state.angle_mode = self.previous_mode;
        Ok(())
    }

    fn redo(&self, state: &mut RpnState) -> Result<()> {
        if state.angle_mode != self.previous_mode {
            return Err(anyhow!(
                "expected current mode to be {:?} but was {:?}",
                self.previous_mode,
                state.angle_mode
            ));
        }
        state.angle_mode = self.new_mode;
        Ok(())
    }
}

mod test {
    #[cfg(test)]
    #[macro_export]
    macro_rules! test_angle_mode_func {
        ($previous_angle_mode: expr, $new_angle_mode: expr, $op: expr) => {
            use crate::state::RpnState;

            let mut state = RpnState::new().unwrap();
            state.angle_mode = $previous_angle_mode;
            state.push_str($op).unwrap();
            assert_eq!($new_angle_mode, state.angle_mode, "answer after op");

            // test undo
            state.undo().unwrap();
            assert_eq!($previous_angle_mode, state.angle_mode, "answer undo");

            // test redo
            state.redo().unwrap();
            assert_eq!($new_angle_mode, state.angle_mode, "answer redo");
        };
    }
}
