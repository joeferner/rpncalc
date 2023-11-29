use crate::error::RpnCalcError;
use crate::functions;
use crate::functions::Category;
use crate::rpn_calc::RpnCalc;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

pub trait Function: fmt::Display {
    fn apply(&self, rpn_calc: &mut RpnCalc) -> Result<(), RpnCalcError>;

    fn get_help(&self) -> String;

    fn get_category(&self) -> Category;
}

pub fn get_functions() -> HashMap<String, Rc<dyn Function>> {
    let mut functions: HashMap<String, Rc<dyn Function>> = HashMap::new();

    // arithmetic
    let add = Rc::new(functions::arithmetic::Add::new());
    functions.insert("add".to_string(), add.clone());
    functions.insert("+".to_string(), add.clone());

    let subtract = Rc::new(functions::arithmetic::Subtract::new());
    functions.insert("sub".to_string(), subtract.clone());
    functions.insert("-".to_string(), subtract.clone());

    let multiply = Rc::new(functions::arithmetic::Multiply::new());
    functions.insert("mul".to_string(), multiply.clone());
    functions.insert("*".to_string(), multiply.clone());

    let divide = Rc::new(functions::arithmetic::Divide::new());
    functions.insert("div".to_string(), divide.clone());
    functions.insert("/".to_string(), divide.clone());

    let pow = Rc::new(functions::arithmetic::Pow::new());
    functions.insert("pow".to_string(), pow.clone());
    functions.insert("^".to_string(), pow.clone());

    let negate = Rc::new(functions::arithmetic::Negate::new());
    functions.insert("neg".to_string(), negate.clone());
    functions.insert("_".to_string(), negate.clone());

    functions.insert("sqrt".to_string(), Rc::new(functions::arithmetic::SquareRoot::new()));

    // base
    functions.insert("bin".to_string(), Rc::new(functions::base::Binary::new()));
    functions.insert("oct".to_string(), Rc::new(functions::base::Octal::new()));
    functions.insert("dec".to_string(), Rc::new(functions::base::Decimal::new()));
    functions.insert("hex".to_string(), Rc::new(functions::base::Hexidecimal::new()));

    // stack
    functions.insert("drop".to_string(), Rc::new(functions::stack::Drop::new()));
    functions.insert("dup".to_string(), Rc::new(functions::stack::Duplicate::new()));

    // trig
    functions.insert("sin".to_string(), Rc::new(functions::trig::Sin::new()));
    functions.insert("cos".to_string(), Rc::new(functions::trig::Cos::new()));
    functions.insert("tan".to_string(), Rc::new(functions::trig::Tan::new()));
    functions.insert("deg".to_string(), Rc::new(functions::trig::Degrees::new()));
    functions.insert("rad".to_string(), Rc::new(functions::trig::Radians::new()));

    return functions;
}
