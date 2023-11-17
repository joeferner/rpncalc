use std::f64::consts::PI;
use std::fmt::{Display, Formatter};
use crate::angle_type::AngleType;
use crate::error::RpnCalcError;
use crate::units::Units;
use regex::Regex;

pub type MagnitudeType = f64;

#[derive(Clone, Debug)]
pub struct Number {
    magnitude: MagnitudeType,
    units: Units,
}

impl Number {
    pub fn from_str(str: &str) -> Result<Number, RpnCalcError> {
        let re = Regex::new(r"^(-?\d*\.?\d*)(.*)$").unwrap();
        let rs_results = re.captures(str);
        if let Some(rs_results) = rs_results {
            let magnitude_str = rs_results[1].trim();
            let units_str = rs_results[2].trim();

            if magnitude_str.len() == 0 {
                return Err(RpnCalcError::ParseStackItem(format!("could not parse {}", str)));
            }

            let units = Units::parse(units_str)?;

            let magnitude = match magnitude_str.parse::<f64>() {
                Ok(m) => m,
                Err(_err) => {
                    return Err(RpnCalcError::ParseStackItem(format!("could not parse {} to magnitude", magnitude_str)));
                }
            };
            return Ok(Number { magnitude, units });
        } else {
            return Err(RpnCalcError::ParseStackItem(format!("could not parse {}", str)));
        }
    }

    pub fn units(&self) -> Units {
        return self.units.clone();
    }

    pub fn magnitude(&self) -> MagnitudeType {
        return self.magnitude;
    }

    pub fn to_radians(&self, angle_mode: AngleType) -> Result<Number, RpnCalcError> {
        return match self.units {
            Units::None => Ok(to_radians(self.magnitude, angle_mode)),
            Units::Angle(angle_type) => Ok(to_radians(self.magnitude, angle_type)),
            _ => Err(RpnCalcError::InvalidUnits("expected angle or no units".to_string()))
        };
    }

    pub fn add(&self, other: &Number) -> Result<Number, RpnCalcError> {
        let magnitude: f64;
        if self.units == other.units {
            magnitude = self.magnitude + other.magnitude;
        } else {
            let a = self.units().convert_to_base_units(self.magnitude);
            let b = other.units().convert_to_base_units(other.magnitude);
            magnitude = other.units().convert_from_base_units(a + b);
        }
        return Ok(Number { magnitude, units: other.units.clone() });
    }

    pub fn subtract(&self, other: &Number) -> Result<Number, RpnCalcError> {
        return Ok(Number { magnitude: self.magnitude - other.magnitude, units: Units::None });
    }

    pub fn multiply(&self, other: &Number) -> Result<Number, RpnCalcError> {
        return Ok(Number { magnitude: self.magnitude * other.magnitude, units: Units::None });
    }

    pub fn divide(&self, other: &Number) -> Result<Number, RpnCalcError> {
        return Ok(Number { magnitude: self.magnitude / other.magnitude, units: Units::None });
    }

    pub fn pow(&self, other: &Number) -> Result<Number, RpnCalcError> {
        return Ok(Number { magnitude: self.magnitude.powf(other.magnitude), units: Units::None });
    }

    pub fn sin(&self, angle_mode: AngleType) -> Result<Number, RpnCalcError> {
        return Ok(Number { magnitude: self.to_radians(angle_mode)?.magnitude.sin(), units: Units::None });
    }

    pub fn cos(&self, angle_mode: AngleType) -> Result<Number, RpnCalcError> {
        return Ok(Number { magnitude: self.to_radians(angle_mode)?.magnitude.cos(), units: Units::None });
    }

    pub fn tan(&self, angle_mode: AngleType) -> Result<Number, RpnCalcError> {
        return Ok(Number { magnitude: self.to_radians(angle_mode)?.magnitude.tan(), units: Units::None });
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
        return Number { magnitude, units: Units::None };
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.magnitude)
    }
}

pub fn to_radians(magnitude: MagnitudeType, angle_type: AngleType) -> Number {
    return match angle_type {
        AngleType::Radians => Number { magnitude, units: Units::Angle(AngleType::Radians) },
        AngleType::Degrees => Number { magnitude: magnitude * PI / 180.0, units: Units::Angle(AngleType::Radians) }
    };
}