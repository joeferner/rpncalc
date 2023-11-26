use crate::error::RpnCalcError;
use crate::functions::Category;
use crate::rpn_calc::RpnCalc;
use std::fmt;

pub trait Function: fmt::Display {
    fn apply(&self, rpn_calc: &mut RpnCalc) -> Result<(), RpnCalcError>;

    fn get_help(&self) -> String;

    fn get_category(&self) -> Category;
}
