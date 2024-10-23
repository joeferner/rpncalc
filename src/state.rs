use std::{collections::HashMap, sync::Arc};

use anyhow::{anyhow, Error, Result};

use crate::{
    func::{add::AddFunc, Func},
    stack::{item::StackItem, Stack},
    undo_action::push::PushUndoEvent,
    undo_stack::UndoStack,
};

pub struct RpnState {
    pub stack: Stack,
    pub functions: HashMap<String, Arc<Box<dyn Func>>>,
    pub undo_stack: UndoStack,
    pub error: Option<Error>,
}

impl RpnState {
    pub fn new() -> Self {
        let mut functions: HashMap<String, Arc<Box<dyn Func>>> = HashMap::new();
        functions.insert("add".to_string(), Arc::new(Box::new(AddFunc::new())));

        Self {
            stack: Stack::new(),
            functions,
            undo_stack: UndoStack::new(),
            error: None,
        }
    }

    pub fn push_str(&mut self, s: &str) -> Result<()> {
        if let Some(func) = self.functions.get(s) {
            let func = func.clone();
            let undo = func.execute(self)?;
            self.undo_stack.push_undo_stack(undo);
            return Ok(());
        }

        let stack_item = StackItem::from_str(s)?;
        self.stack.push(stack_item.clone());
        self.undo_stack
            .push_undo_stack(Box::new(PushUndoEvent::new(stack_item)));
        Ok(())
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
}
