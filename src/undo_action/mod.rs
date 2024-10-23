use std::fmt::Debug;

use anyhow::Result;

use crate::state::RpnState;

pub mod binary;
pub mod push;
pub mod pop;

pub trait UndoEvent: Debug + Send + Sync {
    fn undo(&self, state: &mut RpnState) -> Result<()>;
    fn redo(&self, state: &mut RpnState) -> Result<()>;
}
