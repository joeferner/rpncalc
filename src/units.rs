use std::fmt::{Display, Formatter};
use crate::angle_type::AngleType;
use crate::error::RpnCalcError;
use crate::number::MagnitudeType;

#[derive(Clone, Debug, PartialEq)]
pub enum UnitsOperator {
    Divide,
    Multiply,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SIPrefix {
    /// 10^30
    Quetta,
    /// 10^27
    Ronna,
    /// 10^24
    Yotta,
    /// 10^21
    Zetta,
    /// 10^18
    Exa,
    /// 10^15
    Peta,
    /// 10^12
    Tera,
    /// 10^9
    Giga,
    /// 10^6
    Mega,
    /// 10^3
    Kilo,
    /// 10^2
    Hecto,
    /// 10^1
    Deka,
    /// 10^0
    None,
    /// 10^-1
    Deci,
    /// 10^-2
    Centi,
    /// 10^-3
    Milli,
    /// 10^-6
    Micro,
    /// 10^-9
    Nano,
    /// 10^-12
    Pico,
    /// 10^-15
    Femto,
    /// 10^-18
    Atto,
    /// 10^-21
    Zepto,
    /// 10^-24
    Yocto,
    /// 10^-27
    Ronto,
    /// 10^-30
    Quecto,
}

impl SIPrefix {
    fn parse(str: &str) -> Result<SIPrefix, RpnCalcError> {
        return if str.len() == 0 {
            Ok(SIPrefix::None)
        } else if str == "d" {
            Ok(SIPrefix::Deci)
        } else if str == "c" {
            Ok(SIPrefix::Centi)
        } else if str == "m" {
            Ok(SIPrefix::Milli)
        } else if str == "u" || str == "μ" {
            Ok(SIPrefix::Micro)
        } else if str == "n" {
            Ok(SIPrefix::Nano)
        } else if str == "p" {
            Ok(SIPrefix::Pico)
        } else if str == "f" {
            Ok(SIPrefix::Femto)
        } else if str == "a" {
            Ok(SIPrefix::Atto)
        } else if str == "z" {
            Ok(SIPrefix::Zepto)
        } else if str == "y" {
            Ok(SIPrefix::Yocto)
        } else if str == "r" {
            Ok(SIPrefix::Ronto)
        } else if str == "q" {
            Ok(SIPrefix::Quecto)
        } else if str == "da" {
            Ok(SIPrefix::Deka)
        } else if str == "h" {
            Ok(SIPrefix::Hecto)
        } else if str == "k" {
            Ok(SIPrefix::Kilo)
        } else if str == "M" {
            Ok(SIPrefix::Mega)
        } else if str == "G" {
            Ok(SIPrefix::Giga)
        } else if str == "T" {
            Ok(SIPrefix::Tera)
        } else if str == "P" {
            Ok(SIPrefix::Peta)
        } else if str == "E" {
            Ok(SIPrefix::Exa)
        } else if str == "Z" {
            Ok(SIPrefix::Zetta)
        } else if str == "Y" {
            Ok(SIPrefix::Yotta)
        } else if str == "R" {
            Ok(SIPrefix::Ronna)
        } else if str == "Q" {
            Ok(SIPrefix::Quetta)
        } else {
            Err(RpnCalcError::InvalidUnits(format!("unhandled SI prefix: {}", str)))
        };
    }

