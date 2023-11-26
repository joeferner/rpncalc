use crate::error::RpnCalcError;
use crate::functions::function::Function;
use crate::functions::Category;
use crate::rpn_calc::RpnCalc;
use crate::units::AngleUnits;
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

    fn get_help(&self) -> String {
        return "Sets the current angle mode to degrees. Functions taking angles will assume the \
            given angle is in degrees. Functions returning angles will return the angle in \
            degrees."
            .to_string();
    }
    fn get_category(&self) -> Category {
        return Category::Trig;
    }
}
