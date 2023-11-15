use std::fmt::{Display, Formatter};
use crate::function::Function;
use crate::rpn_calc::{RpnCalc, RpnCalcError};
use crate::stack_item::StackItem;

pub struct SquareRoot {}

impl SquareRoot {
    pub fn new() -> Self {
        return SquareRoot {};
    }
}

impl Display for SquareRoot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "pow")
    }
}

impl Function for SquareRoot {
    fn apply(&self, rpn_calc: &mut RpnCalc) -> Result<(), RpnCalcError> {
        let arg = rpn_calc.get_unary_number_operator_arg()?;
        let result = arg.powf(0.5);
        rpn_calc.push(StackItem::Number(result));
        return Ok(());
    }
}
