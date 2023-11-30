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
