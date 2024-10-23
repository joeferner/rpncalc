use anyhow::Result;

use crate::{state::RpnState, undo_action::UndoEvent};

pub mod add;
pub mod divide;
pub mod multiply;
pub mod subtract;

pub trait Func {
    fn execute(&self, state: &mut RpnState) -> Result<Box<dyn UndoEvent>>;
}

mod test {
    #[cfg(test)]
    #[macro_export]
    macro_rules! test_binary_func {
        ($arg0: expr, $arg1: expr, $op: expr, $expected: expr) => {
            use crate::{stack::item::StackItem, state::RpnState};

            let mut state = RpnState::new();
            state.push($arg0).unwrap();
            state.push($arg1).unwrap();
            state.push_str($op).unwrap();
            assert_eq!(state.stack.len(), 1, "stack size after op");
            let answer = state.stack.peek(0).unwrap();
            assert_eq!(*answer, $expected, "answer after op");

            // test undo
            state.undo().unwrap();
            assert_eq!(2, state.stack.len(), "stack size after undo");
            assert_eq!(*state.stack.peek(1).unwrap(), $arg0,);
            assert_eq!(*state.stack.peek(0).unwrap(), $arg1);

            // test redo
            state.redo().unwrap();
            assert_eq!(state.stack.len(), 1);
            let answer = state.stack.peek(0).unwrap();
            assert_eq!(*answer, $expected);
        };
    }
}
