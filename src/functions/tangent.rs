use std::fmt::{Display, Formatter};
use crate::function::Function;
use crate::rpn_calc::{RpnCalc, RpnCalcError};
use crate::stack_item::StackItem;

pub struct Tangent {}

impl Tangent {
    pub fn new() -> Self {
        return Tangent {};
    }
}

impl Display for Tangent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "tan")
    }
}

impl Function for Tangent {
    fn apply(&self, rpn_calc: &mut RpnCalc) -> Result<(), RpnCalcError> {
        let arg = rpn_calc.get_unary_number_operator_arg()?;
        let result = arg.tan(rpn_calc.angle_mode())?;
        rpn_calc.push(StackItem::Number(result));
        return Ok(());
    }
}
