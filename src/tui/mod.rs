mod column_layout;
mod console;
mod control;
mod help;
mod less;
pub mod nroff;
mod prompt;
mod stack;
mod status;

use crate::error::RpnCalcError;
use crate::rpn_calc::RpnCalc;
use crate::tui::column_layout::{ColumnLayout, ColumnLayoutInit};
use crate::tui::console::{Console, CrosstermConsole};
use crate::tui::control::Control;
pub use crate::tui::help::create_help_string;
use crate::tui::less::Less;
use crate::tui::prompt::{Prompt, PromptInit};
use crate::tui::stack::{Stack, StackInit};
use crate::tui::status::{Status, StatusInit};
use crossterm::event::KeyModifiers;
use crossterm::terminal::{size, Clear, ClearType, LeaveAlternateScreen};
use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
    QueueableCommand,
};
use std::cell::RefCell;
use std::cmp::min;
use std::io::{stdout, Write};
use std::rc::Rc;

const DEFAULT_STACK_HEIGHT: u16 = 8;
const DEFAULT_STACK_WIDTH: u16 = 25;

struct Tui<TConsole>
where
    TConsole: Console,
{
    console: Rc<RefCell<TConsole>>,
    console_width: u16,
    console_height: u16,
    stack_height_config: u16,
    stack_width_config: u16,
    control: ColumnLayout,
    status: Rc<RefCell<Status>>,
    stack: Rc<RefCell<Stack>>,
    help: Option<Less>,
    rpn_calc: Rc<RefCell<RpnCalc>>,
}

pub fn run_tui(rpn_calc: RpnCalc) -> Result<(), RpnCalcError> {
    let (width, height) = size()?;
    let stack_height_config = DEFAULT_STACK_HEIGHT;
    let stack_width_config = DEFAULT_STACK_WIDTH;

    let rpn_calc = Rc::new(RefCell::new(rpn_calc));

    // create controls
    let status = Rc::new(RefCell::new(Status::new(StatusInit {
        rpn_calc: rpn_calc.clone(),
    })));
    let stack = Rc::new(RefCell::new(Stack::new(StackInit {
        rpn_calc: rpn_calc.clone(),
        width: stack_width_config,
        height: stack_height_config,
    })));
    let prompt = Rc::new(RefCell::new(Prompt::new(PromptInit {
        rpn_calc: rpn_calc.clone(),
    })));
    let mut column_layout = ColumnLayout::new(ColumnLayoutInit {
        top: 0,
        controls: vec![status.clone(), stack.clone(), prompt],
    });
    column_layout.set_focused_control_index(2);

    // adjust top
    let total_height = column_layout.get_height();
    for _ in 0..total_height {
        println!();
    }
    let (_, cursor_y) = cursor::position()?;
    column_layout.set_top(cursor_y - total_height);

    let mut state = Tui {
        console: Rc::new(RefCell::new(CrosstermConsole::new())),
        console_width: width,
        console_height: height,
        stack_width_config,
        stack_height_config,
        control: column_layout,
        status,
        stack,
        help: None,
        rpn_calc,
    };

    enable_raw_mode()?;
    state.run_loop()?;
    disable_raw_mode()?;
    return Ok(());
}

impl<TConsole> Tui<TConsole>
where
    TConsole: Console,
{
    fn run_loop(&mut self) -> Result<(), RpnCalcError> {
        loop {
            self.redraw()?;
            let event = read()?;

            match event {
                Event::Key(key) => {
                    if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
                        break;
                    }
                    if let Some(help) = self.help.as_mut() {
                        if matches!(help.key_event(key)?, HandleKeyEventResult::Exit) {
                            execute!(stdout(), LeaveAlternateScreen)?;
                            self.help = None;
                            self.redraw()?;
                        }
                    } else {
                        match self.control.handle_key_event(key)? {
                            HandleKeyEventResult::Exit => {
                                stdout().queue(cursor::MoveTo(
                                    0,
                                    self.control.get_top() + self.control.get_height() + 1,
                                ))?;
                                stdout().queue(Clear(ClearType::CurrentLine))?;
                                stdout().flush()?;
                                break;
                            }
                            HandleKeyEventResult::Help => {
                                self.console.borrow_mut().enter_alternate_screen()?;
                                self.help = Some(Less::new(
                                    self.console_width,
                                    self.console_height,
                                    create_help_string(&self.rpn_calc.borrow()),
                                ));
                            }
                            HandleKeyEventResult::SetMessage(message) => {
                                self.status.borrow_mut().set_message(message);
                            }
                            HandleKeyEventResult::Continue => {}
                        }
                    }
                }
                Event::Resize(width, height) => {
                    self.console_width = width;
                    self.console_height = height;
                    self.control.set_width(min(width, self.stack_width_config));
                    {
                        let mut s = self.stack.borrow_mut();
                        s.set_height(min(height - 2, self.stack_height_config));
                    }
                    if let Some(help) = self.help.as_mut() {
                        help.resize(width, height)?;
                    } else {
                        self.redraw()?;
                    }
                }
                _ => {}
            }
        }
        return Ok(());
    }

    fn redraw(&self) -> Result<(), RpnCalcError> {
        if let Some(help) = &self.help {
            return help.redraw();
        }

        self.control.redraw(&mut *self.console.borrow_mut())?;
        stdout().flush()?;
        return Ok(());
    }
}

pub enum HandleKeyEventResult {
    Exit,
    Continue,
    Help,
    SetMessage(Option<String>),
}
