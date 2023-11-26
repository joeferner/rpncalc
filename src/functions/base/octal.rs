use crate::error::RpnCalcError;
use crate::function::Function;
use crate::rpn_calc::RpnCalc;
use std::fmt::{Display, Formatter};

pub struct Octal {}

impl Octal {
    pub fn new() -> Self {
        return Octal {};
    }
}

impl Display for Octal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "oct")
    }
}

impl Function for Octal {
    fn apply(&self, rpn_calc: &mut RpnCalc) -> Result<(), RpnCalcError> {
        rpn_calc.base = 8;
        return Ok(());
    }

    fn get_help(&self) -> String {
        return "Sets the current display to base 8.".to_string();
    }
}
