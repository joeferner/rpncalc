use std::f64::consts::PI;
use std::fmt::{Display, Formatter};

use regex::Regex;

use crate::error::RpnCalcError;
use crate::units::{degrees_to_radians, AngleUnits};
use crate::units::{UnitTrait, Units};

pub type MagnitudeType = f64;
pub type MagnitudeTypeInteger = i64;

pub const MAGNITUDE_TYPE_PI: f64 = PI;

#[derive(Clone, Debug)]
pub struct Number {
    pub magnitude: MagnitudeType,
    pub units: Units,
}

impl Number {
    pub const MAX_DISPLAY_BASE: u8 = 26 + 10; // letters in alphabet + digits

    pub fn to_string_format(&self, width: usize, base: u16) -> String {
        let mut units_str = format!("{}", self.units);
        if !units_str.is_empty() {
            units_str = format!(" {}", units_str);
        }
        let number_width = width - units_str.len();
        let magnitude_str = Self::magnitude_to_string(self.magnitude, number_width, base);
        return format!("{}{}", magnitude_str, units_str);
    }

    fn digit_to_char(u: u8) -> char {
        if u < 10 {
            return (u + b'0') as char;
        } else if u < Number::MAX_DISPLAY_BASE {
            return (u - 10 + b'a') as char;
        } else {
            unreachable!("number out of range");
        }
    }

    fn char_to_digit(ch: char) -> Result<u8, RpnCalcError> {
        if ch.is_ascii_digit() {
            return Ok(ch as u8 - b'0');
        } else if ch.is_ascii_lowercase() {
            return Ok(ch as u8 - b'a' + 10);
        } else if ch.is_ascii_uppercase() {
            return Ok(ch as u8 - b'A' + 10);
        } else {
            return Err(RpnCalcError::GenericError(format!("invalid digit character {}", ch)));
        }
    }

    fn magnitude_to_string(n: MagnitudeType, width: usize, base: u16) -> String {
        let base = base as i64;
        if base == 10 {
            return format!("{}", n);
        }
        let sign_str = if n >= 0.0 { "" } else { "-" };
        let n = n.abs();
        let base_str = match base {
            2 => "0b".to_string(),
            8 => "0o".to_string(),
            16 => "0x".to_string(),
            _ => format!("{}#", base),
        };

        let mut whole_number = n.floor() as MagnitudeTypeInteger;
        let mut decimal_number = n - whole_number as MagnitudeType;

        // whole number part
        let mut whole_number_str = "".to_string();
        if whole_number == 0 {
            whole_number_str.push('0');
        } else {
            while whole_number > 0 {
                let digit = whole_number % base;
                whole_number_str.push(Number::digit_to_char(digit as u8));
                whole_number = (whole_number - digit) / base;
            }
            whole_number_str = whole_number_str.chars().rev().collect::<String>();
        }

        // decimal part
        let mut decimal_number_str = "".to_string();
        if decimal_number > MagnitudeType::EPSILON {
            decimal_number_str.push('.');
            while decimal_number > MagnitudeType::EPSILON || decimal_number_str.len() > width {
                decimal_number *= base as MagnitudeType;
                let digit = decimal_number.floor();
                decimal_number_str.push(Number::digit_to_char(digit as u8));
                decimal_number -= digit;
            }
        }

        return format!("{}{}{}{}", sign_str, base_str, whole_number_str, decimal_number_str);
    }

