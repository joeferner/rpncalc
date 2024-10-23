use anyhow::Result;

use crate::state::RpnState;

pub mod binary;

pub trait UndoEvent {
    fn undo(&self, state: &mut RpnState) -> Result<()>;
    fn redo(&self, state: &mut RpnState) -> Result<()>;
}
