use crate::error::RpnCalcError;
use crate::function::Function;
use crate::rpn_calc::RpnCalc;
use std::fmt::{Display, Formatter};

pub struct Binary {}

impl Binary {
    pub fn new() -> Self {
        return Binary {};
    }
}

impl Display for Binary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "bin")
    }
}

impl Function for Binary {
    fn apply(&self, rpn_calc: &mut RpnCalc) -> Result<(), RpnCalcError> {
        rpn_calc.base = 2;
        return Ok(());
    }

    fn get_help(&self) -> String {
        return "Sets the current display to base 2.".to_string();
    }
}
