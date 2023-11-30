use crate::error::RpnCalcError;
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen};
use crossterm::{cursor, execute, QueueableCommand};
use std::io::stdout;

pub trait Console {
    fn move_to(&mut self, left: u16, top: u16) -> Result<(), RpnCalcError>;

    fn clear_current_line(&mut self) -> Result<(), RpnCalcError>;

    fn print(&mut self, s: &str) -> Result<(), RpnCalcError>;

    fn enter_alternate_screen(&mut self) -> Result<(), RpnCalcError>;
}

pub struct CrosstermConsole {}

impl CrosstermConsole {
    pub fn new() -> Self {
        return CrosstermConsole {};
    }
}

impl Console for CrosstermConsole {
    fn move_to(&mut self, left: u16, top: u16) -> Result<(), RpnCalcError> {
        stdout().queue(cursor::MoveTo(left, top))?;
        return Ok(());
    }

    fn clear_current_line(&mut self) -> Result<(), RpnCalcError> {
        stdout().queue(Clear(ClearType::CurrentLine))?;
        return Ok(());
    }

    fn print(&mut self, s: &str) -> Result<(), RpnCalcError> {
        stdout().queue(Print(s))?;
        return Ok(());
    }

    fn enter_alternate_screen(&mut self) -> Result<(), RpnCalcError> {
        execute!(stdout(), EnterAlternateScreen)?;
        return Ok(());
    }
}
