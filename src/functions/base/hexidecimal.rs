use crate::error::RpnCalcError;
use crate::functions::function::Function;
use crate::functions::Category;
use crate::rpn_calc::RpnCalc;
use std::fmt::{Display, Formatter};

pub struct Hexidecimal {}

impl Hexidecimal {
    pub fn new() -> Self {
        return Hexidecimal {};
    }
}

impl Display for Hexidecimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "hex")
    }
}

impl Function for Hexidecimal {
    fn apply(&self, rpn_calc: &mut RpnCalc) -> Result<(), RpnCalcError> {
        rpn_calc.base = 16;
        return Ok(());
    }

    fn get_help(&self) -> String {
        return "Sets the current display to base 16.".to_string();
    }
    fn get_category(&self) -> Category {
        return Category::Base;
    }
}
