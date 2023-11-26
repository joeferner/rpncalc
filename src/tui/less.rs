use crate::error::RpnCalcError;
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
        let lines = text_to_lines(&original_text, width as usize);
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
        let new_pos = (pos as i32).min(self.lines.len() as i32 - self.height as i32) as usize;
        if self.scroll_pos != new_pos {
            self.scroll_pos = new_pos;
            self.redraw()?;
        }
        return Ok(());
    }

    pub fn resize(&mut self, width: u16, height: u16) -> Result<(), RpnCalcError> {
        self.width = width;
        self.height = height;
        self.lines = text_to_lines(&self.original_text, self.width as usize);
        self.scroll_relative(0)?;
        return self.redraw();
    }

    pub fn redraw(&self) -> Result<(), RpnCalcError> {
        for i in 0..self.height - 1 {
            let line_index = self.scroll_pos + i as usize;
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

fn text_to_lines(text: &str, width: usize) -> Vec<String> {
    return text.split('\n').flat_map(|s| split_line(s, width)).collect();
}

fn split_line(line: &str, width: usize) -> Vec<String> {
    let mut line = line.to_string();
    let mut results: Vec<String> = Vec::new();
    while line.len() > width {
        let first_part = line.get(0..width).unwrap();
        let last_part = line.get(width..).unwrap();

        if let Some(space_parts) = first_part.rsplit_once(' ') {
            results.push(space_parts.0.to_string());
            line = format!("{}{}", space_parts.1, last_part);
        } else {
            results.push(first_part.to_string());
            line = last_part.to_string();
        }
    }
    if !line.is_empty() {
        results.push(line);
    }
    return results;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_no_split() {
        assert_eq!(vec!["0123456789"], split_line("0123456789", 10));
    }

    #[test]
    fn test_split_line_last_space() {
        assert_eq!(vec!["0123 0123", "0123"], split_line("0123 0123 0123", 10));
    }

    #[test]
    fn test_split_line_no_good_split() {
        assert_eq!(vec!["0123456789", "0"], split_line("01234567890", 10));
    }

    #[test]
    fn test_split_line_multiple_splits() {
        assert_eq!(
            vec!["0123456789", "0", "012345678", "012345678"],
            split_line("01234567890 012345678 012345678", 10)
        );
    }
}
