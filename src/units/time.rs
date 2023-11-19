use std::fmt::{Display, Formatter};
use crate::number::MagnitudeType;
use crate::units::si_prefix::SIPrefix;
use crate::units::UnitTrait;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TimeUnits {
    Second(SIPrefix),
    Minute,
    Hour,
}

impl UnitTrait for TimeUnits {
    fn convert_to_base_units(&self, n: MagnitudeType) -> MagnitudeType {
        return match self {
            TimeUnits::Second(si) => n * si.multiplier(),
            TimeUnits::Minute => n * 60.0,
            TimeUnits::Hour => n * 60.0 * 60.0
        };
    }

    fn convert_from_base_units(&self, n: MagnitudeType) -> MagnitudeType {
        return match self {
            TimeUnits::Second(si) => n / si.multiplier(),
            TimeUnits::Minute => n / 60.0,
            TimeUnits::Hour => n / 60.0 / 60.0
        };
    }
}

impl Display for TimeUnits {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeUnits::Second(si) => write!(f, "{}m", si),
            TimeUnits::Minute => write!(f, "min"),
            TimeUnits::Hour => write!(f, "hour"),
        }
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use super::*;

    #[test]
    fn test_convert_to_base_units() {
        assert_relative_eq!(0.01, TimeUnits::Second(SIPrefix::Milli).convert_to_base_units(10.0));
        assert_relative_eq!(120.0, TimeUnits::Minute.convert_to_base_units(2.0));
        assert_relative_eq!(18000.0, TimeUnits::Hour.convert_to_base_units(5.0));
    }

    #[test]
    fn test_convert_from_base_units() {
        assert_relative_eq!(10000.0, TimeUnits::Second(SIPrefix::Milli).convert_from_base_units(10.0));
        assert_relative_eq!(0.5, TimeUnits::Minute.convert_from_base_units(30.0));
        assert_relative_eq!(0.5, TimeUnits::Hour.convert_from_base_units(1800.0));
    }
}