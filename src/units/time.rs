use crate::error::RpnCalcError;
use crate::number::MagnitudeType;
use crate::units::si_prefix::SIPrefix;
use crate::units::UnitTrait;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TimeUnits {
    Second(SIPrefix),
    Minute,
    Hour,
}

impl FromStr for TimeUnits {
    type Err = RpnCalcError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        if str == "min" {
            Ok(TimeUnits::Minute)
        } else if str == "hour" {
            Ok(TimeUnits::Hour)
        } else if let Some(prefix) = str.strip_suffix('s') {
            Ok(TimeUnits::Second(SIPrefix::parse(prefix)?))
        } else {
            Err(RpnCalcError::ParseStackItem("failed to parse".to_string()))
        }
    }
}

impl UnitTrait for TimeUnits {
    fn convert_to_base_units(&self, n: MagnitudeType) -> MagnitudeType {
        return match self {
            TimeUnits::Second(si) => n * si.multiplier(),
            TimeUnits::Minute => n * 60.0,
            TimeUnits::Hour => n * 60.0 * 60.0,
        };
    }

    fn convert_from_base_units(&self, n: MagnitudeType) -> MagnitudeType {
        return match self {
            TimeUnits::Second(si) => n / si.multiplier(),
            TimeUnits::Minute => n / 60.0,
            TimeUnits::Hour => n / 60.0 / 60.0,
        };
    }
}

impl Display for TimeUnits {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeUnits::Second(si) => write!(f, "{}s", si),
            TimeUnits::Minute => write!(f, "min"),
            TimeUnits::Hour => write!(f, "hour"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_fmt() {
        assert_eq!("s", format!("{}", TimeUnits::Second(SIPrefix::None)));
        assert_eq!("ms", format!("{}", TimeUnits::Second(SIPrefix::Milli)));
        assert_eq!("min", format!("{}", TimeUnits::Minute));
        assert_eq!("hour", format!("{}", TimeUnits::Hour));
    }

    #[test]
    fn test_convert_to_base_units() {
        assert_relative_eq!(0.01, TimeUnits::Second(SIPrefix::Milli).convert_to_base_units(10.0));
        assert_relative_eq!(120.0, TimeUnits::Minute.convert_to_base_units(2.0));
        assert_relative_eq!(18000.0, TimeUnits::Hour.convert_to_base_units(5.0));
    }

    #[test]
    fn test_convert_from_base_units() {
        assert_relative_eq!(
            10000.0,
            TimeUnits::Second(SIPrefix::Milli).convert_from_base_units(10.0)
        );
        assert_relative_eq!(0.5, TimeUnits::Minute.convert_from_base_units(30.0));
        assert_relative_eq!(0.5, TimeUnits::Hour.convert_from_base_units(1800.0));
    }
}
