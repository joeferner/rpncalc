mod angle;
mod length;
mod mass;
mod operator;
mod si_prefix;
mod temperature;
mod time;

pub use self::angle::{degrees_to_radians, radians_to_degrees, AngleUnits};
pub use self::length::LengthUnits;
pub use self::mass::MassUnits;
pub use self::operator::UnitsOperator;
pub use self::si_prefix::SIPrefix;
pub use self::temperature::TemperatureUnits;
pub use self::time::TimeUnits;

use crate::error::RpnCalcError;
use crate::number::MagnitudeType;
use std::fmt::{Display, Formatter};

pub trait UnitTrait {
    fn convert_to_base_units(&self, n: MagnitudeType) -> MagnitudeType;
    fn convert_from_base_units(&self, n: MagnitudeType) -> MagnitudeType;
}

#[derive(Clone, Debug, PartialEq)]
pub enum Units {
    None,
    Length(LengthUnits),
    Mass(MassUnits),
    Time(TimeUnits),
    Temperature(TemperatureUnits),
    Angle(AngleUnits),
    Compound(Box<Units>, UnitsOperator, Box<Units>),
}

impl Units {
    pub fn parse(str: &str) -> Result<Units, RpnCalcError> {
        return if str.is_empty() {
            Ok(Units::None)
        } else if let Some(parts) = str.split_once('/') {
            Ok(Units::Compound(
                Box::new(Units::parse(parts.0)?),
                UnitsOperator::Divide,
                Box::new(Units::parse(parts.1)?),
            ))
        } else if let Some(parts) = str.split_once('*') {
            Ok(Units::Compound(
                Box::new(Units::parse(parts.0)?),
                UnitsOperator::Multiply,
                Box::new(Units::parse(parts.1)?),
            ))
        } else if let Some(parts) = str.split_once('^') {
            if parts.1 == "2" {
                Ok(Units::Compound(
                    Box::new(Units::parse(parts.0)?),
                    UnitsOperator::Multiply,
                    Box::new(Units::parse(parts.0)?),
                ))
            } else {
                Err(RpnCalcError::ParseStackItem(format!("parse units {}", str)))
            }
        } else if let Ok(angle) = str.parse::<AngleUnits>() {
            Ok(Units::Angle(angle))
        } else if let Ok(temp) = str.parse::<TemperatureUnits>() {
            Ok(Units::Temperature(temp))
        } else if let Ok(l) = str.parse::<LengthUnits>() {
            Ok(Units::Length(l))
        } else if let Ok(m) = str.parse::<MassUnits>() {
            Ok(Units::Mass(m))
        } else if let Ok(t) = str.parse::<TimeUnits>() {
            Ok(Units::Time(t))
        } else {
            Err(RpnCalcError::ParseStackItem(format!("parse units \"{}\"", str)))
        };
    }

    pub fn can_add_subtract(&self, other: &Units) -> bool {
        return match self {
            Units::None => true,
            Units::Length(_) => matches!(other, Units::None | Units::Length(_)),
            Units::Mass(_) => matches!(other, Units::None | Units::Mass(_)),
            Units::Time(_) => matches!(other, Units::None | Units::Time(_)),
            Units::Temperature(_) => matches!(other, Units::None | Units::Temperature(_)),
            Units::Angle(_) => matches!(other, Units::None | Units::Angle(_)),
            Units::Compound(a, op, b) => match other {
                Units::None => true,
                Units::Compound(other_a, other_op, other_b) => {
                    op == other_op && a.can_add_subtract(other_a) && b.can_add_subtract(other_b)
                }
                _ => false,
            },
        };
    }
}

