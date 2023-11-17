use std::fmt::{Display, Formatter};
use crate::angle_type::AngleType;
use crate::error::RpnCalcError;
use crate::function::Function;
use crate::rpn_calc::{RpnCalc};

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
        rpn_calc.set_angle_mode(AngleType::Degrees);
        return Ok(());
    }
}
