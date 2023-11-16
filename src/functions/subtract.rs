use std::fmt::{Display, Formatter};
use crate::error::RpnCalcError;
use crate::function::Function;
use crate::rpn_calc::{RpnCalc};
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
        return rpn_calc.execute_binary_number_operator(|rpn_calc, a, b| {
            let result = a.subtract(&b)?;
            rpn_calc.push(StackItem::Number(result));
            return Ok(());
        });
    }
}
