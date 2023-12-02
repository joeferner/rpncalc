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

#[cfg(test)]
pub mod tests {
    use crate::error::RpnCalcError;
    use crate::tui::console::Console;
    use std::mem;

    pub struct MockConsole {
        left: u16,
        top: u16,
        width: u16,
        height: u16,
        screen: Vec<Vec<char>>,
    }

    impl MockConsole {
        pub fn new(width: u16, height: u16) -> Self {
            let mut screen = Vec::new();
            for _y in 0..height {
                let mut row = Vec::new();
                for _x in 0..width {
                    row.push(' ');
                }
                screen.push(row);
            }
            return MockConsole {
                left: 5,
                top: 8,
                width,
                height,
                screen,
            };
        }

        pub fn get_ch(&self, x: u16, y: u16) -> Option<char> {
            if let Some(row) = self.screen.get(y as usize) {
                let ch = row.get(x as usize);
                return ch.map(|ch| *ch);
            }
            return None;
        }
    }

    impl Console for MockConsole {
        fn move_to(&mut self, left: u16, top: u16) -> Result<(), RpnCalcError> {
            self.left = left;
            self.top = top;
            return Ok(());
        }

        fn clear_current_line(&mut self) -> Result<(), RpnCalcError> {
            if let Some(row) = self.screen.get_mut(self.top as usize) {
                row.clear();
                for _x in 0..self.width {
                    row.push(' ');
                }
            }
            return Ok(());
        }

        fn print(&mut self, s: &str) -> Result<(), RpnCalcError> {
            if self.top >= self.height {
                return Ok(());
            }
            for ch in s.chars() {
                if let Some(row) = self.screen.get_mut(self.top as usize) {
                    if self.left < self.width {
                        let _ = mem::replace(&mut row[self.left as usize], ch);
                    }
                    self.left += 1;
                    if self.left >= self.width {
                        self.left = 0;
                        self.top += 1;
                    }
                }
            }
            return Ok(());
        }

        fn enter_alternate_screen(&mut self) -> Result<(), RpnCalcError> {
            todo!()
        }
    }
}
