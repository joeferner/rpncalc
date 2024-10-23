use std::{
    io::stdout, path::{Path, PathBuf}, process
};

use anyhow::{Context, Result};
use clap::Parser;
use crossterm::{event::{Event, KeyCode, KeyEvent, KeyModifiers, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags}, execute};
use log::{debug, error, info, LevelFilter};
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Root},
};
use ratatui::{text::Text, Frame};
use state::RpnState;

mod func;
mod stack;
mod state;
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
        terminal.draw(draw).context("failed to draw frame")?;
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

fn handle_key_event(key_event: KeyEvent, state: &mut RpnState) -> Result<()> {
    debug!("key {key_event:?}");
    match key_event.modifiers {
        KeyModifiers::CONTROL => match key_event.code {
            KeyCode::Char('c') => handle_exit(),
            KeyCode::Char('d') => handle_exit(),
            KeyCode::Char('y') => handle_redo(state),
            KeyCode::Char('z') => handle_undo(state),
            _ => {
                debug!("key {key_event:?}");
                Ok(())
            }
        },
        _ => {
            debug!("key {key_event:?}");
            Ok(())
        }
    }
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

fn draw(frame: &mut Frame) {
    let text = Text::raw("Hello World!");
    frame.render_widget(text, frame.area());
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
