use std::fmt::{Display, Formatter};
use crate::function::Function;
use crate::rpn_calc::{RpnCalc, RpnCalcError};
use crate::stack_item::StackItem;

pub struct Cosine {}

impl Cosine {
    pub fn new() -> Self {
        return Cosine {};
    }
}

impl Display for Cosine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "cos")
    }
}

impl Function for Cosine {
    fn apply(&self, rpn_calc: &mut RpnCalc) -> Result<(), RpnCalcError> {
        let arg = rpn_calc.get_unary_number_operator_arg_radians()?;
        let result = arg.cos();
        rpn_calc.push(StackItem::Number(result));
        return Ok(());
    }
}
