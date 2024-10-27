use std::{collections::HashMap, sync::Arc};

use anyhow::{anyhow, Result};
use basic::basic_register_functions;
use trig::trig_register_functions;
use variable::variable_register_functions;

use crate::{
    stack::item::StackItem,
    state::RpnState,
    undo_action::{binary::BinaryFuncUndoEvent, unary::UnaryFuncUndoEvent, UndoEvent},
};

pub mod basic;
pub mod trig;
pub mod variable;

pub fn register_functions(functions: &mut HashMap<String, Arc<Box<dyn Func>>>) {
    basic_register_functions(functions);
    trig_register_functions(functions);
    variable_register_functions(functions);
}

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

pub(super) fn execute_unary<F>(state: &mut RpnState, calc: F) -> Result<Box<dyn UndoEvent>>
where
    F: FnOnce(&StackItem) -> Result<StackItem>,
{
    if state.stack.len() < 1 {
        return Err(anyhow!("Not enough arguments"));
    }
    let a = state.stack.peek(0).unwrap().clone();
    let result = calc(&a)?;
    state.stack.pop_n(1)?;
    state.stack.push(result.clone());
    Ok(Box::new(UnaryFuncUndoEvent::new(a, result)))
}

#[cfg(test)]
mod test {
    #[macro_export]
    macro_rules! test_expr {
        ($expr: expr, $expected: expr) => {
            use crate::{stack::item::StackItem, state::RpnState};

            let mut state = RpnState::new().unwrap();
            state.push_str(&($expr).to_string()).unwrap();
            assert_eq!(state.stack.len(), 1, "stack size after op");
            let answer = state.stack.peek(0).unwrap();
            assert_eq!(*answer, $expected, "answer after op");

            // test undo
            state.undo().unwrap();
            assert_eq!(0, state.stack.len(), "stack size after undo");

            // test redo
            state.redo().unwrap();
            assert_eq!(state.stack.len(), 1);
            let answer = state.stack.peek(0).unwrap();
            assert_eq!(*answer, $expected);
        };
    }

    #[macro_export]
    macro_rules! test_binary_func {
        ($arg0: expr, $arg1: expr, $op: expr, $expected: expr) => {
            use crate::{stack::item::StackItem, state::RpnState};

            let mut state = RpnState::new().unwrap();
            state.push_str(&($arg0).to_string()).unwrap();
            state.push_str(&($arg1).to_string()).unwrap();
            state.push_str($op).unwrap();
            assert_eq!(state.stack.len(), 1, "stack size after op");
            let answer = state.stack.peek(0).unwrap();
            assert_eq!(*answer, $expected, "answer after op");

            // test undo
            state.undo().unwrap();
            assert_eq!(2, state.stack.len(), "stack size after undo");
            assert_eq!(*state.stack.peek(1).unwrap(), $arg0);
            assert_eq!(*state.stack.peek(0).unwrap(), $arg1);

            // test redo
            state.redo().unwrap();
            assert_eq!(state.stack.len(), 1);
            let answer = state.stack.peek(0).unwrap();
            assert_eq!(*answer, $expected);
        };
    }

    #[macro_export]
    macro_rules! test_unary_angle_func {
        ($angle_mode: expr, $arg0: expr, $op: expr, $expected: expr) => {
            use crate::{stack::item::StackItem, state::RpnState};

            let mut state = RpnState::new().unwrap();
            state.angle_mode = $angle_mode;
            state.push_str(&($arg0).to_string()).unwrap();
            state.push_str($op).unwrap();
            assert_eq!(state.stack.len(), 1, "stack size after op");
            let answer = state.stack.peek(0).unwrap();
            assert_eq!(*answer, $expected, "answer after op");

            // test undo
            state.undo().unwrap();
            assert_eq!(1, state.stack.len(), "stack size after undo");
            assert_eq!(*state.stack.peek(0).unwrap(), $arg0);

            // test redo
            state.redo().unwrap();
            assert_eq!(state.stack.len(), 1);
            let answer = state.stack.peek(0).unwrap();
            assert_eq!(*answer, $expected);
        };
    }
}
