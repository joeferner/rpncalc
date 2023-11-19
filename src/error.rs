use thiserror::Error;
use crate::units::Units;

#[derive(Debug, Error)]
pub enum RpnCalcError {
    #[error("parse stack item: {0}")]
    ParseStackItem(String),
    #[error("not enough arguments")]
    NotEnoughArguments,
    #[error("invalid argument {0}")]
    InvalidArgument(String),
    #[error("IO error: {0}")]
    StdIoError(#[from] std::io::Error),
    #[error("invalid units {0}")]
    InvalidUnits(String),
    #[error("incompatible units {0} and {1}")]
    IncompatibleUnits(Units, Units),
}

impl PartialEq for RpnCalcError {
    fn eq(&self, other: &Self) -> bool {
        return match self {
            RpnCalcError::ParseStackItem(str) => {
                match other {
                    RpnCalcError::ParseStackItem(other_str) => str == other_str,
                    _ => false
                }
            }
            RpnCalcError::NotEnoughArguments => {
                match other {
                    RpnCalcError::NotEnoughArguments => true,
                    _ => false
                }
            }
            RpnCalcError::InvalidArgument(str) => {
                match other {
                    RpnCalcError::InvalidArgument(other_str) => str == other_str,
                    _ => false
                }
            }
            RpnCalcError::StdIoError(err) => {
                match other {
                    RpnCalcError::StdIoError(other_err) => format!("{}", err) == format!("{}", other_err),
                    _ => false
                }
            }
            RpnCalcError::InvalidUnits(str) => {
                match other {
                    RpnCalcError::InvalidUnits(other_str) => str == other_str,
                    _ => false
                }
            }
            RpnCalcError::IncompatibleUnits(units_a, units_b) => {
                match other {
                    RpnCalcError::IncompatibleUnits(other_units_a, other_units_b) => units_a == other_units_a && units_b == other_units_b,
                    _ => false
                }
            }
        };
    }
}