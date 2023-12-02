use crate::error::RpnCalcError;
use crate::tui::console::Console;
use crate::tui::control::Control;
use crate::tui::HandleKeyEventResult;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

pub struct TextBox {
    input: String,
    cursor_location: usize,
    top: u16,
    width: u16,
}

impl TextBox {
    pub fn new() -> Self {
        return TextBox {
            input: "".to_string(),
            cursor_location: 0,
            top: 0,
            width: 0,
        };
    }

    pub fn clear_input(&mut self) -> () {
        self.cursor_location = 0;
        self.input = "".to_string();
    }

    pub fn get_value(&self) -> String {
        return self.input.to_string();
    }

    pub fn set_value(&mut self, value: &str) -> () {
        self.input = value.to_string();
        self.cursor_location = self.input.len();
    }

    fn handle_char_key_press(&mut self, ch: char, key: KeyEvent) -> Result<HandleKeyEventResult, RpnCalcError> {
        if key.modifiers == KeyModifiers::NONE || key.modifiers == KeyModifiers::SHIFT {
            let (before, after) = self.input.split_at(self.cursor_location);
            self.input = format!("{}{}{}", before, ch, after);
            self.cursor_location += 1;
            return Ok(HandleKeyEventResult::SetMessage(None));
        } else {
            return Ok(HandleKeyEventResult::SetMessage(None));
        }
    }

    fn handle_backspace_key_press(&mut self, key: KeyEvent) -> Result<HandleKeyEventResult, RpnCalcError> {
        if key.modifiers != KeyModifiers::NONE {
            return Ok(HandleKeyEventResult::SetMessage(None));
        }

        if self.cursor_location > 0 {
            let loc = self.cursor_location;
            let mut new_input = self.input[..loc - 1].to_string();
            new_input.push_str(&self.input[loc..]);
            self.input = new_input;
            self.cursor_location -= 1;
        }
        return Ok(HandleKeyEventResult::SetMessage(None));
    }

    pub fn get_width(&self) -> u16 {
        return self.width;
    }
}

impl Control for TextBox {
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
        let prompt = format!(">{}", self.input);
        console.move_to(0, self.top)?;
        console.clear_current_line()?;
        console.print(prompt.as_str())?;
        console.move_to(self.cursor_location as u16 + 1, self.top)?;
        return Ok(());
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<HandleKeyEventResult, RpnCalcError> {
        if key.kind != KeyEventKind::Press {
            return Ok(HandleKeyEventResult::Continue);
        }

        match key.code {
            KeyCode::Char(ch) => {
                return self.handle_char_key_press(ch, key);
            }
            KeyCode::Backspace => {
                return self.handle_backspace_key_press(key);
            }
            KeyCode::Left => {
                if key.modifiers == KeyModifiers::NONE && self.cursor_location > 0 {
                    self.cursor_location -= 1;
                }
            }
            KeyCode::Right => {
                if key.modifiers == KeyModifiers::NONE && self.cursor_location < self.input.len() {
                    self.cursor_location += 1;
                }
            }
            KeyCode::Home => {
                if key.modifiers == KeyModifiers::NONE {
                    self.cursor_location = 0;
                }
            }
            KeyCode::End => {
                if key.modifiers == KeyModifiers::NONE {
                    self.cursor_location = self.input.len();
                }
            }
            _ => {}
        }

        return Ok(HandleKeyEventResult::SetMessage(None));
    }
}
