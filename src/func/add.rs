use anyhow::Result;

use crate::{state::RpnState, undo_action::UndoEvent};

use super::{execute_binary, Func};

pub struct AddFunc {}

impl AddFunc {
    pub fn new() -> Self {
        Self {}
    }
}

impl Func for AddFunc {
    fn execute(&self, state: &mut RpnState) -> Result<Box<dyn UndoEvent>> {
        execute_binary(state, |a, b| a.add(b))
    }
}

#[cfg(test)]
mod test {
    use crate::test_binary_func;

    #[test]
    fn test_add() {
        test_binary_func!(
            StackItem::Number(1.0, 10),
            StackItem::Number(2.0, 10),
            "add",
            StackItem::Number(3.0, 10)
        );
    }
}