impl UnitTrait for Units {
    fn convert_to_base_units(&self, n: MagnitudeType) -> MagnitudeType {
        return match self {
            Units::None => n,
            Units::Length(u) => u.convert_to_base_units(n),
            Units::Mass(u) => u.convert_to_base_units(n),
            Units::Time(u) => u.convert_to_base_units(n),
            Units::Temperature(u) => u.convert_to_base_units(n),
            Units::Angle(u) => u.convert_to_base_units(n),
            Units::Compound(a, op, b) => {
                let converted_a = a.convert_to_base_units(n);
                let converted_b = b.convert_to_base_units(1.0);
                match op {
                    UnitsOperator::Divide => converted_a / converted_b,
                    UnitsOperator::Multiply => converted_a * converted_b,
                }
            }
        };
    }

    fn convert_from_base_units(&self, n: MagnitudeType) -> MagnitudeType {
        return match self {
            Units::None => n,
            Units::Length(u) => u.convert_from_base_units(n),
            Units::Mass(u) => u.convert_from_base_units(n),
            Units::Time(u) => u.convert_from_base_units(n),
            Units::Temperature(u) => u.convert_from_base_units(n),
            Units::Angle(u) => u.convert_from_base_units(n),
            Units::Compound(a, op, b) => {
                let converted_a = a.convert_from_base_units(n);
                let converted_b = b.convert_from_base_units(1.0);
                match op {
                    UnitsOperator::Divide => converted_a / converted_b,
                    UnitsOperator::Multiply => converted_a * converted_b,
                }
            }
        };
    }
}

