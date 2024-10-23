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
    pub input: Input,
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
            input: Input::new(),
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

pub struct Input {
    /// Current value of the input box
    input: String,
    /// Position of cursor in the editor area.
    character_index: usize,
}

impl Input {
    pub fn new() -> Self {
        Self {
            input: "".to_string(),
            character_index: 0,
        }
    }

    pub fn get_character_index(&self) -> usize {
        self.character_index
    }

    pub fn get_input(&self) -> &str {
        &self.input
    }

    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can be contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    pub fn clear(&mut self) {
        self.input.clear();
        self.reset_cursor();
    }
}
