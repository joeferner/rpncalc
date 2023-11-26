use crate::error::RpnCalcError;
use crate::tui::nroff::nroff_format;
use crate::tui::HandleKeyEventResult;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor, QueueableCommand};
use std::io::{stdout, Write};

pub struct Less {
    width: u16,
    height: u16,
    original_text: String,
    lines: Vec<String>,
    scroll_pos: usize,
}

impl Less {
    pub fn new(width: u16, height: u16, original_text: String) -> Self {
        let lines = format_text(&original_text, width as usize);
        return Less {
            width,
            height,
            original_text,
            lines,
            scroll_pos: 0,
        };
    }

    pub fn key_event(&mut self, key: KeyEvent) -> Result<HandleKeyEventResult, RpnCalcError> {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Up => self.scroll_relative(-1)?,
                KeyCode::PageUp => self.scroll_relative(-(self.height as i32))?,
                KeyCode::Down => self.scroll_relative(1)?,
                KeyCode::PageDown => self.scroll_relative(self.height as i32)?,
                KeyCode::Home => self.scroll_absolute(0)?,
                KeyCode::End => self.scroll_absolute(self.lines.len())?,
                KeyCode::Char(ch) => {
                    if ch == 'q' {
                        return Ok(HandleKeyEventResult::Exit);
                    }
                }
                _ => {}
            }
        }
        return Ok(HandleKeyEventResult::Continue);
    }

    pub fn scroll_relative(&mut self, amount: i32) -> Result<(), RpnCalcError> {
        let mut new_pos = self.scroll_pos as i32 + amount;
        new_pos = new_pos.min(self.lines.len() as i32 - self.height as i32);
        new_pos = new_pos.max(0);
        return self.scroll_absolute(new_pos as usize);
    }

    pub fn scroll_absolute(&mut self, pos: usize) -> Result<(), RpnCalcError> {
        let new_pos = (pos as i32).min(self.lines.len() as i32 - self.height as i32).max(0) as usize;
        if self.scroll_pos != new_pos {
            self.scroll_pos = new_pos;
            self.redraw()?;
        }
        return Ok(());
    }

    pub fn resize(&mut self, width: u16, height: u16) -> Result<(), RpnCalcError> {
        self.width = width;
        self.height = height;
        self.lines = format_text(&self.original_text, self.width as usize);
        self.scroll_relative(0)?;
        return self.redraw();
    }

    pub fn redraw(&self) -> Result<(), RpnCalcError> {
        let height = self.height.max(1) - 1;
        for i in 0..height {
            let i_usize = i as usize;
            let line_index = self.scroll_pos + i_usize;
            stdout().queue(cursor::MoveTo(0, i))?;
            stdout().queue(Clear(ClearType::CurrentLine))?;
            if line_index < self.lines.len() {
                let line = &self.lines[line_index];
                stdout().queue(cursor::MoveTo(0, i))?;
                stdout().queue(Print(line))?;
            }
        }

        stdout().queue(cursor::MoveTo(0, self.height - 1))?;
        stdout().queue(Clear(ClearType::CurrentLine))?;

        stdout().flush()?;
        return Ok(());
    }
}

fn format_text(text: &str, width: usize) -> Vec<String> {
    return nroff_format(text, width).split('\n').map(|s| s.to_string()).collect();
}
