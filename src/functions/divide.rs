use crate::error::RpnCalcError;
use crate::function::Function;
use crate::rpn_calc::RpnCalc;
use crate::stack_item::StackItem;
use std::fmt::{Display, Formatter};

pub struct Divide {}

impl Divide {
    pub fn new() -> Self {
        return Divide {};
    }
}

impl Display for Divide {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "div")
    }
}

impl Function for Divide {
    fn apply(&self, rpn_calc: &mut RpnCalc) -> Result<(), RpnCalcError> {
        return rpn_calc.execute_binary_number_operator(|rpn_calc, a, b| {
            let result = a.divide(&b)?;
            rpn_calc.push(StackItem::Number(result));
            return Ok(());
        });
    }

    fn get_help(&self) -> String {
        return "Divides the top two items on the stack".to_string();
    }
}
