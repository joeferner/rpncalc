use crate::error::RpnCalcError;
use crate::rpn_calc::RpnCalc;
use crate::tui::console::Console;
use crate::tui::control::Control;
use crate::tui::HandleKeyEventResult;
use crate::units::AngleUnits;
use crossterm::event::KeyEvent;
use std::cell::RefCell;
use std::rc::Rc;

pub struct StatusInit {
    pub rpn_calc: Rc<RefCell<RpnCalc>>,
}

pub struct Status {
    rpn_calc: Rc<RefCell<RpnCalc>>,
    top: u16,
    width: u16,
}

impl Status {
    pub fn new(init: StatusInit) -> Self {
        return Status {
            rpn_calc: init.rpn_calc,
            top: 0,
            width: 0,
        };
    }
}

impl Control for Status {
    fn get_top(&self) -> u16 {
        return self.top;
    }

    fn set_top(&mut self, top: u16) -> () {
        self.top = top;
    }

    fn get_height(&self) -> u16 {
        return 1;
    }

    fn set_width(&mut self, width: u16) -> () {
        self.width = width;
    }

    fn redraw(&self, console: &mut dyn Console) -> Result<(), RpnCalcError> {
        let rpn_calc = self.rpn_calc.borrow();

        // draw mode
        let angle_mode = match rpn_calc.angle_mode {
            AngleUnits::Degrees => "DEG",
            AngleUnits::Radians => "RAD",
            AngleUnits::Gradians => "GRAD",
        };

        // draw base
        let base_str = match rpn_calc.base {
            2 => "BIN".to_string(),
            8 => "OCT".to_string(),
            10 => "DEC".to_string(),
            16 => "HEX".to_string(),
            _ => format!("BASE{}", rpn_calc.base),
        };

        let status_line = format!("{:4} {}", angle_mode, base_str);

        console.move_to(0, self.top)?;
        console.clear_current_line()?;
        console.print(status_line.as_str())?;

        return Ok(());
    }

    fn handle_key_event(&mut self, _key: KeyEvent) -> Result<HandleKeyEventResult, RpnCalcError> {
        return Ok(HandleKeyEventResult::Continue);
    }
}
