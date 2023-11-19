use crate::error::RpnCalcError;
use crate::rpn_calc::RpnCalc;
use std::fmt;

pub trait Function: fmt::Display {
    fn apply(&self, rpn_calc: &mut RpnCalc) -> Result<(), RpnCalcError>;
}
