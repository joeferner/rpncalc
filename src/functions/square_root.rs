use std::fmt::{Display, Formatter};
use crate::error::RpnCalcError;
use crate::function::Function;
use crate::number::Number;
use crate::rpn_calc::{RpnCalc};
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
        return rpn_calc.execute_unary_number_operator(|rpn_calc, a| {
            let result = a.pow(&Number::from(0.5))?;
            rpn_calc.push(StackItem::Number(result));
            return Ok(());
        });
    }
}
