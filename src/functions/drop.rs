use crate::error::RpnCalcError;
use crate::function::Function;
use crate::rpn_calc::RpnCalc;
use std::fmt::{Display, Formatter};

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
        rpn_calc.pop()?;
        return Ok(());
    }
}
