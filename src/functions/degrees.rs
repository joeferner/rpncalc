use crate::error::RpnCalcError;
use crate::function::Function;
use crate::rpn_calc::RpnCalc;
use crate::units::angle::AngleUnits;
use std::fmt::{Display, Formatter};

pub struct Degrees {}

impl Degrees {
    pub fn new() -> Self {
        return Degrees {};
    }
}

impl Display for Degrees {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "deg")
    }
}

impl Function for Degrees {
    fn apply(&self, rpn_calc: &mut RpnCalc) -> Result<(), RpnCalcError> {
        rpn_calc.angle_mode = AngleUnits::Degrees;
        return Ok(());
    }
}
