use std::fmt::{Display, Formatter};
use crate::error::RpnCalcError;
use crate::function::Function;
use crate::rpn_calc::{RpnCalc};
use crate::stack_item::StackItem;

pub struct Add {}

impl Add {
    pub fn new() -> Self {
        return Add {};
    }
}

impl Display for Add {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "add")
    }
}

impl Function for Add {
    fn apply(&self, rpn_calc: &mut RpnCalc) -> Result<(), RpnCalcError> {
        return rpn_calc.execute_binary_number_operator(|rpn_calc, a, b| {
            let result = a.add(&b)?;
            rpn_calc.push(StackItem::Number(result));
            return Ok(());
        });
    }
}
