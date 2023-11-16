use std::fmt;
use crate::error::RpnCalcError;
use crate::rpn_calc::{RpnCalc};

pub trait Function: fmt::Display {
    fn apply(&self, rpn_calc: &mut RpnCalc) -> Result<(), RpnCalcError>;
}