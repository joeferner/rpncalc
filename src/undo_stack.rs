use std::sync::Arc;

use crate::undo_action::UndoEvent;

#[derive(Debug)]
pub struct UndoStack {
    pub items: Vec<Arc<Box<dyn UndoEvent>>>,
    pub undo_index: usize,
}

impl UndoStack {
    pub fn new() -> Self {
        Self {
            items: vec![],
            // points to the item after the item to be undone next
            undo_index: 1,
        }
    }

    pub fn get_undo_item(&mut self) -> Option<Arc<Box<dyn UndoEvent>>> {
        // already undid all items
        if self.undo_index == 0 {
            return None;
        }

        let undo = self.items.get(self.undo_index - 1);
        match undo {
            Some(undo) => {
                self.undo_index -= 1;
                Some(undo.clone())
            }
            None => None,
        }
    }

    pub fn get_redo_item(&mut self) -> Option<Arc<Box<dyn UndoEvent>>> {
        // already redid all items
        if self.undo_index == self.items.len() {
            return None;
        }

        let redo = self.items.get(self.undo_index);
        match redo {
            Some(redo) => {
                self.undo_index += 1;
                Some(redo.clone())
            }
            None => None,
        }
    }

    pub fn push_undo_stack(&mut self, undo: Box<dyn UndoEvent>) {
        self.items.truncate(self.undo_index);
        self.items.push(Arc::new(undo));
        self.undo_index = self.items.len();
    }
}
