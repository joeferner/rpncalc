use core::f64;
use std::{collections::HashMap, sync::Arc};

use angle_mode::AngleMode;
use anyhow::{anyhow, Error, Result};
use constant::Constant;
use input::Input;
use num_format::SystemLocale;
use ratatui::widgets::ListState;

use crate::{
    func::{
        basic::{add::AddFunc, divide::DivideFunc, multiply::MultiplyFunc, subtract::SubtractFunc},
        trig::{
            cos::CosFunc, degrees::DegreesFunc, radians::RadiansFunc, sin::SinFunc, tan::TanFunc,
        },
        Func,
    },
    stack::{item::StackItem, Stack},
    undo_action::{pop::PopUndoEvent, push::PushUndoEvent},
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

        functions.insert("rad".to_string(), Arc::new(Box::new(RadiansFunc::new())));
        functions.insert("deg".to_string(), Arc::new(Box::new(DegreesFunc::new())));
        functions.insert("sin".to_string(), Arc::new(Box::new(SinFunc::new())));
        functions.insert("cos".to_string(), Arc::new(Box::new(CosFunc::new())));
        functions.insert("tan".to_string(), Arc::new(Box::new(TanFunc::new())));

        let mut constants: HashMap<String, Arc<Constant>> = HashMap::new();
        constants.insert("pi".to_string(), Arc::new(Constant::new(f64::consts::PI)));
        constants.insert("e".to_string(), Arc::new(Constant::new(f64::consts::E)));
        constants.insert("tau".to_string(), Arc::new(Constant::new(f64::consts::TAU)));

        Ok(Self {
            locale: SystemLocale::default()?,
            stack: Stack::new(),
            angle_mode: AngleMode::Degrees,
            functions,
            constants,
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
