use std::fmt::{Display, Formatter};
use crate::error::RpnCalcError;
use crate::function::Function;
use crate::rpn_calc::{RpnCalc};

pub struct Drop {}

impl Drop {
    pub fn new() -> Self {
        return Drop {};
    }
}

impl Display for Drop {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "drop")
    }
}

impl Function for Drop {
    fn apply(&self, rpn_calc: &mut RpnCalc) -> Result<(), RpnCalcError> {
        return rpn_calc.pop();
    }
}
