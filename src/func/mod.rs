use anyhow::{anyhow, Result};

use crate::{
    stack::item::StackItem,
    state::RpnState,
    undo_action::{binary::BinaryFuncUndoEvent, UndoEvent},
};

pub mod add;
pub mod divide;
pub mod multiply;
pub mod subtract;

pub trait Func {
    fn execute(&self, state: &mut RpnState) -> Result<Box<dyn UndoEvent>>;
}

pub(super) fn execute_binary<F>(state: &mut RpnState, calc: F) -> Result<Box<dyn UndoEvent>>
where
    F: FnOnce(&StackItem, &StackItem) -> Result<StackItem>,
{
    if state.stack.len() < 2 {
        return Err(anyhow!("Not enough arguments"));
    }
    let a = state.stack.peek(1).unwrap().clone();
    let b = state.stack.peek(0).unwrap().clone();
    let result = calc(&a, &b)?;
    state.stack.pop_n(2)?;
    state.stack.push(result.clone());
    Ok(Box::new(BinaryFuncUndoEvent::new(a, b, result)))
}

mod test {
    #[cfg(test)]
    #[macro_export]
    macro_rules! test_binary_func {
        ($arg0: expr, $arg1: expr, $op: expr, $expected: expr) => {
            use crate::{stack::item::StackItem, state::RpnState};

            let mut state = RpnState::new().unwrap();
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