impl Display for Units {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Units::None => write!(f, ""),
            Units::Length(u) => write!(f, "{}", u),
            Units::Mass(u) => write!(f, "{}", u),
            Units::Time(u) => write!(f, "{}", u),
            Units::Temperature(u) => write!(f, "{}", u),
            Units::Angle(u) => write!(f, "{}", u),
            Units::Compound(a, op, b) => {
                write!(f, "{}{}{}", *a, op, *b)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::si_prefix::SIPrefix;
    use approx::assert_relative_eq;

    pub fn feet_per_min_sq() -> Units {
        return Units::Compound(
            Box::new(Units::Length(LengthUnits::Foot)),
            UnitsOperator::Divide,
            Box::new(Units::Compound(
                Box::new(Units::Time(TimeUnits::Minute)),
                UnitsOperator::Multiply,
                Box::new(Units::Time(TimeUnits::Minute)),
            )),
        );
    }

    #[test]
    fn test_deg() {
        let r = Units::parse("deg").unwrap();
        assert!(matches!(r, Units::Angle(AngleUnits::Degrees)));
    }

    #[test]
    fn test_rad() {
        let r = Units::parse("rad").unwrap();
        assert!(matches!(r, Units::Angle(AngleUnits::Radians)));
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
        assert_relative_eq!(
            2e30,
            Units::Length(LengthUnits::Meter(SIPrefix::Quetta)).convert_to_base_units(2.0)
        );
        assert_relative_eq!(
            2e27,
            Units::Length(LengthUnits::Meter(SIPrefix::Ronna)).convert_to_base_units(2.0)
        );
        assert_relative_eq!(
            2e24,
            Units::Length(LengthUnits::Meter(SIPrefix::Yotta)).convert_to_base_units(2.0)
        );
        assert_relative_eq!(
            2e21,
            Units::Length(LengthUnits::Meter(SIPrefix::Zetta)).convert_to_base_units(2.0)
        );
        assert_relative_eq!(
            2e18,
            Units::Length(LengthUnits::Meter(SIPrefix::Exa)).convert_to_base_units(2.0)
        );
        assert_relative_eq!(
            2e15,
            Units::Length(LengthUnits::Meter(SIPrefix::Peta)).convert_to_base_units(2.0)
        );
        assert_relative_eq!(
            2e12,
            Units::Length(LengthUnits::Meter(SIPrefix::Tera)).convert_to_base_units(2.0)
        );
        assert_relative_eq!(
            2e9,
            Units::Length(LengthUnits::Meter(SIPrefix::Giga)).convert_to_base_units(2.0)
        );
        assert_relative_eq!(
            2e6,
            Units::Length(LengthUnits::Meter(SIPrefix::Mega)).convert_to_base_units(2.0)
        );
        assert_relative_eq!(
            2000.0,
            Units::Length(LengthUnits::Meter(SIPrefix::Kilo)).convert_to_base_units(2.0)
        );
        assert_relative_eq!(
            200.0,
            Units::Length(LengthUnits::Meter(SIPrefix::Hecto)).convert_to_base_units(2.0)
        );
        assert_relative_eq!(
            20.0,
            Units::Length(LengthUnits::Meter(SIPrefix::Deka)).convert_to_base_units(2.0)
        );
        assert_relative_eq!(
            2.0,
            Units::Length(LengthUnits::Meter(SIPrefix::None)).convert_to_base_units(2.0)
        );
        assert_relative_eq!(
            0.2,
            Units::Length(LengthUnits::Meter(SIPrefix::Deci)).convert_to_base_units(2.0)
        );
        assert_relative_eq!(
            0.02,
            Units::Length(LengthUnits::Meter(SIPrefix::Centi)).convert_to_base_units(2.0)
        );
        assert_relative_eq!(
            0.002,
            Units::Length(LengthUnits::Meter(SIPrefix::Milli)).convert_to_base_units(2.0)
        );
        assert_relative_eq!(
            2e-6,
            Units::Length(LengthUnits::Meter(SIPrefix::Micro)).convert_to_base_units(2.0)
        );
        assert_relative_eq!(
            2e-9,
            Units::Length(LengthUnits::Meter(SIPrefix::Nano)).convert_to_base_units(2.0)
        );
        assert_relative_eq!(
            2e-12,
            Units::Length(LengthUnits::Meter(SIPrefix::Pico)).convert_to_base_units(2.0)
        );
        assert_relative_eq!(
            2e-15,
            Units::Length(LengthUnits::Meter(SIPrefix::Femto)).convert_to_base_units(2.0)
        );
        assert_relative_eq!(
            2e-18,
            Units::Length(LengthUnits::Meter(SIPrefix::Atto)).convert_to_base_units(2.0)
        );
        assert_relative_eq!(
            2e-21,
            Units::Length(LengthUnits::Meter(SIPrefix::Zepto)).convert_to_base_units(2.0)
        );
        assert_relative_eq!(
            2e-24,
            Units::Length(LengthUnits::Meter(SIPrefix::Yocto)).convert_to_base_units(2.0)
        );
        assert_relative_eq!(
            2e-27,
            Units::Length(LengthUnits::Meter(SIPrefix::Ronto)).convert_to_base_units(2.0)
        );
        assert_relative_eq!(
            2e-30,
            Units::Length(LengthUnits::Meter(SIPrefix::Quecto)).convert_to_base_units(2.0)
        );

        assert_relative_eq!(0.6096, Units::Length(LengthUnits::Foot).convert_to_base_units(2.0));
        assert_relative_eq!(0.0508, Units::Length(LengthUnits::Inch).convert_to_base_units(2.0));

        assert_relative_eq!(9.8, feet_per_min_sq().convert_to_base_units(115748.03149606299));
    }

    #[test]
    fn test_from_base_units_length() {
        assert_relative_eq!(
            2e-9,
            Units::Length(LengthUnits::Meter(SIPrefix::Giga)).convert_from_base_units(2.0)
        );
        assert_relative_eq!(
            2000000.0,
            Units::Length(LengthUnits::Meter(SIPrefix::Micro)).convert_from_base_units(2.0)
        );
        assert_relative_eq!(
            6.561679790026246,
            Units::Length(LengthUnits::Foot).convert_from_base_units(2.0)
        );
        assert_relative_eq!(
            78.74015748031496,
            Units::Length(LengthUnits::Inch).convert_from_base_units(2.0)
        );
        assert_relative_eq!(115748.03149606299, feet_per_min_sq().convert_from_base_units(9.8));
    }

    #[test]
    fn test_display() {
        assert_eq!("", format!("{}", Units::None));
        assert_eq!("in", format!("{}", Units::Length(LengthUnits::Inch)));
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
