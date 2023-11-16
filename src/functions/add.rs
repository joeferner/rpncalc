use std::fmt::{Display, Formatter};
use crate::function::Function;
use crate::rpn_calc::{RpnCalc, RpnCalcError};
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
        let args = rpn_calc.get_binary_number_operator_args()?;
        let result = args.1.add(&args.0)?;
        rpn_calc.push(StackItem::Number(result));
        return Ok(());
    }
}
