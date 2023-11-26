use crate::error::RpnCalcError;
use crate::functions::function::Function;
use crate::functions::Category;
use crate::rpn_calc::RpnCalc;
use crate::units::AngleUnits;
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

    fn get_help(&self) -> String {
        return "Sets the current angle mode to radians. Functions taking angles will assume the \
            given angle is in radians. Functions returning angles will return the angle in \
            radians."
            .to_string();
    }
    fn get_category(&self) -> Category {
        return Category::Trig;
    }
}
