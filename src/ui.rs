use crate::error::RpnCalcError;
use crate::rpn_calc::RpnCalc;
use crate::stack_item::StackItem;
use crate::ui_less::Less;
use crate::units::AngleUnits;
use crossterm::event::{KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
    QueueableCommand,
};
use std::cmp::min;
use std::io::{stdout, Write};

const DEFAULT_STACK_HEIGHT: u16 = 8;
const DEFAULT_STACK_WIDTH: u16 = 20;

struct InteractiveState {
    console_width: u16,
    console_height: u16,
    stack_height: u16,
    stack_width: u16,
    message: Option<String>,
    input: String,
    cursor_location: u16,
    base: u16,
    help: Option<Less>,
}

impl InteractiveState {
    pub fn clear_input(&mut self) -> () {
        self.cursor_location = 0;
        self.input = "".to_string();
    }

    pub fn get_top(&self) -> u16 {
        // header (1)
        // stack (stack height)
        // prompt (1)
        // buffer (1) - to prevent enter key from causing a new line
        return (self.console_height as i16 - 1 - self.stack_height as i16 - 1 - 1).max(4) as u16;
    }
}

pub fn run_interactive(rpn_calc: RpnCalc) -> Result<(), RpnCalcError> {
    let state = InteractiveState {
        console_width: 0,
        console_height: 0,
        stack_height: DEFAULT_STACK_HEIGHT,
        stack_width: DEFAULT_STACK_WIDTH,
        message: None,
        input: "".to_string(),
        cursor_location: 0,
        base: 10,
        help: None,
    };

    enable_raw_mode()?;
    run_loop(rpn_calc, state)?;
    disable_raw_mode()?;
    return Ok(());
}

fn run_loop(mut rpn_calc: RpnCalc, mut state: InteractiveState) -> Result<(), RpnCalcError> {
    loop {
        let event = read()?;

        match event {
            Event::Key(key) => {
                if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
                    break;
                }
                if let Some(help) = state.help.as_mut() {
                    if matches!(help.key_event(key)?, HandleKeyEventResult::Exit) {
                        execute!(stdout(), LeaveAlternateScreen)?;
                        state.help = None;
                        redraw(&rpn_calc, &state)?;
                    }
                } else if matches!(
                    handle_key_event(&mut rpn_calc, &mut state, key)?,
                    HandleKeyEventResult::Exit
                ) {
                    break;
                }
            }
            Event::Resize(width, height) => {
                state.console_width = width;
                state.console_height = height;
                state.stack_width = min(width, DEFAULT_STACK_WIDTH);
                state.stack_height = min(height - 2, DEFAULT_STACK_HEIGHT);
                if let Some(help) = state.help.as_mut() {
                    help.resize(width, height)?;
                } else {
                    redraw(&rpn_calc, &state)?;
                }
            }
            other => {
                println!("Event::{:?}\r", other);
            }
        }
    }
    return Ok(());
}

fn redraw(rpn_calc: &RpnCalc, state: &InteractiveState) -> Result<(), RpnCalcError> {
    if state.console_width == 0 && state.console_height == 0 {
        return Ok(());
    }

    if let Some(help) = &state.help {
        return help.redraw();
    }

    let top = state.get_top();

    // draw header line
    stdout().queue(cursor::MoveTo(0, top))?;
    stdout().queue(Clear(ClearType::CurrentLine))?;
    if let Some(message) = &state.message {
        stdout().queue(Print(message))?;
    } else {
        // draw mode
        let angle_mode = match rpn_calc.angle_mode {
            AngleUnits::Degrees => "DEG",
            AngleUnits::Radians => "RAD",
        };

        // draw base
        let base_str = match state.base {
            2 => "BIN".to_string(),
            8 => "OCT".to_string(),
            10 => "DEC".to_string(),
            16 => "HEX".to_string(),
            _ => format!("BASE{}", state.base),
        };

        let status_line = format!("{} {}", angle_mode, base_str);

        stdout().queue(Print(status_line))?;
    }

    // draw stack
    for i in 0..state.stack_height {
        let stack_offset = (state.stack_height - i) as usize;
        let stack_index = rpn_calc.stack.items.len() as i16 - stack_offset as i16;
        stdout().queue(cursor::MoveTo(0, top + 1 + i))?;
        stdout().queue(Clear(ClearType::CurrentLine))?;
        let mut stack_item: Option<&StackItem> = None;
        if stack_index >= 0 {
            stack_item = rpn_calc.stack.items.get(stack_index as usize);
        }
        let stack_item_str = match stack_item {
            Some(stack_item) => format_stack_item(stack_offset, stack_item, state),
            None => format!("{}:", stack_offset),
        };
        stdout().queue(Print(stack_item_str))?;
    }

    // draw prompt
    let prompt = format!(">{}", state.input);
    stdout().queue(cursor::MoveTo(0, top + state.stack_height + 1))?;
    stdout().queue(Clear(ClearType::CurrentLine))?;
    stdout().queue(Print(prompt))?;
    stdout().queue(cursor::MoveTo(state.cursor_location + 1, top + state.stack_height + 1))?;
    stdout().flush()?;

    return Ok(());
}

