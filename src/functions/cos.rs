use crate::error::RpnCalcError;
use crate::function::Function;
use crate::rpn_calc::RpnCalc;
use crate::stack_item::StackItem;
use std::fmt::{Display, Formatter};

pub struct Cos {}

impl Cos {
    pub fn new() -> Self {
        return Cos {};
    }
}

impl Display for Cos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "cos")
    }
}

impl Function for Cos {
    fn apply(&self, rpn_calc: &mut RpnCalc) -> Result<(), RpnCalcError> {
        return rpn_calc.execute_unary_number_operator(|rpn_calc, a| {
            let result = a.cos(rpn_calc.angle_mode)?;
            rpn_calc.push(StackItem::Number(result));
            return Ok(());
        });
    }
}