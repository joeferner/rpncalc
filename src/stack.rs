use crate::stack_item::StackItem;

pub struct Stack {
    pub items: Vec<StackItem>,
}

impl Stack {
    pub fn new() -> Self {
        return Stack { items: Vec::new() };
    }

    pub fn push(&mut self, stack_item: StackItem) -> () {
        self.items.push(stack_item)
    }

    pub fn pop(&mut self) -> Option<StackItem> {
        return self.items.pop();
    }

    pub fn peek(&self) -> Option<StackItem> {
        return self.peekn(0);
    }

    pub fn peekn(&self, idx: i32) -> Option<StackItem> {
        let stack_index = self.items.len() as i16 - idx as i16 - 1;
        if stack_index < 0 {
            return None;
        }
        if let Some(si) = self.items.get(stack_index as usize) {
            return Some(si.clone());
        }
        return None;
    }
}