    pub fn multiplier(&self) -> MagnitudeType {
        return match self {
            SIPrefix::Quetta => 1e30,
            SIPrefix::Ronna => 1e27,
            SIPrefix::Yotta => 1e24,
            SIPrefix::Zetta => 1e21,
            SIPrefix::Exa => 1e18,
            SIPrefix::Peta => 1e15,
            SIPrefix::Tera => 1e12,
            SIPrefix::Giga => 1e9,
            SIPrefix::Mega => 1e6,
            SIPrefix::Kilo => 1e3,
            SIPrefix::Hecto => 1e2,
            SIPrefix::Deka => 1e1,
            SIPrefix::None => 1.0,
            SIPrefix::Deci => 1e-1,
            SIPrefix::Centi => 1e-2,
            SIPrefix::Milli => 1e-3,
            SIPrefix::Micro => 1e-6,
            SIPrefix::Nano => 1e-9,
            SIPrefix::Pico => 1e-12,
            SIPrefix::Femto => 1e-15,
            SIPrefix::Atto => 1e-18,
            SIPrefix::Zepto => 1e-21,
            SIPrefix::Yocto => 1e-24,
            SIPrefix::Ronto => 1e-27,
            SIPrefix::Quecto => 1e-30,
        };
    }
}

impl Display for SIPrefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SIPrefix::Quetta => write!(f, "Q"),
            SIPrefix::Ronna => write!(f, "R"),
            SIPrefix::Yotta => write!(f, "Y"),
            SIPrefix::Zetta => write!(f, "Z"),
            SIPrefix::Exa => write!(f, "E"),
            SIPrefix::Peta => write!(f, "P"),
            SIPrefix::Tera => write!(f, "T"),
            SIPrefix::Giga => write!(f, "G"),
            SIPrefix::Mega => write!(f, "M"),
            SIPrefix::Kilo => write!(f, "k"),
            SIPrefix::Hecto => write!(f, "h"),
            SIPrefix::Deka => write!(f, "da"),
            SIPrefix::None => write!(f, ""),
            SIPrefix::Deci => write!(f, "d"),
            SIPrefix::Centi => write!(f, "c"),
            SIPrefix::Milli => write!(f, "m"),
            SIPrefix::Micro => write!(f, "μ"),
            SIPrefix::Nano => write!(f, "n"),
            SIPrefix::Pico => write!(f, "p"),
            SIPrefix::Femto => write!(f, "f"),
            SIPrefix::Atto => write!(f, "a"),
            SIPrefix::Zepto => write!(f, "z"),
            SIPrefix::Yocto => write!(f, "y"),
            SIPrefix::Ronto => write!(f, "r"),
            SIPrefix::Quecto => write!(f, "q"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LengthUnits {
    Meter(SIPrefix),
    Inches,
    Feet,
}

impl LengthUnits {
    pub fn convert_to_base_units(&self, n: MagnitudeType) -> MagnitudeType {
        return match self {
            LengthUnits::Meter(si) => n * si.multiplier(),
            LengthUnits::Feet => n * 0.3048,
            LengthUnits::Inches => n * 0.0254,
        };
    }

    pub fn convert_from_base_units(&self, n: MagnitudeType) -> MagnitudeType {
        return match self {
            LengthUnits::Meter(si) => n / si.multiplier(),
            LengthUnits::Feet => n / 0.3048,
            LengthUnits::Inches => n / 0.0254
        };
    }
}

impl Display for LengthUnits {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LengthUnits::Meter(si) => write!(f, "{}m", si),
            LengthUnits::Feet => write!(f, "ft"),
            LengthUnits::Inches => write!(f, "in")
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MassUnits {
    Gram(SIPrefix)
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TimeUnits {
    Second(SIPrefix)
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TemperatureUnits {
    Kelvin(SIPrefix),
    Celsius,
    Fahrenheit,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Units {
    None,
    Length(LengthUnits),
    Mass(MassUnits),
    Time(TimeUnits),
    Temperature(TemperatureUnits),
    Angle(AngleType),
    Compound(Box<Units>, UnitsOperator, Box<Units>),
}

impl Units {
    pub fn parse(str: &str) -> Result<Units, RpnCalcError> {
        return if str.len() == 0 {
            Ok(Units::None)
        } else if let Some(parts) = str.split_once("/") {
            Ok(Units::Compound(
                Box::new(Units::parse(parts.0)?),
                UnitsOperator::Divide,
                Box::new(Units::parse(parts.1)?),
            ))
        } else if let Some(parts) = str.split_once("*") {
            Ok(Units::Compound(
                Box::new(Units::parse(parts.0)?),
                UnitsOperator::Multiply,
                Box::new(Units::parse(parts.1)?),
            ))
        } else if let Some(parts) = str.split_once("^") {
            if parts.1 == "2" {
                Ok(Units::Compound(
                    Box::new(Units::parse(parts.0)?),
                    UnitsOperator::Multiply,
                    Box::new(Units::parse(parts.0)?),
                ))
            } else {
                Err(RpnCalcError::ParseStackItem(format!("parse units {}", str)))
            }
        } else if str == "deg" {
            Ok(Units::Angle(AngleType::Degrees))
        } else if str == "rad" {
            Ok(Units::Angle(AngleType::Radians))
        } else if str == "C" || str == "°C" {
            Ok(Units::Temperature(TemperatureUnits::Celsius))
        } else if str == "F" || str == "°F" {
            Ok(Units::Temperature(TemperatureUnits::Fahrenheit))
        } else if str == "ft" {
            Ok(Units::Length(LengthUnits::Feet))
        } else if str == "in" {
            Ok(Units::Length(LengthUnits::Inches))
        } else if let Some(prefix) = str.strip_suffix("m") {
            Ok(Units::Length(LengthUnits::Meter(SIPrefix::parse(prefix)?)))
        } else if let Some(prefix) = str.strip_suffix("g") {
            Ok(Units::Mass(MassUnits::Gram(SIPrefix::parse(prefix)?)))
        } else if let Some(prefix) = str.strip_suffix("s") {
            Ok(Units::Time(TimeUnits::Second(SIPrefix::parse(prefix)?)))
        } else if let Some(prefix) = str.strip_suffix("K") {
            Ok(Units::Temperature(TemperatureUnits::Kelvin(SIPrefix::parse(prefix)?)))
        } else {
            Err(RpnCalcError::ParseStackItem(format!("parse units {}", str)))
        };
    }

    pub fn convert_to_base_units(&self, n: MagnitudeType) -> MagnitudeType {
        return match self {
            Units::None => n,
            Units::Length(u) => u.convert_to_base_units(n),
            _ => todo!()
        };
    }

    pub fn convert_from_base_units(&self, n: MagnitudeType) -> MagnitudeType {
        return match self {
            Units::None => n,
            Units::Length(u) => u.convert_from_base_units(n),
            _ => todo!()
        };
    }
}

impl Display for Units {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Units::None => write!(f, ""),
            Units::Length(length) => write!(f, "{}", length),
            _ => todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use super::*;

    #[test]
    fn test_deg() {
        let r = Units::parse("deg").unwrap();
        assert!(matches!(r, Units::Angle(AngleType::Degrees)));
    }

    #[test]
    fn test_rad() {
        let r = Units::parse("rad").unwrap();
        assert!(matches!(r, Units::Angle(AngleType::Radians)));
    }

    #[test]
    fn test_meters() {
        let r = Units::parse("m").unwrap();
        assert!(matches!(r, Units::Length(LengthUnits::Meter(SIPrefix::None))));
    }

    #[test]
    fn test_millimeters() {
        let r = Units::parse("mm").unwrap();
        assert!(matches!(r, Units::Length(LengthUnits::Meter(SIPrefix::Milli))));
    }

    #[test]
    fn test_to_base_units_length() {
        assert_relative_eq!(2e30, Units::Length(LengthUnits::Meter(SIPrefix::Quetta)).convert_to_base_units(2.0));
        assert_relative_eq!(2e27, Units::Length(LengthUnits::Meter(SIPrefix::Ronna)).convert_to_base_units(2.0));
        assert_relative_eq!(2e24, Units::Length(LengthUnits::Meter(SIPrefix::Yotta)).convert_to_base_units(2.0));
        assert_relative_eq!(2e21, Units::Length(LengthUnits::Meter(SIPrefix::Zetta)).convert_to_base_units(2.0));
        assert_relative_eq!(2e18, Units::Length(LengthUnits::Meter(SIPrefix::Exa)).convert_to_base_units(2.0));
        assert_relative_eq!(2e15, Units::Length(LengthUnits::Meter(SIPrefix::Peta)).convert_to_base_units(2.0));
        assert_relative_eq!(2e12, Units::Length(LengthUnits::Meter(SIPrefix::Tera)).convert_to_base_units(2.0));
        assert_relative_eq!(2e9, Units::Length(LengthUnits::Meter(SIPrefix::Giga)).convert_to_base_units(2.0));
        assert_relative_eq!(2e6, Units::Length(LengthUnits::Meter(SIPrefix::Mega)).convert_to_base_units(2.0));
        assert_relative_eq!(2000.0, Units::Length(LengthUnits::Meter(SIPrefix::Kilo)).convert_to_base_units(2.0));
        assert_relative_eq!(200.0, Units::Length(LengthUnits::Meter(SIPrefix::Hecto)).convert_to_base_units(2.0));
        assert_relative_eq!(20.0, Units::Length(LengthUnits::Meter(SIPrefix::Deka)).convert_to_base_units(2.0));
        assert_relative_eq!(2.0, Units::Length(LengthUnits::Meter(SIPrefix::None)).convert_to_base_units(2.0));
        assert_relative_eq!(0.2, Units::Length(LengthUnits::Meter(SIPrefix::Deci)).convert_to_base_units(2.0));
        assert_relative_eq!(0.02, Units::Length(LengthUnits::Meter(SIPrefix::Centi)).convert_to_base_units(2.0));
        assert_relative_eq!(0.002, Units::Length(LengthUnits::Meter(SIPrefix::Milli)).convert_to_base_units(2.0));
        assert_relative_eq!(2e-6, Units::Length(LengthUnits::Meter(SIPrefix::Micro)).convert_to_base_units(2.0));
        assert_relative_eq!(2e-9, Units::Length(LengthUnits::Meter(SIPrefix::Nano)).convert_to_base_units(2.0));
        assert_relative_eq!(2e-12, Units::Length(LengthUnits::Meter(SIPrefix::Pico)).convert_to_base_units(2.0));
        assert_relative_eq!(2e-15, Units::Length(LengthUnits::Meter(SIPrefix::Femto)).convert_to_base_units(2.0));
        assert_relative_eq!(2e-18, Units::Length(LengthUnits::Meter(SIPrefix::Atto)).convert_to_base_units(2.0));
        assert_relative_eq!(2e-21, Units::Length(LengthUnits::Meter(SIPrefix::Zepto)).convert_to_base_units(2.0));
        assert_relative_eq!(2e-24, Units::Length(LengthUnits::Meter(SIPrefix::Yocto)).convert_to_base_units(2.0));
        assert_relative_eq!(2e-27, Units::Length(LengthUnits::Meter(SIPrefix::Ronto)).convert_to_base_units(2.0));
        assert_relative_eq!(2e-30, Units::Length(LengthUnits::Meter(SIPrefix::Quecto)).convert_to_base_units(2.0));

        assert_relative_eq!(0.6096, Units::Length(LengthUnits::Feet).convert_to_base_units(2.0));
        assert_relative_eq!(0.0508, Units::Length(LengthUnits::Inches).convert_to_base_units(2.0));
    }

    #[test]
    fn test_from_base_units_length() {
        assert_relative_eq!(2e-9, Units::Length(LengthUnits::Meter(SIPrefix::Giga)).convert_from_base_units(2.0));
        assert_relative_eq!(2000000.0, Units::Length(LengthUnits::Meter(SIPrefix::Micro)).convert_from_base_units(2.0));
        assert_relative_eq!(6.561679790026246, Units::Length(LengthUnits::Feet).convert_from_base_units(2.0));
        assert_relative_eq!(78.74015748031496, Units::Length(LengthUnits::Inches).convert_from_base_units(2.0));
    }

    #[test]
    fn test_display() {
        assert_eq!("", format!("{}", Units::None));
        assert_eq!("in", format!("{}", Units::Length(LengthUnits::Inches)));
        assert_eq!("km", format!("{}", Units::Length(LengthUnits::Meter(SIPrefix::Kilo))));
    }

    #[test]
    fn test_operator_precedence() {
        let u = Units::parse("m*g/s^2").unwrap();
        assert!(matches!(u, Units::Compound(_, UnitsOperator::Divide, _)));
        if let Units::Compound(a, _, b) = u {
            assert!(matches!(*a, Units::Compound(_, UnitsOperator::Multiply, _)));
            if let Units::Compound(a, _, b) = *a {
                assert!(matches!(*a, Units::Length(LengthUnits::Meter(SIPrefix::None))));
                assert!(matches!(*b, Units::Mass(MassUnits::Gram(SIPrefix::None))));
            } else {
                assert!(false);
            }
            if let Units::Compound(a, _, b) = *b {
                assert!(matches!(*a, Units::Time(TimeUnits::Second(SIPrefix::None))));
                assert!(matches!(*b, Units::Time(TimeUnits::Second(SIPrefix::None))));
            } else {
                assert!(false);
            }
        }
    }
}
