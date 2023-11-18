use std::fmt::{Display, Formatter};

use crate::number::{MAGNITUDE_TYPE_PI, MagnitudeType};
use crate::units::UnitTrait;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AngleUnits {
    Radians,
    Degrees,
}

impl UnitTrait for AngleUnits {
    fn convert_to_base_units(&self, n: MagnitudeType) -> MagnitudeType {
        match self {
            AngleUnits::Radians => n,
            AngleUnits::Degrees => degrees_to_radians(n)
        }
    }

    fn convert_from_base_units(&self, n: MagnitudeType) -> MagnitudeType {
        match self {
            AngleUnits::Radians => n,
            AngleUnits::Degrees => radians_to_degrees(n)
        }
    }
}

impl Display for AngleUnits {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AngleUnits::Radians => write!(f, "rad"),
            AngleUnits::Degrees => write!(f, "deg")
        }
    }
}

pub fn degrees_to_radians(deg: MagnitudeType) -> MagnitudeType {
    return deg * MAGNITUDE_TYPE_PI / 180.0;
}

pub fn radians_to_degrees(rad: MagnitudeType) -> MagnitudeType {
    return rad * 180.0 / MAGNITUDE_TYPE_PI;
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use super::*;

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