use crate::error::RpnCalcError;
use crate::functions::function::Function;
use crate::functions::Category;
use crate::rpn_calc::RpnCalc;
use crate::units::AngleUnits;
use std::fmt::{Display, Formatter};

pub struct Gradians {}

impl Gradians {
    pub fn new() -> Self {
        return Gradians {};
    }
}

impl Display for Gradians {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "grad")
    }
}

impl Function for Gradians {
    fn apply(&self, rpn_calc: &mut RpnCalc) -> Result<(), RpnCalcError> {
        rpn_calc.angle_mode = AngleUnits::Gradians;
        return Ok(());
    }

    fn get_help(&self) -> String {
        return "Sets the current angle mode to gradians. Functions taking angles will assume the \
            given angle is in gradians. Functions returning angles will return the angle in \
            gradians."
            .to_string();
    }
    fn get_category(&self) -> Category {
        return Category::Trig;
    }
}
