use crate::error::RpnCalcError;
use crate::function::Function;
use crate::rpn_calc::RpnCalc;
use std::fmt::{Display, Formatter};

pub struct Decimal {}

impl Decimal {
    pub fn new() -> Self {
        return Decimal {};
    }
}

impl Display for Decimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "dec")
    }
}

impl Function for Decimal {
    fn apply(&self, rpn_calc: &mut RpnCalc) -> Result<(), RpnCalcError> {
        rpn_calc.base = 10;
        return Ok(());
    }

    fn get_help(&self) -> String {
        return "Sets the current display to base 10.".to_string();
    }
}
