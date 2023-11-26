use crate::error::RpnCalcError;
use crate::functions::function::Function;
use crate::functions::Category;
use crate::rpn_calc::RpnCalc;
use crate::stack_item::StackItem;
use std::fmt::{Display, Formatter};

pub struct Multiply {}

impl Multiply {
    pub fn new() -> Self {
        return Multiply {};
    }
}

impl Display for Multiply {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "mul")
    }
}

impl Function for Multiply {
    fn apply(&self, rpn_calc: &mut RpnCalc) -> Result<(), RpnCalcError> {
        return rpn_calc.execute_binary_number_operator(|rpn_calc, a, b| {
            let result = a.multiply(&b)?;
            rpn_calc.push(StackItem::Number(result));
            return Ok(());
        });
    }

    fn get_help(&self) -> String {
        return "Multiply the top two items on the stack".to_string();
    }
    fn get_category(&self) -> Category {
        return Category::Arithmetic;
    }
}
