use std::f64::consts::PI;
use std::fmt::{Display, Formatter};

use regex::Regex;

use crate::error::RpnCalcError;
use crate::units::angle::{degrees_to_radians, AngleUnits};
use crate::units::{UnitTrait, Units};

pub type MagnitudeType = f64;

pub const MAGNITUDE_TYPE_PI: f64 = PI;

#[derive(Clone, Debug)]
pub struct Number {
    pub magnitude: MagnitudeType,
    pub units: Units,
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
                    return Err(RpnCalcError::ParseStackItem(format!(
                        "could not parse {} to magnitude",
                        magnitude_str
                    )));
                }
            };
            return Ok(Number { magnitude, units });
        } else {
            return Err(RpnCalcError::ParseStackItem(format!("could not parse {}", str)));
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
        write!(f, "{} {}", self.magnitude, self.units)
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
