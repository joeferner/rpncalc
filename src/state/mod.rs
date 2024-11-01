use core::f64;
use std::{collections::HashMap, sync::Arc};

use angle_mode::AngleMode;
use anyhow::{anyhow, Error, Result};
use constant::Constant;
use input::Input;
use num_format::SystemLocale;
use ratatui::widgets::ListState;

use crate::{
    expr::run::run_expression,
    func::{register_functions, Func},
    stack::{item::StackItem, Stack},
    undo_action::pop::PopUndoEvent,
    undo_stack::UndoStack,
};

pub mod angle_mode;
pub mod constant;
pub mod input;

pub struct RpnState {
    pub locale: SystemLocale,
    pub precision: usize,
    pub scientific_notation_limit: f64,
    pub angle_mode: AngleMode,
    pub stack: Stack,
    pub functions: HashMap<String, Arc<Box<dyn Func>>>,
    pub constants: HashMap<String, Arc<Constant>>,
    pub variables: HashMap<String, StackItem>,
    pub undo_stack: UndoStack,
    pub error: Option<Error>,
    pub completions: Option<Vec<String>>,
    pub ui_input_state: Input,
    pub ui_stack_state: ListState,
}

impl RpnState {
    pub fn new() -> Result<Self> {
        let mut state = Self {
            locale: SystemLocale::default()?,
            stack: Stack::new(),
            angle_mode: AngleMode::Degrees,
            functions: HashMap::new(),
            constants: HashMap::new(),
            variables: HashMap::default(),
            undo_stack: UndoStack::new(),
            error: None,
            completions: None,
            ui_input_state: Input::new(),
            ui_stack_state: ListState::default(),
            precision: 10,
            scientific_notation_limit: 1_000_000_000.0,
        };

        register_functions(&mut state);

        state
            .constants
            .insert("pi".to_string(), Arc::new(Constant::new(f64::consts::PI)));
        state
            .constants
            .insert("e".to_string(), Arc::new(Constant::new(f64::consts::E)));
        state
            .constants
            .insert("tau".to_string(), Arc::new(Constant::new(f64::consts::TAU)));

        Ok(state)
    }

    pub fn push_str(&mut self, s: &str) -> Result<()> {
        run_expression(s, self)
    }

    pub fn undo(&mut self) -> Result<()> {
        if let Some(undo) = self.undo_stack.get_undo_item() {
            undo.undo(self)
        } else {
            Err(anyhow!("Nothing to undo"))
        }
    }

    pub fn redo(&mut self) -> Result<()> {
        if let Some(redo) = self.undo_stack.get_redo_item() {
            redo.redo(self)
        } else {
            Err(anyhow!("Nothing to redo"))
        }
    }

    pub fn pop(&mut self) -> Result<()> {
        if let Some(stack_item) = self.stack.pop() {
            self.undo_stack
                .push_undo_stack(Box::new(PopUndoEvent::new(stack_item)));
            Ok(())
        } else {
            Err(anyhow!("Pop failed, stack is empty"))
        }
    }

    pub fn register_function(&mut self, func: Box<dyn Func>) {
        let func = Arc::new(func);
        self.functions.insert(func.name().to_string(), func.clone());
        for alias in func.aliases() {
            self.functions.insert(alias.to_string(), func.clone());
        }
    }
}

#[cfg(test)]
mod test {
    use core::f64;

    use anyhow::Result;

    use crate::stack::item::StackItem;

    use super::RpnState;

    #[test]
    fn test_constant() -> Result<()> {
        let mut state = RpnState::new()?;

        state.push_str("pi")?;

        let v = state.stack.peek(0).unwrap();
        assert_eq!(&StackItem::Number(f64::consts::PI, 10), v);

        Ok(())
    }
}
