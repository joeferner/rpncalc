use std::f64::consts::PI;
use std::fmt::{Display, Formatter};
use crate::rpn_calc::{AngleMode, RpnCalcError};

pub type MagnitudeType = f64;

#[derive(Clone)]
pub struct Number {
    magnitude: MagnitudeType,
}

impl Number {
    pub fn from_str(str: &str) -> Result<Number, RpnCalcError> {
        let magnitude = match str.parse::<f64>() {
            Ok(m) => m,
            Err(_err) => {
                return Err(RpnCalcError::ParseStackItem(format!("could not parse {} to magnitude", str)));
            }
        };
        return Ok(Number { magnitude });
    }

    pub fn magnitude(&self) -> MagnitudeType {
        return self.magnitude;
    }

    pub fn to_radians(&self, angle_mode: AngleMode) -> Result<Number, RpnCalcError> {
        return match angle_mode {
            AngleMode::Radians => Ok(Number { magnitude: self.magnitude }),
            AngleMode::Degrees => Ok(Number { magnitude: self.magnitude * PI / 180.0 })
        };
    }

    pub fn add(&self, other: &Number) -> Result<Number, RpnCalcError> {
        return Ok(Number { magnitude: self.magnitude + other.magnitude });
    }

    pub fn subtract(&self, other: &Number) -> Result<Number, RpnCalcError> {
        return Ok(Number { magnitude: self.magnitude - other.magnitude });
    }

    pub fn multiply(&self, other: &Number) -> Result<Number, RpnCalcError> {
        return Ok(Number { magnitude: self.magnitude * other.magnitude });
    }

    pub fn divide(&self, other: &Number) -> Result<Number, RpnCalcError> {
        return Ok(Number { magnitude: self.magnitude / other.magnitude });
    }

    pub fn pow(&self, other: &Number) -> Result<Number, RpnCalcError> {
        return Ok(Number { magnitude: self.magnitude.powf(other.magnitude) });
    }

    pub fn sin(&self, angle_mode: AngleMode) -> Result<Number, RpnCalcError> {
        return Ok(Number { magnitude: self.to_radians(angle_mode)?.magnitude.sin() });
    }

    pub fn cos(&self, angle_mode: AngleMode) -> Result<Number, RpnCalcError> {
        return Ok(Number { magnitude: self.to_radians(angle_mode)?.magnitude.cos() });
    }

    pub fn tan(&self, angle_mode: AngleMode) -> Result<Number, RpnCalcError> {
        return Ok(Number { magnitude: self.to_radians(angle_mode)?.magnitude.tan() });
    }
}

impl From<f64> for Number {
    fn from(magnitude: f64) -> Self {
        return Number { magnitude };
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.magnitude)
    }
}