use std::f64::consts::PI;
use crate::operator::{Operator, ParseOperatorError};
use crate::stack::Stack;
use crate::stack_item::{NumberType, ParseStackItemError, StackItem};
use thiserror::Error;

pub enum AngleMode {
    Radians,
    Degrees,
}

pub struct RpnCalc {
    stack: Stack,
    angle_mode: AngleMode,
}

#[derive(Debug, Error)]
pub enum RpnCalcError {
    #[error("parse stack item: {0}")]
    ParseStackItem(#[from] ParseStackItemError),
    #[error("parse operator: {0}")]
    ParseOperator(#[from] ParseOperatorError),
    #[error("not enough arguments")]
    NotEnoughArguments,
    #[error("invalid argument {0}")]
    InvalidArgument(String),
}

impl RpnCalc {
    pub fn new() -> Self {
        return RpnCalc {
            stack: Stack::new(),
            angle_mode: AngleMode::Degrees,
        };
    }

    /// pushes a string onto the stack, first parsing it into a stack item. If the string results
    /// in a operator, the operator will be executed.
    pub fn push_str(&mut self, str: &str) -> Result<(), RpnCalcError> {
        let stack_item = str.parse::<StackItem>()?;
        match stack_item {
            StackItem::Operator(op) => {
                self.apply_operator(op)?;
            }
            _ => self.stack.push(stack_item),
        }
        return Ok(());
    }

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

    pub fn stack(&self) -> &Stack {
        return &self.stack;
    }

    fn apply_operator(&mut self, op: Operator) -> Result<(), RpnCalcError> {
        return match op {
            Operator::Degrees => self.apply_operator_degrees(),
            Operator::Radians => self.apply_operator_radians(),
            Operator::Add => self.apply_operator_add(),
            Operator::Subtract => self.apply_operator_subtract(),
            Operator::Multiply => self.apply_operator_multiply(),
            Operator::Divide => self.apply_operator_divide(),
            Operator::Pow => self.apply_operator_pow(),
            Operator::SquareRoot => self.apply_operator_square_root(),
            Operator::Sine => self.apply_operator_sine(),
            Operator::Cosine => self.apply_operator_cosine(),
            Operator::Tangent => self.apply_operator_tangent(),
        };
    }

    fn apply_operator_degrees(&mut self) -> Result<(), RpnCalcError> {
        self.angle_mode = AngleMode::Degrees;
        return Ok(());
    }

    fn apply_operator_radians(&mut self) -> Result<(), RpnCalcError> {
        self.angle_mode = AngleMode::Radians;
        return Ok(());
    }

    fn apply_operator_add(&mut self) -> Result<(), RpnCalcError> {
        let args = self.get_binary_number_operator_args()?;
        let result = args.0 + args.1;
        self.stack.push(StackItem::Number(result));
        return Ok(());
    }

    fn apply_operator_subtract(&mut self) -> Result<(), RpnCalcError> {
        let args = self.get_binary_number_operator_args()?;
        let result = args.1 - args.0;
        self.stack.push(StackItem::Number(result));
        return Ok(());
    }

    fn apply_operator_multiply(&mut self) -> Result<(), RpnCalcError> {
        let args = self.get_binary_number_operator_args()?;
        let result = args.1 * args.0;
        self.stack.push(StackItem::Number(result));
        return Ok(());
    }

    fn apply_operator_divide(&mut self) -> Result<(), RpnCalcError> {
        let args = self.get_binary_number_operator_args()?;
        let result = args.1 / args.0;
        self.stack.push(StackItem::Number(result));
        return Ok(());
    }

    fn apply_operator_pow(&mut self) -> Result<(), RpnCalcError> {
        let args = self.get_binary_number_operator_args()?;
        let result = args.1.powf(args.0);
        self.stack.push(StackItem::Number(result));
        return Ok(());
    }

    fn apply_operator_square_root(&mut self) -> Result<(), RpnCalcError> {
        let arg = self.get_unary_number_operator_arg()?;
        let result = arg.powf(0.5);
        self.stack.push(StackItem::Number(result));
        return Ok(());
    }

    fn apply_operator_sine(&mut self) -> Result<(), RpnCalcError> {
        let arg = self.get_unary_number_operator_arg_radians()?;
        let result = arg.sin();
        self.stack.push(StackItem::Number(result));
        return Ok(());
    }

    fn apply_operator_cosine(&mut self) -> Result<(), RpnCalcError> {
        let arg = self.get_unary_number_operator_arg_radians()?;
        let result = arg.cos();
        self.stack.push(StackItem::Number(result));
        return Ok(());
    }

    fn apply_operator_tangent(&mut self) -> Result<(), RpnCalcError> {
        let arg = self.get_unary_number_operator_arg_radians()?;
        let result = arg.tan();
        self.stack.push(StackItem::Number(result));
        return Ok(());
    }

    fn get_binary_number_operator_args(
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

    fn get_unary_number_operator_arg_radians(&mut self) -> Result<NumberType, RpnCalcError> {
        let a = self.get_unary_number_operator_arg()?;
        return match self.angle_mode {
            AngleMode::Radians => Ok(a),
            AngleMode::Degrees => Ok(a * PI / 180.0)
        };
    }

    fn get_unary_number_operator_arg(&mut self) -> Result<NumberType, RpnCalcError> {
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
