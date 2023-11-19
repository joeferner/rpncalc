use std::fmt::{Display, Formatter};
use crate::error::RpnCalcError;
use crate::function::Function;
use crate::rpn_calc::{RpnCalc};
use crate::stack_item::StackItem;

pub struct Sine {}

impl Sine {
    pub fn new() -> Self {
        return Sine {};
    }
}

impl Display for Sine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "sin")
    }
}

impl Function for Sine {
    fn apply(&self, rpn_calc: &mut RpnCalc) -> Result<(), RpnCalcError> {
        return rpn_calc.execute_unary_number_operator(|rpn_calc, a| {
            let result = a.sin(rpn_calc.angle_mode)?;
            rpn_calc.push(StackItem::Number(result));
            return Ok(());
        });
    }
}
