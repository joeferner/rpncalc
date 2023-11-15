use std::fmt::{Display, Formatter};
use crate::function::Function;
use crate::rpn_calc::{RpnCalc, RpnCalcError};
use crate::stack_item::StackItem;

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
        let args = rpn_calc.get_binary_number_operator_args()?;
        let result = args.1 / args.0;
        rpn_calc.push(StackItem::Number(result));
        return Ok(());
    }
}