    pub fn from_str(str: &str) -> Result<Number, RpnCalcError> {
        let mut my_str = str.to_string();

        let negative;
        if my_str.starts_with('-') {
            negative = true;
            my_str = my_str[1..].to_string();
        } else {
            negative = false;
        }

        let mut base = 10;
        let mut regex = r"^(\d*\.?\d*)(.*)$";
        if my_str.starts_with("0b") {
            base = 2;
            regex = r"^([01]*\.?[01]*)(.*)$";
            my_str = my_str[2..].to_string();
        } else if my_str.starts_with("0o") {
            base = 8;
            regex = r"^([0-8]*\.?[0-8]*)(.*)$";
            my_str = my_str[2..].to_string();
        } else if my_str.starts_with("0x") {
            base = 16;
            regex = r"^([0-9a-fA-F]*\.?[0-9a-fA-F]*)(.*)$";
            my_str = my_str[2..].to_string();
        } else {
            let re = Regex::new(r"^([0-9]+)#(.*)$").unwrap();
            let rs_results = re.captures(my_str.as_str());
            if let Some(rs_results) = rs_results {
                let base_str = rs_results[1].trim();
                let number_str = rs_results[2].trim();

                base = base_str
                    .parse::<u8>()
                    .map_err(|_err| RpnCalcError::ParseStackItem(format!("could not parse {}: invalid base", str)))?;
                if base == 0 || base > Number::MAX_DISPLAY_BASE {
                    return Err(RpnCalcError::ParseStackItem(format!(
                        "could not parse {}: invalid base, must be between {} and {}",
                        str,
                        1,
                        Number::MAX_DISPLAY_BASE
                    )));
                }
                regex = r"^([0-9a-zA-Z]*\.?[0-9a-zA-Z]*)(.*)$";
                my_str = number_str.to_string();
            }
        }
        let re = Regex::new(regex).unwrap();
        let rs_results = re.captures(my_str.as_str());
        if let Some(rs_results) = rs_results {
            let magnitude_str = rs_results[1].trim();
            let units_str = rs_results[2].trim();

            if magnitude_str.is_empty() {
                return Err(RpnCalcError::ParseStackItem(format!("could not parse {}", str)));
            }

            let units = Units::parse(units_str)?;

            let mut magnitude = Number::parse_magnitude_str(magnitude_str, base).map_err(|err| {
                RpnCalcError::ParseStackItem(format!("could not parse {} to magnitude: {}", str, err))
            })?;
            if negative {
                magnitude *= -1.0;
            }
            return Ok(Number { magnitude, units });
        } else {
            return Err(RpnCalcError::ParseStackItem(format!("could not parse {}", str)));
        }
    }

    fn parse_magnitude_str(str: &str, base: u8) -> Result<MagnitudeType, RpnCalcError> {
        if base == 10 {
            return str
                .parse::<f64>()
                .map_err(|err| RpnCalcError::GenericError(format!("{}", err)));
        } else {
            let whole_part;
            let decimal_number;
            if let Some((a, b)) = str.split_once('.') {
                whole_part = a;
                decimal_number = b;
            } else {
                whole_part = str;
                decimal_number = "";
            }

            let mut result: MagnitudeType = 0.0;
            let mut multiplier: MagnitudeType = 1.0;
            for ch in whole_part.chars().rev() {
                let digit = Number::char_to_digit(ch)?;
                if digit >= base {
                    return Err(RpnCalcError::GenericError(format!(
                        "expected digit less than {} but found {}",
                        base, digit
                    )));
                }
                result += digit as MagnitudeType * multiplier;
                multiplier *= base as MagnitudeType;
            }

            multiplier = 1.0 / base as MagnitudeType;
            for ch in decimal_number.chars() {
                let digit = Number::char_to_digit(ch)?;
                if digit >= base {
                    return Err(RpnCalcError::GenericError(format!(
                        "expected digit less than {} but found {}",
                        base, digit
                    )));
                }
                result += digit as MagnitudeType * multiplier;
                multiplier /= base as MagnitudeType;
            }

            return Ok(result);
        }
    }

    pub fn to_radians(&self, angle_mode: AngleUnits) -> Result<Number, RpnCalcError> {
        return match self.units {
            Units::None => Ok(to_radians(self.magnitude, angle_mode)),
            Units::Angle(angle_type) => Ok(to_radians(self.magnitude, angle_type)),
            _ => Err(RpnCalcError::InvalidUnits("expected angle or no units".to_string())),
        };
    }

    pub fn add(&self, other: &Number) -> Result<Number, RpnCalcError> {
        let magnitude: f64;
        if self.units == other.units || matches!(self.units, Units::None) || matches!(other.units, Units::None) {
            magnitude = self.magnitude + other.magnitude;
            let units = if matches!(self.units, Units::None) {
                other.units.clone()
            } else {
                self.units.clone()
            };
            return Ok(Number { magnitude, units });
        } else {
            if !self.units.can_add_subtract(&other.units) {
                return Err(RpnCalcError::IncompatibleUnits(self.units.clone(), other.units.clone()));
            }

            let a = self.units.convert_to_base_units(self.magnitude);
            let b = other.units.convert_to_base_units(other.magnitude);
            magnitude = other.units.convert_from_base_units(a + b);
            return Ok(Number {
                magnitude,
                units: other.units.clone(),
            });
        }
    }

    pub fn subtract(&self, other: &Number) -> Result<Number, RpnCalcError> {
        let other = other.negate()?;
        return self.add(&other);
    }

    pub fn multiply(&self, other: &Number) -> Result<Number, RpnCalcError> {
        return Ok(Number {
            magnitude: self.magnitude * other.magnitude,
            units: Units::None,
        });
    }

