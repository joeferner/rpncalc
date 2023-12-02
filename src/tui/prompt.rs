use crate::error::RpnCalcError;
use crate::rpn_calc::RpnCalc;
use crate::tui::console::Console;
use crate::tui::control::Control;
use crate::tui::text_box::TextBox;
use crate::tui::HandleKeyEventResult;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::cell::RefCell;
use std::rc::Rc;

pub struct PromptInit {
    pub rpn_calc: Rc<RefCell<RpnCalc>>,
}

pub struct Prompt {
    rpn_calc: Rc<RefCell<RpnCalc>>,
    text_box: TextBox,
}

impl Prompt {
    pub fn new(init: PromptInit) -> Self {
        return Prompt {
            rpn_calc: init.rpn_calc,
            text_box: TextBox::new(),
        };
    }

    fn handle_tab_key_press(&mut self, key: KeyEvent) -> Result<HandleKeyEventResult, RpnCalcError> {
        if key.modifiers != KeyModifiers::NONE {
            return Ok(HandleKeyEventResult::SetMessage(None));
        }

        let str = self.text_box.get_value();
        let str = str.trim();
        if str.is_empty() {
            return Ok(HandleKeyEventResult::SetMessage(None));
        }
        let completions = self.rpn_calc.borrow().get_tab_completions(str);
        if completions.is_empty() {
            return Ok(HandleKeyEventResult::SetMessage(None));
        }
        if completions.len() == 1 {
            self.text_box.set_value(completions.get(0).unwrap());
            return Ok(HandleKeyEventResult::SetMessage(None));
        }
        let mut completions_str = "".to_string();
        for completion in completions {
            if completions_str.len() + 1 + completion.len() > self.text_box.get_width() as usize {
                break;
            }
            completions_str.push(' ');
            completions_str.push_str(completion.as_str());
        }
        return Ok(HandleKeyEventResult::SetMessage(Some(completions_str)));
    }

    fn handle_char_key_press(&mut self, ch: char, key: KeyEvent) -> Result<HandleKeyEventResult, RpnCalcError> {
        if key.modifiers == KeyModifiers::NONE
            && self.text_box.get_value().is_empty()
            && (ch == '+' || ch == '-' || ch == '*' || ch == '/' || ch == '^' || ch == '_')
        {
            let mut rpn_calc = self.rpn_calc.borrow_mut();
            let f = rpn_calc.functions.get(format!("{}", ch).as_str());
            if let Some(f) = f {
                if let Err(err) = f.clone().apply(&mut rpn_calc) {
                    return Ok(HandleKeyEventResult::SetMessage(Some(format!("{}", err))));
                };
                return Ok(HandleKeyEventResult::Continue);
            }
        } else {
            self.text_box.handle_key_event(key)?;
        }
        return Ok(HandleKeyEventResult::SetMessage(None));
    }

    fn handle_enter_key_press(&mut self, key: KeyEvent) -> Result<HandleKeyEventResult, RpnCalcError> {
        if key.modifiers != KeyModifiers::NONE {
            return Ok(HandleKeyEventResult::SetMessage(None));
        }

        let str = self.text_box.get_value();
        let str = str.trim();
        if str == "exit" || str == "quit" {
            self.text_box.clear_input();
            return Ok(HandleKeyEventResult::Exit);
        } else if str == "help" || str == "?" {
            self.text_box.clear_input();
            return Ok(HandleKeyEventResult::Help);
        } else if let Err(err) = self.rpn_calc.clone().borrow_mut().push_str(str) {
            return Ok(HandleKeyEventResult::SetMessage(Some(format!("{}", err))));
        } else {
            self.text_box.clear_input();
        }
        return Ok(HandleKeyEventResult::SetMessage(None));
    }

    fn handle_backspace_key_press(&mut self, key: KeyEvent) -> Result<HandleKeyEventResult, RpnCalcError> {
        if self.text_box.get_value().is_empty() {
            if key.modifiers == KeyModifiers::NONE {
                if let Err(err) = self.rpn_calc.borrow_mut().push_str("drop") {
                    return Ok(HandleKeyEventResult::SetMessage(Some(format!("{}", err))));
                }
            }
            return Ok(HandleKeyEventResult::SetMessage(None));
        } else {
            return self.text_box.handle_key_event(key);
        }
    }
}

impl Control for Prompt {
    fn get_top(&self) -> u16 {
        return self.text_box.get_top();
    }

    fn set_top(&mut self, top: u16) -> () {
        self.text_box.set_top(top);
    }

    fn get_height(&self) -> u16 {
        return self.text_box.get_height();
    }

    fn set_width(&mut self, width: u16) -> () {
        self.text_box.set_width(width);
    }

    fn redraw(&self, console: &mut dyn Console) -> Result<(), RpnCalcError> {
        return self.text_box.redraw(console);
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<HandleKeyEventResult, RpnCalcError> {
        if key.kind != KeyEventKind::Press {
            return Ok(HandleKeyEventResult::Continue);
        }

        return match key.code {
            KeyCode::Char(ch) => self.handle_char_key_press(ch, key),
            KeyCode::Enter => self.handle_enter_key_press(key),
            KeyCode::Backspace => self.handle_backspace_key_press(key),
            KeyCode::Tab => self.handle_tab_key_press(key),
            _ => self.text_box.handle_key_event(key),
        };
    }
}
