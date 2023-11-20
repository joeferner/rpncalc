use crate::error::RpnCalcError;
use crate::number::MagnitudeType;
use crate::units::si_prefix::SIPrefix;
use crate::units::UnitTrait;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LengthUnits {
    Meter(SIPrefix),
    Inch,
    Foot,
    Yard,
    Mile,
    NauticalMile,
}

impl FromStr for LengthUnits {
    type Err = RpnCalcError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        if str == "NM" {
            Ok(LengthUnits::NauticalMile)
        } else if str == "mile" {
            Ok(LengthUnits::Mile)
        } else if str == "yard" {
            Ok(LengthUnits::Yard)
        } else if str == "ft" {
            Ok(LengthUnits::Foot)
        } else if str == "in" {
            Ok(LengthUnits::Inch)
        } else if let Some(prefix) = str.strip_suffix('m') {
            Ok(LengthUnits::Meter(SIPrefix::parse(prefix)?))
        } else {
            Err(RpnCalcError::ParseStackItem("failed to parse".to_string()))
        }
    }
}

impl UnitTrait for LengthUnits {
    fn convert_to_base_units(&self, n: MagnitudeType) -> MagnitudeType {
        return match self {
            LengthUnits::Meter(si) => n * si.multiplier(),
            LengthUnits::Inch => n * 0.0254,
            LengthUnits::Foot => n * 0.3048,
            LengthUnits::Yard => n * 0.9144,
            LengthUnits::Mile => n * 1609.36,
            LengthUnits::NauticalMile => n * 1852.0,
        };
    }

    fn convert_from_base_units(&self, n: MagnitudeType) -> MagnitudeType {
        return match self {
            LengthUnits::Meter(si) => n / si.multiplier(),
            LengthUnits::Inch => n / 0.0254,
            LengthUnits::Foot => n / 0.3048,
            LengthUnits::Yard => n / 0.9144,
            LengthUnits::Mile => n / 1609.36,
            LengthUnits::NauticalMile => n / 1852.0,
        };
    }
}

impl Display for LengthUnits {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LengthUnits::Meter(si) => write!(f, "{}m", si),
            LengthUnits::Inch => write!(f, "in"),
            LengthUnits::Foot => write!(f, "ft"),
            LengthUnits::Yard => write!(f, "yard"),
            LengthUnits::Mile => write!(f, "mile"),
            LengthUnits::NauticalMile => write!(f, "NM"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_convert_to_base_units() {
        assert_relative_eq!(0.45, LengthUnits::Meter(SIPrefix::None).convert_to_base_units(0.45));
        assert_relative_eq!(0.7111999999999999, LengthUnits::Inch.convert_to_base_units(28.0));
        assert_relative_eq!(1.2192, LengthUnits::Foot.convert_to_base_units(4.0));
        assert_relative_eq!(3.6576, LengthUnits::Yard.convert_to_base_units(4.0));
        assert_relative_eq!(804.68, LengthUnits::Mile.convert_to_base_units(0.5));
        assert_relative_eq!(926.0, LengthUnits::NauticalMile.convert_to_base_units(0.5));
    }

    #[test]
    fn test_convert_from_base_units() {
        assert_relative_eq!(0.45, LengthUnits::Meter(SIPrefix::None).convert_from_base_units(0.45));
        assert_relative_eq!(19.68503937007874, LengthUnits::Inch.convert_from_base_units(0.5));
        assert_relative_eq!(13.123359580052492, LengthUnits::Foot.convert_from_base_units(4.0));
        assert_relative_eq!(4.374453193350831, LengthUnits::Yard.convert_from_base_units(4.0));
        assert_relative_eq!(0.4970920117313715, LengthUnits::Mile.convert_from_base_units(800.0));
        assert_relative_eq!(
            0.4319654427645788,
            LengthUnits::NauticalMile.convert_from_base_units(800.0)
        );
    }
}
