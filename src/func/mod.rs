use anyhow::Result;

use crate::{state::RpnState, undo_action::UndoEvent};

pub mod add;

pub trait Func {
    fn execute(&self, state: &mut RpnState) -> Result<Box<dyn UndoEvent>>;
}
