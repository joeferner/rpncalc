use std::cmp::{min};
use std::io::stdout;
use crate::rpn_calc::{AngleMode, RpnCalc, RpnCalcError};
use crossterm::{event::{read, Event, KeyCode}, cursor, terminal::{disable_raw_mode, enable_raw_mode}, ExecutableCommand};
use crossterm::event::{KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use crate::stack_item::StackItem;

const DEFAULT_STACK_HEIGHT: u16 = 4;
const DEFAULT_STACK_WIDTH: u16 = 20;

struct InteractiveState {
    console_width: u16,
    console_height: u16,
    stack_height: u16,
    stack_width: u16,
    message: Option<String>,
    input: String,
    cursor_location: u16,
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
                handle_key_event(&mut rpn_calc, &mut state, key)?;
            }
            Event::Resize(width, height) => {
                state.console_width = width;
                state.console_height = height;
                state.stack_width = min(width, DEFAULT_STACK_WIDTH);
                state.stack_height = min(height - 2, DEFAULT_STACK_HEIGHT);
                redraw(&rpn_calc, &state)?;
            }
            other => {
                println!("Event::{:?}\r", other);
            }
        }
    }
    return Ok(());
}

fn get_top(state: &InteractiveState) -> u16 {
    // header (1)
    // stack (stack height)
    // prompt (1)
    // buffer (1) - to prevent enter key from causing a new line
    return state.console_height - 1 - state.stack_height - 1 - 1;
}

fn redraw(rpn_calc: &RpnCalc, state: &InteractiveState) -> Result<(), RpnCalcError> {
    if state.console_width == 0 && state.console_height == 0 {
        return Ok(());
    }

    let top = get_top(state);

    // draw header line
    stdout().execute(cursor::MoveTo(0, top))?;
    stdout().execute(Clear(ClearType::CurrentLine))?;
    if let Some(message) = &state.message {
        stdout().execute(Print(message))?;
    } else {
        // draw mode
        let angle_mode = match rpn_calc.angle_mode() {
            AngleMode::Degrees => "DEG",
            AngleMode::Radians => "RAD",
        };

        let status_line = format!("{}", angle_mode);
        stdout().execute(Print(status_line))?;
    }

    // draw stack
    for i in 0..state.stack_height {
        let stack_offset = (state.stack_height - i) as usize;
        let stack_index = rpn_calc.stack().items().len() as i16 - stack_offset as i16;
        stdout().execute(cursor::MoveTo(0, top + 1 + i))?;
        stdout().execute(Clear(ClearType::CurrentLine))?;
        let mut stack_item: Option<&StackItem> = None;
        if stack_index >= 0 {
            stack_item = rpn_calc.stack().items().get(stack_index as usize);
        }
        let stack_item_str = match stack_item {
            Some(stack_item) => format_stack_item(stack_offset, stack_item, state),
            None => format!("{}:", stack_offset)
        };
        stdout().execute(Print(stack_item_str))?;
    }

    // draw prompt
    stdout().execute(cursor::MoveTo(0, top + state.stack_height + 1))?;
    stdout().execute(Clear(ClearType::CurrentLine))?;
    stdout().execute(Print(&state.input))?;
    stdout().execute(cursor::MoveTo(state.cursor_location, top + state.stack_height + 1))?;

    return Ok(());
}

fn format_stack_item(display_stack_index: usize, stack_item: &StackItem, state: &InteractiveState) -> String {
    let prefix = format!("{}:", display_stack_index);
    let stack_item_str = format!("{}", stack_item);
    let width = state.stack_width as usize - prefix.len();
    let s = format!("{: >width$}", stack_item_str, width = width);
    return format!("{}{}", prefix, s);
}

fn handle_key_event(rpn_calc: &mut RpnCalc, state: &mut InteractiveState, key: KeyEvent) -> Result<(), RpnCalcError> {
    if key.kind == KeyEventKind::Press {
        match key.code {
            KeyCode::Char(ch) => {
                state.input.push(ch);
                state.cursor_location = state.input.len() as u16
            }
            KeyCode::Enter => {
                if let Err(err) = rpn_calc.push_str(state.input.as_str()) {
                    state.message = Some(format!("{}", err));
                } else {
                    state.cursor_location = 0;
                    state.input = "".to_string();
                }
            }
            KeyCode::Backspace => {
                if state.cursor_location > 0 {
                    let loc = state.cursor_location as usize;
                    let mut new_input = state.input[..loc - 1].to_string();
                    new_input.push_str(&state.input[loc..]);
                    state.input = new_input;
                    state.cursor_location = state.cursor_location - 1;
                }
            }
            KeyCode::Left => {
                if state.cursor_location > 0 {
                    state.cursor_location = state.cursor_location - 1;
                }
            }
            KeyCode::Right => {
                if state.cursor_location < state.input.len() as u16 {
                    state.cursor_location = state.cursor_location + 1;
                }
            }
            _ => state.message = Some(format!("key::{:?}\r", key))
        }
        redraw(rpn_calc, state)?;
    }
    return Ok(());
}
