use std::{path::{Path, PathBuf}, process};

use anyhow::{Context, Result};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use log::{debug, info, LevelFilter};
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

fn main() -> Result<()> {
    let config_dir = get_config_dir()?;
    init_logger(&config_dir)?;
    info!("starting rpncalc");

    let mut terminal = ratatui::init();
    let mut state = RpnState::new();
    terminal.clear()?;
    loop {
        terminal.draw(draw).context("failed to draw frame")?;
        match crossterm::event::read().context("failed to read event")? {
            Event::FocusGained => {}
            Event::FocusLost => {}
            Event::Key(key_event) => handle_key_event(key_event, &mut state)?,
            Event::Mouse(mouse_event) => debug!("mouse {mouse_event:?}"),
            Event::Paste(value) => debug!("paste {value:?}"),
            Event::Resize(_, _) => {}
        }
    }
}

fn handle_key_event(key_event: KeyEvent, _state: &mut RpnState) -> Result<()> {
    if matches!(key_event.modifiers, KeyModifiers::CONTROL) {
        if matches!(key_event.code, KeyCode::Char('c')) {
            exit_process(0);
        }
    } else {
        debug!("key {key_event:?}");
    }
    Ok(())
}

pub fn exit_process(code: i32) {
    ratatui::restore();
    process::exit(code);
}

fn draw(frame: &mut Frame) {
    let text = Text::raw("Hello World!");
    frame.render_widget(text, frame.area());
}

fn get_config_dir() -> Result<PathBuf> {
    let mut p = dirs::config_dir().context("could not find config directory")?;
    p.push("rpncalc");
    Ok(p.into())
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
