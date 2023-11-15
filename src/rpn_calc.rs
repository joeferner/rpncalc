use std::collections::HashMap;
use std::f64::consts::PI;
use std::rc::Rc;
use crate::stack::Stack;
use crate::stack_item::{NumberType, StackItem};
use thiserror::Error;
use crate::function::Function;
use crate::functions;
use crate::functions::add::Add;
use crate::functions::cosine::Cosine;
use crate::functions::divide::Divide;
use crate::functions::multiply::Multiply;
use crate::functions::pow::Pow;
use crate::functions::sine::Sine;
use crate::functions::square_root::SquareRoot;
use crate::functions::subtract::Subtract;
use crate::functions::tangent::Tangent;
use crate::rpn_calc::AngleMode::Degrees;

#[derive(Copy, Clone)]
pub enum AngleMode {
    Radians,
    Degrees,
}

#[derive(Debug, Error)]
pub enum RpnCalcError {
    #[error("parse stack item: {0}")]
    ParseStackItem(String),
    #[error("not enough arguments")]
    NotEnoughArguments,
    #[error("invalid argument {0}")]
    InvalidArgument(String),
    #[error("IO error: {0}")]
    StdIoError(#[from] std::io::Error),
}

pub struct RpnCalc {
    stack: Stack,
    angle_mode: AngleMode,
    functions: HashMap<String, Rc<dyn Function>>,
}

impl RpnCalc {
    pub fn new() -> Self {
        let mut functions: HashMap<String, Rc<dyn Function>> = HashMap::new();
        functions.insert("add".to_string(), Rc::new(Add::new()));
        functions.insert("+".to_string(), Rc::new(Add::new()));

        functions.insert("subtract".to_string(), Rc::new(Subtract::new()));
        functions.insert("sub".to_string(), Rc::new(Subtract::new()));
        functions.insert("-".to_string(), Rc::new(Subtract::new()));

        functions.insert("multiply".to_string(), Rc::new(Multiply::new()));
        functions.insert("mul".to_string(), Rc::new(Multiply::new()));
        functions.insert("*".to_string(), Rc::new(Multiply::new()));

        functions.insert("divide".to_string(), Rc::new(Divide::new()));
        functions.insert("div".to_string(), Rc::new(Divide::new()));
        functions.insert("/".to_string(), Rc::new(Divide::new()));

        functions.insert("pow".to_string(), Rc::new(Pow::new()));
        functions.insert("^".to_string(), Rc::new(Pow::new()));

        functions.insert("sin".to_string(), Rc::new(Sine::new()));
        functions.insert("sine".to_string(), Rc::new(Sine::new()));

        functions.insert("cos".to_string(), Rc::new(Cosine::new()));
        functions.insert("cosine".to_string(), Rc::new(Cosine::new()));

        functions.insert("tan".to_string(), Rc::new(Tangent::new()));
        functions.insert("tangent".to_string(), Rc::new(Tangent::new()));

        functions.insert("deg".to_string(), Rc::new(functions::degrees::Degrees::new()));
        functions.insert("degrees".to_string(), Rc::new(functions::degrees::Degrees::new()));

        functions.insert("rad".to_string(), Rc::new(functions::radians::Radians::new()));
        functions.insert("radians".to_string(), Rc::new(functions::radians::Radians::new()));

        functions.insert("sqrt".to_string(), Rc::new(SquareRoot::new()));
        functions.insert("drop".to_string(), Rc::new(functions::drop::Drop::new()));

        return RpnCalc {
            stack: Stack::new(),
            angle_mode: Degrees,
            functions,
        };
    }

    /// pushes a string onto the stack, first parsing it into a stack item. If the string results
    /// in a operator, the operator will be executed.
    pub fn push_str(&mut self, str: &str) -> Result<(), RpnCalcError> {
        let stack_item = self.parse_string_to_stack_item(str)?;
        match stack_item {
            StackItem::Function(func) => {
                func.apply(self)?;
            }
            _ => self.stack.push(stack_item),
        }
        return Ok(());
    }

    pub fn push(&mut self, stack_item: StackItem) -> () {
        self.stack.push(stack_item);
    }

    fn parse_string_to_stack_item(&self, str: &str) -> Result<StackItem, RpnCalcError> {
        if let Ok(n) = str.parse::<f64>() {
            return Ok(StackItem::Number(n));
        }
        if let Ok(func) = self.parse_string_to_function(str) {
            return Ok(StackItem::Function(func));
        }
        if let Ok(s) = RpnCalc::parse_string_to_string_constant(str) {
            return Ok(StackItem::String(s));
        }
        return Err(RpnCalcError::ParseStackItem(str.to_string()));
    }

    fn parse_string_to_function(&self, str: &str) -> Result<Rc<dyn Function>, RpnCalcError> {
        let func = self.functions.get(str);
        if let Some(func) = func {
            return Ok(func.clone());
        }
        return Err(RpnCalcError::ParseStackItem(str.to_string()));
    }

    fn parse_string_to_string_constant(str: &str) -> Result<String, RpnCalcError> {
        let str = str.to_string();
        if str.starts_with("`") && str.ends_with("`") {
            return Ok(str[1..str.len() - 1].to_string());
        }
        return Err(RpnCalcError::ParseStackItem(str));
    }

