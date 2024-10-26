use std::{collections::HashMap, sync::Arc};

use store::StoreFunc;

use super::Func;

pub mod store;

pub fn variable_register_functions(functions: &mut HashMap<String, Arc<Box<dyn Func>>>) {
    functions.insert("store".to_string(), Arc::new(Box::new(StoreFunc::new())));
}
