use std::collections::HashMap;
use std::rc::Rc;
use crate::units::angle::AngleUnits;
use crate::error::RpnCalcError;
use crate::stack::Stack;
use crate::stack_item::{StackItem};
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
use crate::number::Number;

pub struct RpnCalc {
    stack: Stack,
    angle_mode: AngleUnits,
    functions: HashMap<String, Rc<dyn Function>>,
}

impl RpnCalc {
    pub fn new() -> Self {
        let mut functions: HashMap<String, Rc<dyn Function>> = HashMap::new();

        let add = Rc::new(Add::new());
        functions.insert("add".to_string(), add.clone());
        functions.insert("+".to_string(), add.clone());

        let subtract = Rc::new(Subtract::new());
        functions.insert("subtract".to_string(), subtract.clone());
        functions.insert("sub".to_string(), subtract.clone());
        functions.insert("-".to_string(), subtract.clone());

        let multiply = Rc::new(Multiply::new());
        functions.insert("multiply".to_string(), multiply.clone());
        functions.insert("mul".to_string(), multiply.clone());
        functions.insert("*".to_string(), multiply.clone());

        let divide = Rc::new(Divide::new());
        functions.insert("divide".to_string(), divide.clone());
        functions.insert("div".to_string(), divide.clone());
        functions.insert("/".to_string(), divide.clone());

        let pow = Rc::new(Pow::new());
        functions.insert("pow".to_string(), pow.clone());
        functions.insert("^".to_string(), pow.clone());

        let sine = Rc::new(Sine::new());
        functions.insert("sin".to_string(), sine.clone());
        functions.insert("sine".to_string(), sine.clone());

        let cosine = Rc::new(Cosine::new());
        functions.insert("cos".to_string(), cosine.clone());
        functions.insert("cosine".to_string(), cosine.clone());

        let tangent = Rc::new(Tangent::new());
        functions.insert("tan".to_string(), tangent.clone());
        functions.insert("tangent".to_string(), tangent.clone());

        let degrees = Rc::new(functions::degrees::Degrees::new());
        functions.insert("deg".to_string(), degrees.clone());
        functions.insert("degrees".to_string(), degrees.clone());

        let radians = Rc::new(functions::radians::Radians::new());
        functions.insert("rad".to_string(), radians.clone());
        functions.insert("radians".to_string(), radians.clone());

        functions.insert("sqrt".to_string(), Rc::new(SquareRoot::new()));
        functions.insert("drop".to_string(), Rc::new(functions::drop::Drop::new()));

        return RpnCalc {
            stack: Stack::new(),
            angle_mode: AngleUnits::Degrees,
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
        if let Ok(n) = Number::from_str(str) {
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

    pub fn stack(&self) -> &Stack {
        return &self.stack;
    }

    pub fn angle_mode(&self) -> AngleUnits { return self.angle_mode; }

    pub fn set_angle_mode(&mut self, angle_mode: AngleUnits) -> () {
        self.angle_mode = angle_mode;
    }

    pub fn execute_binary_number_operator<F>(&mut self, f: F) -> Result<(), RpnCalcError>
        where F: FnOnce(&mut RpnCalc, Number, Number) -> Result<(), RpnCalcError> {
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
        where F: FnOnce(&mut RpnCalc, Number) -> Result<(), RpnCalcError> {
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
                let msg = format!("expected number found: {}", stack_item);
                self.stack.push(stack_item);
                Err(RpnCalcError::InvalidArgument(msg))
            }
        };
    }

    fn stack_item_to_number(&self, stack_item: &StackItem) -> Result<Number, RpnCalcError> {
        return match stack_item {
            StackItem::Number(a) => Ok(a.clone()),
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
    use crate::units::Units;
    use crate::units::length::LengthUnits;
    use crate::units::si_prefix::SIPrefix;

    fn run(args: Vec<&str>) -> RpnCalc {
        let mut rpn_calc = RpnCalc::new();
        for arg in args {
            rpn_calc.push_str(arg).unwrap();
        }
        return rpn_calc;
    }

    fn assert_stack(rpn_calc: &RpnCalc, args: Vec<&str>) -> () {
        assert_eq!(args.len(), rpn_calc.stack.items().len());
        for (i, arg) in args.iter().enumerate() {
            let found_stack_item = rpn_calc.stack.items().get(i).unwrap();
            let expected_stack_item = rpn_calc.parse_string_to_stack_item(arg).unwrap();
            assert_eq!(expected_stack_item, found_stack_item.clone());
        }
    }

    fn run_binary_operator(arg1: &str, arg2: &str, op: &str, expected: Number) {
        let mut rpn_calc = run(vec![arg1, arg2, op]);
        assert_relative_eq!(expected.magnitude, rpn_calc.pop_number().unwrap().unwrap().magnitude);
    }

    fn run_unary_operator(arg: &str, op: &str, expected: Number) {
        let mut rpn_calc = run(vec![arg, op]);
        assert_relative_eq!(expected.magnitude, rpn_calc.pop_number().unwrap().unwrap().magnitude);
    }

    fn run_unary_operator_deg(arg: &str, op: &str, expected: Number) {
        let mut rpn_calc = run(vec!["deg", arg, op]);
        assert_relative_eq!(expected.magnitude, rpn_calc.pop_number().unwrap().unwrap().magnitude);
    }

    fn run_unary_operator_rad(arg: &str, op: &str, expected: Number) {
        let mut rpn_calc = run(vec!["rad", arg, op]);
        assert_relative_eq!(expected.magnitude, rpn_calc.pop_number().unwrap().unwrap().magnitude);
    }

    #[test]
    fn test_add() {
        run_binary_operator("1.2", "5.6", "+", Number::from(6.8));
    }

    #[test]
    fn test_subtract() {
        run_binary_operator("1.2", "0.8", "-", Number::from(0.4));
    }

    #[test]
    fn test_multiply() {
        run_binary_operator("1.2", "0.8", "*", Number::from(0.96));
    }

    #[test]
    fn test_divide() {
        run_binary_operator("1.2", "0.8", "/", Number::from(1.5));
    }

    #[test]
    fn test_pow() {
        run_binary_operator("3.2", "2", "^", Number::from(10.24));
    }

    #[test]
    fn test_sqrt() {
        run_unary_operator("10.24", "sqrt", Number::from(3.2));
    }

    #[test]
    fn test_sin_deg() {
        run_unary_operator_deg("10", "sin", Number::from(0.17364817766693033));
    }

    #[test]
    fn test_sin_rad() {
        run_unary_operator_rad("0.34", "sin", Number::from(0.3334870921408144));
    }

    #[test]
    fn test_sin_units_deg() {
        run_unary_operator_rad("10 deg", "sin", Number::from(0.17364817766693033));
    }

    #[test]
    fn test_sin_units_rad() {
        run_unary_operator_deg("0.34 rad", "sin", Number::from(0.3334870921408144));
    }

    #[test]
    fn test_cos() {
        run_unary_operator_deg("10", "cos", Number::from(0.984807753012208));
    }

    #[test]
    fn test_tan() {
        run_unary_operator_deg("10", "tan", Number::from(0.17632698070846498));
    }

    #[test]
    fn test_error_not_enough_args() {
        let mut rpn_calc = run(vec!["1"]);
        let result = rpn_calc.push_str("add");
        assert!(matches!(result, Err(RpnCalcError::NotEnoughArguments)));
        assert_stack(&rpn_calc, vec!["1"]);
    }

    #[test]
    fn test_error_first_arg() {
        let mut rpn_calc = run(vec!["`a`", "1"]);
        let result = rpn_calc.push_str("add");
        assert!(matches!(result, Err(RpnCalcError::InvalidArgument(_))));
        assert_stack(&rpn_calc, vec!["`a`", "1"]);
    }

    #[test]
    fn test_error_second_arg() {
        let mut rpn_calc = run(vec!["1", "`a`"]);
        let result = rpn_calc.push_str("add");
        assert!(matches!(result, Err(RpnCalcError::InvalidArgument(_))));
        assert_stack(&rpn_calc, vec!["1", "`a`"]);
    }

    #[test]
    fn test_add_units() {
        let mut rpn_calc = run(vec!["1ft", "0m", "+"]);
        assert_eq!(1, rpn_calc.stack.items().len());
        let stack_item = rpn_calc.pop().unwrap();
        match stack_item {
            StackItem::Number(n) => {
                assert_relative_eq!(0.3048, n.magnitude);
                assert!(matches!(n.units, Units::Length(LengthUnits::Meter(SIPrefix::None))));
            }
            _ => assert!(false)
        }
    }

    #[test]
    fn test_add_units_ft_none() {
        let mut rpn_calc = run(vec!["1ft", "1", "+"]);
        assert_eq!(1, rpn_calc.stack.items().len());
        let stack_item = rpn_calc.pop().unwrap();
        match stack_item {
            StackItem::Number(n) => {
                assert_relative_eq!(2.0, n.magnitude);
                assert!(matches!(n.units,Units::Length(LengthUnits::Feet)));
            }
            _ => assert!(false)
        }
    }

    #[test]
    fn test_add_units_none_ft() {
        let mut rpn_calc = run(vec!["1", "1ft", "+"]);
        assert_eq!(1, rpn_calc.stack.items().len());
        let stack_item = rpn_calc.pop().unwrap();
        match stack_item {
            StackItem::Number(n) => {
                assert_relative_eq!(2.0, n.magnitude);
                assert!(matches!(n.units,Units::Length(LengthUnits::Feet)));
            }
            _ => assert!(false)
        }
    }
}
