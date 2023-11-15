use std::fmt::{Display, Formatter};
use crate::function::Function;
use crate::rpn_calc::{RpnCalc, RpnCalcError};
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
        let arg = rpn_calc.get_unary_number_operator_arg_radians()?;
        let result = arg.sin();
        rpn_calc.push(StackItem::Number(result));
        return Ok(());
    }
}
