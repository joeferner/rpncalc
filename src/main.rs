use std::{
    io::stdout,
    path::{Path, PathBuf},
    process,
};

use anyhow::{Context, Result};
use clap::Parser;
use crossterm::{
    event::{
        Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, KeyboardEnhancementFlags,
        PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
    },
    execute,
};
use log::{debug, error, info, LevelFilter};
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Root},
};
use state::RpnState;
use ui::draw;

mod func;
mod stack;
mod state;
mod ui;
mod undo_action;
mod undo_stack;

/// RPN Calculator
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(last = true, num_args = 0.., allow_hyphen_values = true)]
    extras: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let config_dir = get_config_dir()?;
    init_logger(&config_dir)?;
    info!("starting rpncalc");

    let mut state = RpnState::new();
    if !args.extras.is_empty() {
        for item in &args.extras {
            state.push_str(item)?;
        }
        for i in 0..state.stack.len() {
            let stack_item = state.stack.peek(state.stack.len() - 1 - i).unwrap();
            println!("{stack_item}");
        }
        return Ok(());
    }

    let mut terminal = ratatui::init();

    execute!(
        stdout(),
        PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES)
    )?;

    terminal.clear()?;
    loop {
        terminal
            .draw(|f| draw(f, &mut state))
            .context("failed to draw frame")?;
        let event = crossterm::event::read().context("failed to read event")?;
        if let Err(e) = handle_ui_event(event, &mut state) {
            error!("failed to handle ui event; error = {e}");
            state.error = Some(e);
        } else {
            state.error = None;
        }
    }
}

fn handle_ui_event(event: Event, state: &mut RpnState) -> Result<()> {
    match event {
        Event::FocusGained => Ok(()),
        Event::FocusLost => Ok(()),
        Event::Key(key_event) => handle_key_event(key_event, state),
        Event::Mouse(mouse_event) => {
            debug!("mouse {mouse_event:?}");
            Ok(())
        }
        Event::Paste(value) => {
            debug!("paste {value:?}");
            Ok(())
        }
        Event::Resize(_, _) => Ok(()),
    }
}

fn handle_key_event(key: KeyEvent, state: &mut RpnState) -> Result<()> {
    if matches!(key.modifiers, KeyModifiers::CONTROL) {
        match key.code {
            KeyCode::Char('c') => return handle_exit(),
            KeyCode::Char('d') => return handle_exit(),
            KeyCode::Char('y') => return handle_redo(state),
            KeyCode::Char('z') => return handle_undo(state),
            _ => {}
        }
    }
    if key.kind == KeyEventKind::Press {
        match key.code {
            KeyCode::Enter => return handle_enter_press(state),
            KeyCode::Char(to_insert) => return handle_char_press(to_insert, state),
            KeyCode::Backspace => return handle_backspace_press(state),
            KeyCode::Delete => return handle_delete_press(state),
            KeyCode::Left => return handle_left_press(state),
            KeyCode::Right => return handle_right_press(state),
            _ => {}
        }
    }
    debug!("key {key:?}");
    Ok(())
}

fn handle_left_press(state: &mut RpnState) -> Result<()> {
    state.ui_input_state.move_cursor_left();
    Ok(())
}

fn handle_right_press(state: &mut RpnState) -> Result<()> {
    state.ui_input_state.move_cursor_right();
    Ok(())
}

fn handle_backspace_press(state: &mut RpnState) -> Result<()> {
    if state.ui_input_state.get_input().is_empty() {
        state.pop()
    } else {
        state.ui_input_state.backspace_char();
        Ok(())
    }
}

fn handle_delete_press(state: &mut RpnState) -> Result<()> {
    state.ui_input_state.delete_char();
    Ok(())
}

fn handle_enter_press(state: &mut RpnState) -> Result<()> {
    let input = state.ui_input_state.get_input().to_string();
    state.push_str(&input)?;
    state.ui_input_state.clear();
    Ok(())
}

fn handle_char_press(to_insert: char, state: &mut RpnState) -> Result<()> {
    state.ui_input_state.enter_char(to_insert);
    Ok(())
}

fn handle_redo(state: &mut RpnState) -> Result<()> {
    state.redo()
}

fn handle_undo(state: &mut RpnState) -> Result<()> {
    state.undo()
}

fn handle_exit() -> Result<()> {
    execute!(stdout(), PopKeyboardEnhancementFlags).ok();
    exit_process(0);
    Ok(())
}

pub fn exit_process(code: i32) {
    ratatui::restore();
    info!("rpncalc exiting (code: {code})");
    process::exit(code);
}

fn get_config_dir() -> Result<PathBuf> {
    let mut p = dirs::config_dir().context("could not find config directory")?;
    p.push("rpncalc");
    Ok(p)
}

fn init_logger(config_dir: &Path) -> Result<()> {
    let mut log_filename = config_dir.to_path_buf();
    log_filename.push("rpncalc.log");

    let file = FileAppender::builder().build(log_filename)?;
    let log_config = log4rs::config::Config::builder()
        .appender(Appender::builder().build("file", Box::new(file)))
        .build(Root::builder().appender("file").build(LevelFilter::Debug))
        .unwrap();
    log4rs::init_config(log_config).unwrap();
    Ok(())
}
