use crate::error::RpnCalcError;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::number::{MagnitudeType, MAGNITUDE_TYPE_PI};
use crate::units::UnitTrait;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AngleUnits {
    Radians,
    Degrees,
    Gradians,
}

impl FromStr for AngleUnits {
    type Err = RpnCalcError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        if str == "deg" {
            Ok(AngleUnits::Degrees)
        } else if str == "rad" {
            Ok(AngleUnits::Radians)
        } else if str == "grad" {
            Ok(AngleUnits::Gradians)
        } else {
            Err(RpnCalcError::ParseStackItem("failed to parse".to_string()))
        }
    }
}

impl UnitTrait for AngleUnits {
    fn convert_to_base_units(&self, n: MagnitudeType) -> MagnitudeType {
        match self {
            AngleUnits::Radians => n,
            AngleUnits::Degrees => degrees_to_radians(n),
            AngleUnits::Gradians => gradians_to_radians(n),
        }
    }

    fn convert_from_base_units(&self, n: MagnitudeType) -> MagnitudeType {
        match self {
            AngleUnits::Radians => n,
            AngleUnits::Degrees => radians_to_degrees(n),
            AngleUnits::Gradians => radians_to_gradians(n),
        }
    }
}

impl Display for AngleUnits {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AngleUnits::Radians => write!(f, "rad"),
            AngleUnits::Degrees => write!(f, "deg"),
            AngleUnits::Gradians => write!(f, "grad"),
        }
    }
}

pub fn degrees_to_radians(deg: MagnitudeType) -> MagnitudeType {
    return deg * MAGNITUDE_TYPE_PI / 180.0;
}

pub fn radians_to_degrees(rad: MagnitudeType) -> MagnitudeType {
    return rad * 180.0 / MAGNITUDE_TYPE_PI;
}

pub fn gradians_to_radians(grad: MagnitudeType) -> MagnitudeType {
    return grad * MAGNITUDE_TYPE_PI / 200.0;
}

pub fn radians_to_gradians(rad: MagnitudeType) -> MagnitudeType {
    return rad * 200.0 / MAGNITUDE_TYPE_PI;
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_convert_to_base_units() {
        assert_relative_eq!(0.45, AngleUnits::Radians.convert_to_base_units(0.45));
        assert_relative_eq!(0.7853981633974483, AngleUnits::Degrees.convert_to_base_units(45.0));
    }

    #[test]
    fn test_convert_from_base_units() {
        assert_relative_eq!(0.45, AngleUnits::Radians.convert_from_base_units(0.45));
        assert_relative_eq!(45.0, AngleUnits::Degrees.convert_from_base_units(0.7853981633974483));
    }
}
