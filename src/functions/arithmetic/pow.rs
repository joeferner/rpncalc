use crate::error::RpnCalcError;
use crate::functions::function::Function;
use crate::functions::Category;
use crate::rpn_calc::RpnCalc;
use crate::stack_item::StackItem;
use std::fmt::{Display, Formatter};

pub struct Pow {}

impl Pow {
    pub fn new() -> Self {
        return Pow {};
    }
}

impl Display for Pow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "pow")
    }
}

impl Function for Pow {
    fn apply(&self, rpn_calc: &mut RpnCalc) -> Result<(), RpnCalcError> {
        return rpn_calc.execute_binary_number_operator(|rpn_calc, a, b| {
            let result = a.pow(&b)?;
            rpn_calc.push(StackItem::Number(result));
            return Ok(());
        });
    }

    fn get_help(&self) -> String {
        return "Raise the second item on the stack to the first item on the stack.".to_string();
    }
    fn get_category(&self) -> Category {
        return Category::Arithmetic;
    }
}
