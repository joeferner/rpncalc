use std::{collections::HashMap, sync::Arc};

use anyhow::{anyhow, Error, Result};
use num_format::SystemLocale;
use ratatui::widgets::ListState;

use crate::{
    func::{
        basic::{add::AddFunc, divide::DivideFunc, multiply::MultiplyFunc, subtract::SubtractFunc},
        trig::{cos::CosFunc, sin::SinFunc, tan::TanFunc},
        Func,
    },
    stack::{item::StackItem, Stack},
    undo_action::{pop::PopUndoEvent, push::PushUndoEvent},
    undo_stack::UndoStack,
};

pub struct RpnState {
    pub locale: SystemLocale,
    pub precision: usize,
    pub scientific_notation_limit: f64,
    pub angle_mode: AngleMode,
    pub stack: Stack,
    pub functions: HashMap<String, Arc<Box<dyn Func>>>,
    pub undo_stack: UndoStack,
    pub error: Option<Error>,
    pub ui_input_state: Input,
    pub ui_stack_state: ListState,
}

impl RpnState {
    pub fn new() -> Result<Self> {
        let mut functions: HashMap<String, Arc<Box<dyn Func>>> = HashMap::new();

        let add_func: Arc<Box<dyn Func>> = Arc::new(Box::new(AddFunc::new()));
        functions.insert("add".to_string(), add_func.clone());
        functions.insert("+".to_string(), add_func);

        let subtract_func: Arc<Box<dyn Func>> = Arc::new(Box::new(SubtractFunc::new()));
        functions.insert("subtract".to_string(), subtract_func.clone());
        functions.insert("-".to_string(), subtract_func);

        let multiply_func: Arc<Box<dyn Func>> = Arc::new(Box::new(MultiplyFunc::new()));
        functions.insert("multiply".to_string(), multiply_func.clone());
        functions.insert("*".to_string(), multiply_func);

        let divide_func: Arc<Box<dyn Func>> = Arc::new(Box::new(DivideFunc::new()));
        functions.insert("divide".to_string(), divide_func.clone());
        functions.insert("/".to_string(), divide_func);

        functions.insert("sin".to_string(), Arc::new(Box::new(SinFunc::new())));
        functions.insert("cos".to_string(), Arc::new(Box::new(CosFunc::new())));
        functions.insert("tan".to_string(), Arc::new(Box::new(TanFunc::new())));

        Ok(Self {
            locale: SystemLocale::default()?,
            stack: Stack::new(),
            angle_mode: AngleMode::Degrees,
            functions,
            undo_stack: UndoStack::new(),
            error: None,
            ui_input_state: Input::new(),
            ui_stack_state: ListState::default(),
            precision: 10,
            scientific_notation_limit: 1_000_000_000.0,
        })
    }

    pub fn push_str(&mut self, s: &str) -> Result<()> {
        if let Some(func) = self.functions.get(s) {
            let func = func.clone();
            let undo = func.execute(self)?;
            self.undo_stack.push_undo_stack(undo);
            return Ok(());
        }

        let stack_item = StackItem::from_str(s)?;
        self.push(stack_item)
    }

    pub fn push(&mut self, stack_item: StackItem) -> Result<()> {
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

    pub fn pop(&mut self) -> Result<()> {
        if let Some(stack_item) = self.stack.pop() {
            self.undo_stack
                .push_undo_stack(Box::new(PopUndoEvent::new(stack_item)));
            Ok(())
        } else {
            Err(anyhow!("Pop failed, stack is empty"))
        }
    }
}

pub struct Input {
    /// Current value of the input box
    input: String,
    /// Position of cursor in the editor area.
    character_index: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum AngleMode {
    Degrees,
    Radians,
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

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn move_cursor_home(&mut self) {
        self.character_index = 0;
    }

    pub fn move_cursor_end(&mut self) {
        self.character_index = self.input.chars().count();
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

    pub fn backspace_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_rightmost = self.character_index != self.input.len();
        if is_not_cursor_rightmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let from_left_to_current_index = self.character_index;
            let current_index = self.character_index + 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
        }
    }

    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }
}