fn format_stack_item(display_stack_index: usize, stack_item: &StackItem, state: &InteractiveState) -> String {
    let prefix = format!("{}:", display_stack_index);
    let width = state.stack_width as usize - prefix.len();
    let stack_item_str = stack_item.to_string_format(width, state.base);
    let s = format!("{: >width$}", stack_item_str, width = width);
    return format!("{}{}", prefix, s);
}

pub enum HandleKeyEventResult {
    Exit,
    Continue,
}

fn handle_key_event(
    rpn_calc: &mut RpnCalc,
    state: &mut InteractiveState,
    key: KeyEvent,
) -> Result<HandleKeyEventResult, RpnCalcError> {
    if key.kind == KeyEventKind::Press {
        match key.code {
            KeyCode::Char(ch) => {
                state.input.push(ch);
                state.cursor_location = state.input.len() as u16
            }
            KeyCode::Enter => {
                let str = state.input.trim();
                if str == "exit" || str == "quit" {
                    return Ok(HandleKeyEventResult::Exit);
                } else if str == "help" {
                    execute!(stdout(), EnterAlternateScreen)?;
                    state.clear_input();
                    state.help = Some(Less::new(
                        state.console_width,
                        state.console_height,
                        create_help_string(rpn_calc),
                    ));
                } else if str == "bin" {
                    state.base = 2;
                } else if str == "oct" {
                    state.base = 8;
                } else if str == "dec" {
                    state.base = 10;
                } else if str == "hex" {
                    state.base = 16;
                } else if let Err(err) = rpn_calc.push_str(str) {
                    state.message = Some(format!("{}", err));
                } else {
                    state.clear_input();
                }
            }
            KeyCode::Backspace => {
                if state.input.is_empty() {
                    if !rpn_calc.stack.items.is_empty() {
                        rpn_calc.push_str("drop")?;
                    }
                } else if state.cursor_location > 0 {
                    let loc = state.cursor_location as usize;
                    let mut new_input = state.input[..loc - 1].to_string();
                    new_input.push_str(&state.input[loc..]);
                    state.input = new_input;
                    state.cursor_location -= 1;
                }
            }
            KeyCode::Left => {
                if state.cursor_location > 0 {
                    state.cursor_location -= 1;
                }
            }
            KeyCode::Right => {
                if state.cursor_location < state.input.len() as u16 {
                    state.cursor_location += 1;
                }
            }
            _ => state.message = Some(format!("key::{:?}\r", key)),
        }
        redraw(rpn_calc, state)?;
    }
    return Ok(HandleKeyEventResult::Continue);
}

fn create_help_string(rpn_calc: &RpnCalc) -> String {
    let mut result = "".to_string();

    // function list
    result.push_str("Functions\n");
    result.push_str("=========\n");
    let mut longest_function_name = 0;
    for key in rpn_calc.functions.keys() {
        longest_function_name = longest_function_name.max(key.len());
    }
    let mut function_keys: Vec<_> = rpn_calc.functions.keys().collect();
    function_keys.sort();
    for key in function_keys {
        let fn_help = rpn_calc.functions.get(key).unwrap().get_help();
        let key_str = format!("{: >width$}", key, width = longest_function_name);
        result.push_str(format!(" {} {}\n", key_str, fn_help).as_str());
    }

    return result;
}
