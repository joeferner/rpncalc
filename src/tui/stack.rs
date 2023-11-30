use crate::error::RpnCalcError;
use crate::rpn_calc::RpnCalc;
use crate::stack_item::StackItem;
use crate::tui::console::Console;
use crate::tui::control::Control;
use crate::tui::HandleKeyEventResult;
use crossterm::event::KeyEvent;
use std::cell::RefCell;
use std::rc::Rc;

pub struct StackInit {
    pub rpn_calc: Rc<RefCell<RpnCalc>>,
    pub width: u16,
    pub height: u16,
}

pub struct Stack {
    rpn_calc: Rc<RefCell<RpnCalc>>,
    top: u16,
    width: u16,
    height: u16,
}

impl Stack {
    pub fn new(init: StackInit) -> Self {
        return Stack {
            rpn_calc: init.rpn_calc,
            top: 0,
            width: init.width,
            height: init.height,
        };
    }

    pub fn set_height(&mut self, height: u16) -> () {
        self.height = height;
    }

    fn format_stack_item(&self, display_stack_index: usize, stack_item: &StackItem) -> String {
        let prefix = format!("{}:", display_stack_index);
        let width = self.width as usize - prefix.len();
        let stack_item_str = stack_item.to_string_format(width, self.rpn_calc.borrow().base);
        let s = format!("{: >width$}", stack_item_str, width = width);
        return format!("{}{}", prefix, s);
    }
}

impl Control for Stack {
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
        let rpn_calc = self.rpn_calc.borrow();

        for i in 0..self.height {
            let stack_offset = (self.height - i) as usize;
            let stack_index = rpn_calc.stack.items.len() as i16 - stack_offset as i16;
            console.move_to(0, self.top + i)?;
            console.clear_current_line()?;
            let mut stack_item: Option<&StackItem> = None;
            if stack_index >= 0 {
                stack_item = rpn_calc.stack.items.get(stack_index as usize);
            }
            let stack_item_str = match stack_item {
                Some(stack_item) => self.format_stack_item(stack_offset, stack_item),
                None => format!("{}:", stack_offset),
            };
            console.print(stack_item_str.as_str())?;
        }
        return Ok(());
    }

    fn handle_key_event(&mut self, _key: KeyEvent) -> Result<HandleKeyEventResult, RpnCalcError> {
        return Ok(HandleKeyEventResult::Continue);
    }
}
