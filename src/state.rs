use std::collections::HashMap;

use crate::{
    func::{add::AddFunc, Func},
    stack::Stack,
};

pub struct RpnState {
    pub stack: Stack,
    pub functions: HashMap<String, Box<dyn Func>>,
}

impl RpnState {
    pub fn new() -> Self {
        let mut functions: HashMap<String, Box<dyn Func>> = HashMap::new();
        functions.insert("add".to_string(), Box::new(AddFunc::new()));

        Self {
            stack: Stack::new(),
            functions,
        }
    }
}
