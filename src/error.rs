use crate::units::Units;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RpnCalcError {
    #[error("{0}")]
    GenericError(String),
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
    #[error("divide by zero")]
    DivideByZero,
}

impl PartialEq for RpnCalcError {
    fn eq(&self, other: &Self) -> bool {
        return match self {
            RpnCalcError::GenericError(str) => match other {
                RpnCalcError::GenericError(other_str) => str == other_str,
                _ => false,
            },
            RpnCalcError::ParseStackItem(str) => match other {
                RpnCalcError::ParseStackItem(other_str) => str == other_str,
                _ => false,
            },
            RpnCalcError::NotEnoughArguments => matches!(other, RpnCalcError::NotEnoughArguments),
            RpnCalcError::InvalidArgument(str) => match other {
                RpnCalcError::InvalidArgument(other_str) => str == other_str,
                _ => false,
            },
            RpnCalcError::StdIoError(err) => match other {
                RpnCalcError::StdIoError(other_err) => format!("{}", err) == format!("{}", other_err),
                _ => false,
            },
            RpnCalcError::InvalidUnits(str) => match other {
                RpnCalcError::InvalidUnits(other_str) => str == other_str,
                _ => false,
            },
            RpnCalcError::IncompatibleUnits(units_a, units_b) => match other {
                RpnCalcError::IncompatibleUnits(other_units_a, other_units_b) => {
                    units_a == other_units_a && units_b == other_units_b
                }
                _ => false,
            },
            RpnCalcError::DivideByZero => matches!(other, RpnCalcError::DivideByZero),
        };
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::units::{LengthUnits, SIPrefix};

    #[test]
    fn test_eq() {
        assert_eq!(
            RpnCalcError::GenericError("test".to_string()),
            RpnCalcError::GenericError("test".to_string())
        );
        assert_ne!(
            RpnCalcError::GenericError("test".to_string()),
            RpnCalcError::GenericError("other".to_string())
        );

        assert_eq!(
            RpnCalcError::ParseStackItem("test".to_string()),
            RpnCalcError::ParseStackItem("test".to_string())
        );
        assert_ne!(
            RpnCalcError::ParseStackItem("test".to_string()),
            RpnCalcError::ParseStackItem("other".to_string())
        );

        assert_eq!(RpnCalcError::NotEnoughArguments, RpnCalcError::NotEnoughArguments);
        assert_ne!(
            RpnCalcError::NotEnoughArguments,
            RpnCalcError::ParseStackItem("".to_string())
        );

        assert_eq!(
            RpnCalcError::InvalidArgument("test".to_string()),
            RpnCalcError::InvalidArgument("test".to_string())
        );
        assert_ne!(
            RpnCalcError::InvalidArgument("test".to_string()),
            RpnCalcError::InvalidArgument("other".to_string())
        );

        assert_eq!(
            RpnCalcError::InvalidUnits("test".to_string()),
            RpnCalcError::InvalidUnits("test".to_string())
        );
        assert_ne!(
            RpnCalcError::InvalidUnits("test".to_string()),
            RpnCalcError::InvalidUnits("other".to_string())
        );

        let units_none = Units::None;
        let meters = Units::Length(LengthUnits::Meter(SIPrefix::None));
        assert_eq!(
            RpnCalcError::IncompatibleUnits(units_none.clone(), meters.clone()),
            RpnCalcError::IncompatibleUnits(units_none.clone(), meters.clone())
        );
        assert_ne!(
            RpnCalcError::IncompatibleUnits(units_none.clone(), meters.clone()),
            RpnCalcError::IncompatibleUnits(meters, units_none)
        );

        assert_eq!(RpnCalcError::DivideByZero, RpnCalcError::DivideByZero);
        assert_ne!(RpnCalcError::DivideByZero, RpnCalcError::ParseStackItem("".to_string()));
    }
}
