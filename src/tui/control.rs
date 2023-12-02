use crate::error::RpnCalcError;
use crate::tui::console::Console;
use crate::tui::HandleKeyEventResult;
use crossterm::event::KeyEvent;

pub trait Control {
    fn get_top(&self) -> u16;
    fn set_top(&mut self, top: u16) -> ();
    fn get_height(&self) -> u16;
    fn set_width(&mut self, width: u16) -> ();

    fn redraw(&self, console: &mut dyn Console) -> Result<(), RpnCalcError>;

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<HandleKeyEventResult, RpnCalcError>;
}

#[cfg(test)]
pub mod tests {
    use crate::error::RpnCalcError;
    use crate::tui::console::Console;
    use crate::tui::control::Control;
    use crate::tui::HandleKeyEventResult;
    use crossterm::event::KeyEvent;

    pub struct MockControl {
        pub top: u16,
        pub height: u16,
        pub width: u16,
        pub key_events: Vec<KeyEvent>,
    }

    impl MockControl {
        pub fn new() -> Self {
            return MockControl {
                top: 0,
                height: 5,
                width: 10,
                key_events: Vec::new(),
            };
        }
    }

    impl Control for MockControl {
        fn get_top(&self) -> u16 {
            return self.top;
        }

        fn set_top(&mut self, top: u16) -> () {
            self.top = top;
        }

        fn get_height(&self) -> u16 {
            return self.height;
        }

        fn set_width(&mut self, width: u16) -> () {
            self.width = width;
        }

        fn redraw(&self, console: &mut dyn Console) -> Result<(), RpnCalcError> {
            console.move_to(0, self.top + self.height - 1)?;
            console.print("c")?;

            console.move_to(self.width, self.top + self.height - 1)?;
            console.print("d")?;

            console.move_to(0, self.top)?;
            console.print("a")?;

            console.move_to(self.width, self.top)?;
            console.print("b")?;

            return Ok(());
        }

        fn handle_key_event(&mut self, key: KeyEvent) -> Result<HandleKeyEventResult, RpnCalcError> {
            self.key_events.push(key);
            return Ok(HandleKeyEventResult::Continue);
        }
    }
}
