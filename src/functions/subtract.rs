use std::fmt::{Display, Formatter};
use crate::function::Function;
use crate::rpn_calc::{RpnCalc, RpnCalcError};
use crate::stack_item::StackItem;

pub struct Subtract {}

impl Subtract {
    pub fn new() -> Self {
        return Subtract {};
    }
}

impl Display for Subtract {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "sub")
    }
}

impl Function for Subtract {
    fn apply(&self, rpn_calc: &mut RpnCalc) -> Result<(), RpnCalcError> {
        let args = rpn_calc.get_binary_number_operator_args()?;
        let result = args.1 - args.0;
        rpn_calc.push(StackItem::Number(result));
        return Ok(());
    }
}
