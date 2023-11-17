use std::fmt::{Display, Formatter};
use crate::angle_type::AngleType;
use crate::error::RpnCalcError;
use crate::function::Function;
use crate::rpn_calc::{RpnCalc};

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
        rpn_calc.set_angle_mode(AngleType::Radians);
        return Ok(());
    }
}
