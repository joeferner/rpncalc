use crate::error::RpnCalcError;
use crate::rpn_calc::RpnCalc;
use crate::tui::console::Console;
use crate::tui::control::Control;
use crate::tui::HandleKeyEventResult;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use std::cell::RefCell;
use std::rc::Rc;

pub struct PromptInit {
    pub rpn_calc: Rc<RefCell<RpnCalc>>,
}

pub struct Prompt {
    rpn_calc: Rc<RefCell<RpnCalc>>,
    input: String,
    cursor_location: u16,
    top: u16,
    width: u16,
}

impl Prompt {
    pub fn new(init: PromptInit) -> Self {
        return Prompt {
            rpn_calc: init.rpn_calc,
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

    fn handle_tab_key_press(&mut self) -> Result<HandleKeyEventResult, RpnCalcError> {
        let str = self.input.trim();
        if str.is_empty() {
            return Ok(HandleKeyEventResult::SetMessage(None));
        }
        let completions = self.rpn_calc.borrow().get_tab_completions(str);
        if completions.is_empty() {
            return Ok(HandleKeyEventResult::SetMessage(None));
        }
        if completions.len() == 1 {
            self.input = completions.get(0).unwrap().to_string();
            self.cursor_location = self.input.len() as u16;
            return Ok(HandleKeyEventResult::SetMessage(None));
        }
        let mut completions_str = "".to_string();
        for completion in completions {
            if completions_str.len() + 1 + completion.len() > self.width as usize {
                break;
            }
            completions_str.push(' ');
            completions_str.push_str(completion.as_str());
        }
        return Ok(HandleKeyEventResult::SetMessage(Some(completions_str)));
    }

    fn handle_char_key_press(&mut self, ch: char) -> Result<HandleKeyEventResult, RpnCalcError> {
        if self.input.is_empty() && (ch == '+' || ch == '-' || ch == '*' || ch == '/' || ch == '^' || ch == '_') {
            let mut rpn_calc = self.rpn_calc.borrow_mut();
            let f = rpn_calc.functions.get(format!("{}", ch).as_str());
            if let Some(f) = f {
                if let Err(err) = f.clone().apply(&mut rpn_calc) {
                    return Ok(HandleKeyEventResult::SetMessage(Some(format!("{}", err))));
                };
                return Ok(HandleKeyEventResult::Continue);
            }
        } else {
            self.input.push(ch);
            self.cursor_location = self.input.len() as u16
        }
        return Ok(HandleKeyEventResult::SetMessage(None));
    }

    fn handle_enter_key_press(&mut self) -> Result<HandleKeyEventResult, RpnCalcError> {
        let str = self.input.trim();
        if str == "exit" || str == "quit" {
            self.clear_input();
            return Ok(HandleKeyEventResult::Exit);
        } else if str == "help" || str == "?" {
            self.clear_input();
            return Ok(HandleKeyEventResult::Help);
        } else if let Err(err) = self.rpn_calc.clone().borrow_mut().push_str(str) {
            return Ok(HandleKeyEventResult::SetMessage(Some(format!("{}", err))));
        } else {
            self.clear_input();
        }
        return Ok(HandleKeyEventResult::SetMessage(None));
    }

    fn handle_backspace_key_press(&mut self) -> Result<HandleKeyEventResult, RpnCalcError> {
        if self.input.is_empty() {
            if !self.rpn_calc.borrow().stack.items.is_empty() {
                self.rpn_calc.borrow_mut().push_str("drop")?;
            }
        } else if self.cursor_location > 0 {
            let loc = self.cursor_location as usize;
            let mut new_input = self.input[..loc - 1].to_string();
            new_input.push_str(&self.input[loc..]);
            self.input = new_input;
            self.cursor_location -= 1;
        }
        return Ok(HandleKeyEventResult::SetMessage(None));
    }
}

impl Control for Prompt {
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
        console.move_to(self.cursor_location + 1, self.top)?;
        return Ok(());
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<HandleKeyEventResult, RpnCalcError> {
        if key.kind != KeyEventKind::Press {
            return Ok(HandleKeyEventResult::Continue);
        }

        match key.code {
            KeyCode::Char(ch) => {
                return self.handle_char_key_press(ch);
            }
            KeyCode::Enter => {
                return self.handle_enter_key_press();
            }
            KeyCode::Backspace => {
                return self.handle_backspace_key_press();
            }
            KeyCode::Left => {
                if self.cursor_location > 0 {
                    self.cursor_location -= 1;
                }
            }
            KeyCode::Right => {
                if self.cursor_location < self.input.len() as u16 {
                    self.cursor_location += 1;
                }
            }
            KeyCode::Tab => {
                return self.handle_tab_key_press();
            }
            _ => {}
        }

        return Ok(HandleKeyEventResult::SetMessage(None));
    }
}
