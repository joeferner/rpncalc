use core::slice::Iter;

use anyhow::{anyhow, Result};
use item::StackItem;

pub mod item;

pub struct Stack {
    items: Vec<StackItem>,
}

impl Stack {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn push(&mut self, item: StackItem) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<StackItem> {
        self.items.pop()
    }

    pub fn peek(&self, n: usize) -> Option<&StackItem> {
        if n >= self.len() {
            return None;
        }
        self.items.get(self.len() - 1 - n)
    }

    pub fn pop_n(&mut self, n: usize) -> Result<Vec<StackItem>> {
        let mut result = vec![];
        if n > self.len() {
            return Err(anyhow!("Trying to pop {n} but only {} exist", self.len()));
        }
        for _ in 0..n {
            result.push(self.items.pop().unwrap());
        }
        Ok(result)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn iter(&self) -> Iter<StackItem> {
        self.items.iter()
    }
}
