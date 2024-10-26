use anyhow::{anyhow, Result};

use crate::{func::Func, stack::item::StackItem, state::RpnState, undo_action::UndoEvent};

pub struct StoreFunc {}

impl StoreFunc {
    pub fn new() -> Self {
        Self {}
    }
}

impl Func for StoreFunc {
    fn execute(&self, state: &mut RpnState) -> Result<Box<dyn UndoEvent>> {
        if state.stack.len() < 2 {
            return Err(anyhow!("Not enough arguments"));
        }
        let a = state.stack.peek(1).unwrap().clone();
        let b = state.stack.peek(0).unwrap().clone();

        let name = match b {
            StackItem::String(name) => name,
            _ => {
                return Err(anyhow!("Second argument must be a string"));
            }
        };

        state.stack.pop_n(2)?;
        let previous_variable_value = state.variables.get(&name).cloned();
        state.variables.insert(name.clone(), a.clone());

        Ok(Box::new(StoreFuncUndoEvent {
            name,
            value: a,
            previous_variable_value,
        }))
    }
}

#[derive(Debug)]
pub struct StoreFuncUndoEvent {
    name: String,
    value: StackItem,
    previous_variable_value: Option<StackItem>,
}

impl UndoEvent for StoreFuncUndoEvent {
    fn undo(&self, state: &mut RpnState) -> Result<()> {
        match &self.previous_variable_value {
            Some(v) => state.variables.insert(self.name.clone(), v.clone()),
            None => state.variables.remove(&self.name),
        };
        state.stack.push(self.value.clone());
        state.stack.push(StackItem::String(self.name.clone()));
        Ok(())
    }

    fn redo(&self, state: &mut RpnState) -> Result<()> {
        if state.stack.len() < 2 {
            return Err(anyhow!(
                "unexpected state for redo, expected at least 2 items on the stack"
            ));
        }
        let found_a = state.stack.peek(1).unwrap();
        let found_b = state.stack.peek(0).unwrap();

        if found_a != &self.value {
            return Err(anyhow!(
                "unexpected state for redo, expected item on the stack to be the same as argument 0"
            ));
        }

        if found_b != &StackItem::String(self.name.clone()) {
            return Err(anyhow!(
                "unexpected state for redo, expected item on the stack to be the same as argument 1"
            ));
        }

        state
            .variables
            .insert(self.name.clone(), self.value.clone());
        state.stack.pop_n(2)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{stack::item::StackItem, state::RpnState};

    #[test]
    fn test_store() {
        let mut state = RpnState::new().unwrap();
        state
            .variables
            .insert("x".to_string(), StackItem::Number(1.0, 10));
        state.push_str("42").unwrap();
        state.push_str("'x'").unwrap();
        state.push_str("store").unwrap();
        assert_eq!(0, state.stack.len());
        assert_eq!(1, state.variables.len());
        let x = state.variables.get("x").unwrap();
        assert_eq!(StackItem::Number(42.0, 10), x.clone());

        // undo
        state.undo().unwrap();
        assert_eq!(2, state.stack.len());
        assert_eq!(
            StackItem::Number(42.0, 10),
            state.stack.peek(1).unwrap().clone()
        );
        assert_eq!(
            StackItem::String("x".to_string()),
            state.stack.peek(0).unwrap().clone()
        );
        let x = state.variables.get("x").unwrap();
        assert_eq!(StackItem::Number(1.0, 10), x.clone());

        // redo
        state.redo().unwrap();
        assert_eq!(0, state.stack.len());
        assert_eq!(1, state.variables.len());
        let x = state.variables.get("x").unwrap();
        assert_eq!(StackItem::Number(42.0, 10), x.clone());
    }
}
