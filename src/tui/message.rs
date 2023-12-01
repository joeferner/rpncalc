use crate::error::RpnCalcError;
use crate::tui::console::Console;
use crate::tui::control::Control;
use crate::tui::HandleKeyEventResult;
use crossterm::event::KeyEvent;

pub struct Message {
    message: Option<String>,
    top: u16,
    width: u16,
}

impl Message {
    pub fn new() -> Self {
        return Message {
            message: None,
            top: 0,
            width: 10,
        };
    }

    pub fn set_message(&mut self, message: Option<String>) -> () {
        self.message = message;
    }
}

impl Control for Message {
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
        console.move_to(0, self.top)?;
        console.clear_current_line()?;
        if let Some(message) = &self.message {
            console.print(message)?;
        }
        return Ok(());
    }

    fn handle_key_event(&mut self, _key: KeyEvent) -> Result<HandleKeyEventResult, RpnCalcError> {
        return Ok(HandleKeyEventResult::Continue);
    }
}
