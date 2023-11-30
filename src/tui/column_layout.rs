use crate::error::RpnCalcError;
use crate::tui::console::Console;
use crate::tui::control::Control;
use crate::tui::HandleKeyEventResult;
use crossterm::event::KeyEvent;
use std::cell::RefCell;
use std::rc::Rc;

pub struct ColumnLayoutInit {
    pub top: u16,
    pub controls: Vec<Rc<RefCell<dyn Control>>>,
}

pub struct ColumnLayout {
    top: u16,
    controls: Vec<Rc<RefCell<dyn Control>>>,
    focused_control_index: usize,
}

impl ColumnLayout {
    pub fn new(init: ColumnLayoutInit) -> Self {
        return ColumnLayout {
            top: init.top,
            controls: init.controls,
            focused_control_index: 0,
        };
    }

    pub fn set_focused_control_index(&mut self, idx: usize) -> () {
        self.focused_control_index = idx;
    }
}

impl Control for ColumnLayout {
    fn get_top(&self) -> u16 {
        return self.top;
    }

    fn set_top(&mut self, top: u16) -> () {
        self.top = top;
        let mut y = top;
        for c in &self.controls {
            let mut c = c.borrow_mut();
            c.set_top(y);
            y += c.get_height();
        }
    }

    fn get_height(&self) -> u16 {
        return self.controls.iter().map(|c| c.borrow().get_height()).sum();
    }

    fn set_width(&mut self, width: u16) -> () {
        for c in &self.controls {
            c.borrow_mut().set_width(width);
        }
    }

    fn redraw(&self, console: &mut dyn Console) -> Result<(), RpnCalcError> {
        for c in &self.controls {
            c.borrow().redraw(console)?;
        }
        return Ok(());
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<HandleKeyEventResult, RpnCalcError> {
        if let Some(c) = self.controls.get(self.focused_control_index) {
            let mut c = c.borrow_mut();
            return c.handle_key_event(key);
        }
        return Ok(HandleKeyEventResult::Continue);
    }
}
