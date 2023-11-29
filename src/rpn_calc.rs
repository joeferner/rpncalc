use crate::constant::{get_constants, Constant};
use crate::error::RpnCalcError;
use crate::functions::{get_functions, Function};
use crate::number::Number;
use crate::stack::Stack;
use crate::stack_item::StackItem;
use crate::units::AngleUnits;
use std::collections::HashMap;
use std::rc::Rc;

pub struct RpnCalc {
    pub stack: Stack,
    pub angle_mode: AngleUnits,
    pub base: u16,
    pub functions: HashMap<String, Rc<dyn Function>>,
    pub constants: HashMap<String, Rc<Constant>>,
}

impl RpnCalc {
    pub fn new() -> Self {
        return RpnCalc {
            stack: Stack::new(),
            angle_mode: AngleUnits::Degrees,
            base: 10,
            functions: get_functions(),
            constants: get_constants(),
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
        if let Ok(n) = Number::from_str(str) {
            return Ok(StackItem::Number(n));
        }
        if let Ok(func) = self.parse_string_to_function(str) {
            return Ok(StackItem::Function(func));
        }
        if let Ok(c) = self.parse_string_to_constant(str) {
            return Ok(StackItem::Number(c.get_value()));
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

    fn parse_string_to_constant(&self, str: &str) -> Result<Rc<Constant>, RpnCalcError> {
        let c = self.constants.get(str);
        if let Some(c) = c {
            return Ok(c.clone());
        }
        return Err(RpnCalcError::ParseStackItem(str.to_string()));
    }

    fn parse_string_to_string_constant(str: &str) -> Result<String, RpnCalcError> {
        let str = str.to_string();
        if str.starts_with('`') && str.ends_with('`') {
            return Ok(str[1..str.len() - 1].to_string());
        }
        return Err(RpnCalcError::ParseStackItem(str));
    }

    #[cfg(test)]
    pub fn pop_number(&mut self) -> Result<Option<Number>, RpnCalcError> {
        let opt_stack_item = self.stack.pop();
        return match opt_stack_item {
            None => Ok(None),
            Some(stack_item) => {
                let n = self.stack_item_to_number(&stack_item)?;
                Ok(Some(n))
            }
        };
    }

    pub fn format_stack_item(&self, stack_item: &StackItem) -> String {
        // TODO format based on modes
        return format!("{}", stack_item);
    }

    pub fn pop(&mut self) -> Result<StackItem, RpnCalcError> {
        if let Some(stack_item) = self.stack.pop() {
            return Ok(stack_item);
        }
        return Err(RpnCalcError::NotEnoughArguments);
    }

    pub fn execute_binary_number_operator<F>(&mut self, f: F) -> Result<(), RpnCalcError>
    where
        F: FnOnce(&mut RpnCalc, Number, Number) -> Result<(), RpnCalcError>,
    {
        let a_stack_item = self.pop_number_stack_item()?;
        let b_stack_item = match self.pop_number_stack_item() {
            Ok(b_stack_item) => b_stack_item,
            Err(err) => {
                self.stack.push(a_stack_item);
                return Err(err);
            }
        };

        let a = match self.stack_item_to_number(&a_stack_item) {
            Ok(a) => a,
            Err(err) => {
                self.stack.push(b_stack_item);
                self.stack.push(a_stack_item);
                return Err(err);
            }
        };

        let b = match self.stack_item_to_number(&b_stack_item) {
            Ok(b) => b,
            Err(err) => {
                self.stack.push(b_stack_item);
                self.stack.push(a_stack_item);
                return Err(err);
            }
        };

        if let Err(err) = f(self, b, a) {
            self.stack.push(b_stack_item);
            self.stack.push(a_stack_item);
            return Err(err);
        }

        return Ok(());
    }

    pub fn execute_unary_number_operator<F>(&mut self, f: F) -> Result<(), RpnCalcError>
    where
        F: FnOnce(&mut RpnCalc, Number) -> Result<(), RpnCalcError>,
    {
        let a_stack_item = self.pop_number_stack_item()?;
        let a = match self.stack_item_to_number(&a_stack_item) {
            Ok(a) => a,
            Err(err) => {
                self.stack.push(a_stack_item);
                return Err(err);
            }
        };

        if let Err(err) = f(self, a) {
            self.stack.push(a_stack_item);
            return Err(err);
        }

        return Ok(());
    }

    fn pop_number_stack_item(&mut self) -> Result<StackItem, RpnCalcError> {
        let stack_item = self.stack.pop().ok_or(RpnCalcError::NotEnoughArguments)?;
        return match stack_item {
            StackItem::Number(a) => Ok(StackItem::Number(a)),
            _ => {
                let msg = format!("expected number but found \"{}\"", stack_item);
                self.stack.push(stack_item);
                Err(RpnCalcError::InvalidArgument(msg))
            }
        };
    }

    fn stack_item_to_number(&self, stack_item: &StackItem) -> Result<Number, RpnCalcError> {
        return match stack_item {
            StackItem::Number(a) => Ok(a.clone()),
            _ => {
                let msg = format!("expected number but found \"{}\"", stack_item);
                Err(RpnCalcError::InvalidArgument(msg))
            }
        };
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::units::LengthUnits;
    use crate::units::SIPrefix;
    use crate::units::TimeUnits;
    use crate::units::Units;
    use approx::assert_relative_eq;

    pub fn run(args: Vec<&str>) -> RpnCalc {
        let mut rpn_calc = RpnCalc::new();
        for arg in args {
            rpn_calc.push_str(arg).unwrap();
        }
        return rpn_calc;
    }

    pub fn assert_stack(rpn_calc: &RpnCalc, args: Vec<&str>) -> () {
        assert_eq!(args.len(), rpn_calc.stack.items.len());
        for (i, arg) in args.iter().enumerate() {
            let found_stack_item = rpn_calc.stack.items.get(i).unwrap();
            let expected_stack_item = rpn_calc.parse_string_to_stack_item(arg).unwrap();
            assert_eq!(expected_stack_item, found_stack_item.clone());
        }
    }

    pub fn run_binary_operator(arg1: &str, arg2: &str, op: &str, expected: Number) {
        let mut rpn_calc = run(vec![arg1, arg2, op]);
        assert_relative_eq!(expected.magnitude, rpn_calc.pop_number().unwrap().unwrap().magnitude);
    }

    pub fn run_unary_operator(arg: &str, op: &str, expected: Number) {
        let mut rpn_calc = run(vec![arg, op]);
        assert_relative_eq!(expected.magnitude, rpn_calc.pop_number().unwrap().unwrap().magnitude);
    }

    pub fn run_unary_operator_deg(arg: &str, op: &str, expected: Number) {
        let mut rpn_calc = run(vec!["deg", arg, op]);
        assert_relative_eq!(expected.magnitude, rpn_calc.pop_number().unwrap().unwrap().magnitude);
    }

    pub fn run_unary_operator_rad(arg: &str, op: &str, expected: Number) {
        let mut rpn_calc = run(vec!["rad", arg, op]);
        assert_relative_eq!(expected.magnitude, rpn_calc.pop_number().unwrap().unwrap().magnitude);
    }

    fn assert_run(args: Vec<&str>, expected: Vec<&str>) {
        let rpn_calc = run(args);
        assert_eq!(expected.len(), rpn_calc.stack.items.len(), "unexpected stack length");
        for (i, expected_stack_item_str) in expected.iter().enumerate() {
            let found = format!("{}", rpn_calc.stack.items[i]);
            assert_eq!(
                expected_stack_item_str.to_string(),
                found,
                "stack item mismatch at {}",
                i
            );
        }
    }

    fn assert_error(args: Vec<&str>, expected_err: RpnCalcError) -> RpnCalc {
        let mut rpn_calc = RpnCalc::new();
        for arg in args.iter().take(args.len() - 1) {
            rpn_calc.push_str(arg).unwrap();
        }
        if let Err(err) = rpn_calc.push_str(args[args.len() - 1]) {
            assert_eq!(err, expected_err, "error mismatch");
        } else {
            assert!(false, "expected error");
        }
        return rpn_calc;
    }

    #[test]
    fn test_error_not_enough_args() {
        let rpn_calc = assert_error(vec!["1", "+"], RpnCalcError::NotEnoughArguments);
        assert_stack(&rpn_calc, vec!["1"]);
    }

    #[test]
    fn test_error_first_arg() {
        let rpn_calc = assert_error(
            vec!["`a`", "1", "+"],
            RpnCalcError::InvalidArgument("expected number but found \"`a`\"".to_string()),
        );
        assert_stack(&rpn_calc, vec!["`a`", "1"]);
    }

    #[test]
    fn test_error_second_arg() {
        let rpn_calc = assert_error(
            vec!["1", "`a`", "+"],
            RpnCalcError::InvalidArgument("expected number but found \"`a`\"".to_string()),
        );
        assert_stack(&rpn_calc, vec!["1", "`a`"]);
    }

    #[test]
    fn test_add_units() {
        assert_run(vec!["1ft", "0m", "+"], vec!["0.3048 m"]);
    }

    #[test]
    fn test_add_units_ft_none() {
        assert_run(vec!["1ft", "1", "+"], vec!["2 ft"]);
    }

    #[test]
    fn test_add_units_none_ft() {
        assert_run(vec!["1", "1ft", "+"], vec!["2 ft"]);
    }

    #[test]
    fn test_add_incompatible_units() {
        assert_error(
            vec!["1ft", "1s", "+"],
            RpnCalcError::IncompatibleUnits(
                Units::Length(LengthUnits::Foot),
                Units::Time(TimeUnits::Second(SIPrefix::None)),
            ),
        );
    }

    #[test]
    fn test_divide_by_zero() {
        assert_error(vec!["5", "0", "/"], RpnCalcError::DivideByZero);
        assert_error(vec!["0", "0", "/"], RpnCalcError::DivideByZero);
    }
}
