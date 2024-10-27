use std::{collections::HashMap, sync::Arc};

use add::AddFunc;
use divide::DivideFunc;
use modulus::ModulusFunc;
use multiply::MultiplyFunc;
use subtract::SubtractFunc;

use super::Func;

pub mod add;
pub mod divide;
pub mod modulus;
pub mod multiply;
pub mod subtract;

pub fn basic_register_functions(functions: &mut HashMap<String, Arc<Box<dyn Func>>>) {
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

    let modulus_func: Arc<Box<dyn Func>> = Arc::new(Box::new(ModulusFunc::new()));
    functions.insert("mod".to_string(), modulus_func.clone());
    functions.insert("%".to_string(), modulus_func);
}