    #[cfg(test)]
    pub fn pop_number(&mut self) -> Result<Option<NumberType>, RpnCalcError> {
        let opt_stack_item = self.stack.pop();
        return match opt_stack_item {
            None => Ok(None),
            Some(stack_item) => {
                let n = self.stack_item_to_number(stack_item)?;
                Ok(Some(n))
            }
        };
    }

    pub fn format_stack_item(&self, stack_item: &StackItem) -> String {
        // TODO format based on modes
        return format!("{}", stack_item);
    }

    pub fn pop(&mut self) -> Result<(), RpnCalcError> {
        if self.stack.pop().is_some() {
            return Ok(());
        }
        return Err(RpnCalcError::NotEnoughArguments);
    }

    pub fn stack(&self) -> &Stack {
        return &self.stack;
    }

    pub fn angle_mode(&self) -> AngleMode { return self.angle_mode; }

    pub fn set_angle_mode(&mut self, angle_mode: AngleMode) -> () {
        self.angle_mode = angle_mode;
    }

    pub fn get_binary_number_operator_args(
        &mut self,
    ) -> Result<(NumberType, NumberType), RpnCalcError> {
        let a = self.pop_number_stack_item()?;
        let b = self.pop_number_stack_item();
        if let Err(err) = b {
            self.stack.push(a);
            return Err(err);
        }
        let b = b.unwrap();

        let a = self.stack_item_to_number(a)?;
        let b = self.stack_item_to_number(b)?;
        return Ok((a, b));
    }

    pub fn get_unary_number_operator_arg_radians(&mut self) -> Result<NumberType, RpnCalcError> {
        let a = self.get_unary_number_operator_arg()?;
        return match self.angle_mode {
            AngleMode::Radians => Ok(a),
            AngleMode::Degrees => Ok(a * PI / 180.0)
        };
    }

    pub fn get_unary_number_operator_arg(&mut self) -> Result<NumberType, RpnCalcError> {
        let a = self.pop_number_stack_item()?;
        let a = self.stack_item_to_number(a)?;
        return Ok(a);
    }

    fn pop_number_stack_item(&mut self) -> Result<StackItem, RpnCalcError> {
        let stack_item = self.stack.pop().ok_or(RpnCalcError::NotEnoughArguments)?;
        return match stack_item {
            StackItem::Number(a) => Ok(StackItem::Number(a)),
            _ => {
                let msg = format!("expected number found: {}", stack_item);
                self.stack.push(stack_item);
                Err(RpnCalcError::InvalidArgument(msg))
            }
        };
    }

    fn stack_item_to_number(&self, stack_item: StackItem) -> Result<NumberType, RpnCalcError> {
        return match stack_item {
            StackItem::Number(a) => Ok(a),
            _ => {
                let msg = format!("expected number found: {}", stack_item);
                Err(RpnCalcError::InvalidArgument(msg))
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    fn run(args: Vec<&str>) -> RpnCalc {
        let mut rpn_calc = RpnCalc::new();
        for arg in args {
            rpn_calc.push_str(arg).unwrap();
        }
        return rpn_calc;
    }

    fn run_binary_operator(arg1: &str, arg2: &str, op: &str, expected: NumberType) {
        let mut rpn_calc = run(vec![arg1, arg2, op]);
        assert_relative_eq!(expected, rpn_calc.pop_number().unwrap().unwrap());
    }

    fn run_unary_operator(arg: &str, op: &str, expected: NumberType) {
        let mut rpn_calc = run(vec![arg, op]);
        assert_relative_eq!(expected, rpn_calc.pop_number().unwrap().unwrap());
    }

    fn run_unary_operator_deg(arg: &str, op: &str, expected: NumberType) {
        let mut rpn_calc = run(vec!["deg", arg, op]);
        assert_relative_eq!(expected, rpn_calc.pop_number().unwrap().unwrap());
    }

    fn run_unary_operator_rad(arg: &str, op: &str, expected: NumberType) {
        let mut rpn_calc = run(vec!["rad", arg, op]);
        assert_relative_eq!(expected, rpn_calc.pop_number().unwrap().unwrap());
    }

    #[test]
    fn test_add() {
        run_binary_operator("1.2", "5.6", "+", 6.8);
    }

    #[test]
    fn test_subtract() {
        run_binary_operator("1.2", "0.8", "-", 0.4);
    }

    #[test]
    fn test_multiply() {
        run_binary_operator("1.2", "0.8", "*", 0.96);
    }

    #[test]
    fn test_divide() {
        run_binary_operator("1.2", "0.8", "/", 1.5);
    }

    #[test]
    fn test_pow() {
        run_binary_operator("3.2", "2", "^", 10.24);
    }

    #[test]
    fn test_sqrt() {
        run_unary_operator("10.24", "sqrt", 3.2);
    }

    #[test]
    fn test_sin_deg() {
        run_unary_operator_deg("10", "sin", 0.17364817766693033);
    }

    #[test]
    fn test_sin_rad() {
        run_unary_operator_rad("0.34", "sin", 0.3334870921408144);
    }

    #[test]
    fn test_cos() {
        run_unary_operator_deg("10", "cos", 0.984807753012208);
    }

    #[test]
    fn test_tan() {
        run_unary_operator_deg("10", "tan", 0.17632698070846498);
    }
}
