use std::fmt;
use crate::rpn_calc::{RpnCalc, RpnCalcError};

pub trait Function: fmt::Display {
    fn apply(&self, rpn_calc: &mut RpnCalc) -> Result<(), RpnCalcError>;
}