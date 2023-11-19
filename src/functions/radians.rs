use crate::error::RpnCalcError;
use crate::function::Function;
use crate::rpn_calc::RpnCalc;
use crate::units::angle::AngleUnits;
use std::fmt::{Display, Formatter};

pub struct Radians {}

impl Radians {
    pub fn new() -> Self {
        return Radians {};
    }
}

impl Display for Radians {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "rad")
    }
}

impl Function for Radians {
    fn apply(&self, rpn_calc: &mut RpnCalc) -> Result<(), RpnCalcError> {
        rpn_calc.angle_mode = AngleUnits::Radians;
        return Ok(());
    }
}