    pub fn divide(&self, other: &Number) -> Result<Number, RpnCalcError> {
        if other.magnitude == 0.0 {
            return Err(RpnCalcError::DivideByZero);
        }
        return Ok(Number {
            magnitude: self.magnitude / other.magnitude,
            units: Units::None,
        });
    }

    pub fn pow(&self, other: &Number) -> Result<Number, RpnCalcError> {
        return Ok(Number {
            magnitude: self.magnitude.powf(other.magnitude),
            units: Units::None,
        });
    }

    pub fn sin(&self, angle_mode: AngleUnits) -> Result<Number, RpnCalcError> {
        return Ok(Number {
            magnitude: self.to_radians(angle_mode)?.magnitude.sin(),
            units: Units::None,
        });
    }

    pub fn cos(&self, angle_mode: AngleUnits) -> Result<Number, RpnCalcError> {
        return Ok(Number {
            magnitude: self.to_radians(angle_mode)?.magnitude.cos(),
            units: Units::None,
        });
    }

    pub fn tan(&self, angle_mode: AngleUnits) -> Result<Number, RpnCalcError> {
        return Ok(Number {
            magnitude: self.to_radians(angle_mode)?.magnitude.tan(),
            units: Units::None,
        });
    }

    pub fn negate(&self) -> Result<Number, RpnCalcError> {
        return Ok(Number {
            magnitude: -self.magnitude,
            units: self.units.clone(),
        });
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        let delta = (self.magnitude - other.magnitude).abs();
        return delta <= f64::EPSILON;
    }
}

impl From<f64> for Number {
    fn from(magnitude: f64) -> Self {
        return Number {
            magnitude,
            units: Units::None,
        };
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut units_str = format!("{}", self.units);
        if !units_str.is_empty() {
            units_str = format!(" {}", units_str);
        }
        write!(f, "{}{}", self.magnitude, units_str)
    }
}

pub fn to_radians(magnitude: MagnitudeType, angle_type: AngleUnits) -> Number {
    return match angle_type {
        AngleUnits::Radians => Number {
            magnitude,
            units: Units::Angle(AngleUnits::Radians),
        },
        AngleUnits::Degrees => Number {
            magnitude: degrees_to_radians(magnitude),
            units: Units::Angle(AngleUnits::Radians),
        },
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_display_binary() {
        assert_eq!(
            "0b111100011011.1011",
            Number::from_str("3867.6875").unwrap().to_string_format(1000, 2)
        );
        assert_eq!(
            "-0b111100011011.1011",
            Number::from_str("-3867.6875").unwrap().to_string_format(1000, 2)
        );
    }

    #[test]
    fn test_parse_binary() {
        assert_eq!(3867.6875, Number::from_str("0b111100011011.1011").unwrap().magnitude);
        assert_eq!(-3867.6875, Number::from_str("-0b111100011011.1011").unwrap().magnitude);
    }

    #[test]
    fn test_display_octal() {
        assert_eq!("0o7433", Number::from_str("3867").unwrap().to_string_format(1000, 8));
        assert_eq!("-0o7433", Number::from_str("-3867").unwrap().to_string_format(1000, 8));
    }

    #[test]
    fn test_parse_octal() {
        assert_eq!(3867.0, Number::from_str("0o7433").unwrap().magnitude);
        assert_eq!(-3867.0, Number::from_str("-0o7433").unwrap().magnitude);
    }

    #[test]
    fn test_display_hex() {
        assert_eq!(
            "0xf1a.b",
            Number::from_str("3866.6875").unwrap().to_string_format(1000, 16)
        );
        assert_eq!(
            "-0xf1a.b",
            Number::from_str("-3866.6875").unwrap().to_string_format(1000, 16)
        );
    }

    #[test]
    fn test_parse_hex() {
        assert_eq!(3866.6875, Number::from_str("0xf1a.b").unwrap().magnitude);
        assert_eq!(-3866.6875, Number::from_str("-0xf1a.b").unwrap().magnitude);
    }

    #[test]
    fn test_display_radix() {
        assert_eq!(
            "4#0.3213",
            Number::from_str("0.90234375").unwrap().to_string_format(1000, 4)
        );
        assert_eq!(
            "-4#0.3213",
            Number::from_str("-0.90234375").unwrap().to_string_format(1000, 4)
        );
    }

    #[test]
    fn test_parse_radix() {
        assert_relative_eq!(0.90234375, Number::from_str("4#0.3213").unwrap().magnitude);
        assert_relative_eq!(-0.90234375, Number::from_str("-4#0.3213").unwrap().magnitude);
    }
}
