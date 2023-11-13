use crate::stack_item::StackItem;

pub struct Stack {
    items: Vec<StackItem>,
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

    pub fn items(&self) -> &Vec<StackItem> {
        return &self.items;
    }
}
