use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::error::RpnCalcError;
use crate::number::MagnitudeType;
use crate::units::si_prefix::SIPrefix;
use crate::units::UnitTrait;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TemperatureUnits {
    Kelvin(SIPrefix),
    Celsius,
    Fahrenheit,
    Rankine,
}

impl FromStr for TemperatureUnits {
    type Err = RpnCalcError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        if let Some(prefix) = str.strip_suffix("K") {
            Ok(TemperatureUnits::Kelvin(SIPrefix::parse(prefix)?))
        } else if str == "C" || str == "°C" {
            Ok(TemperatureUnits::Celsius)
        } else if str == "F" || str == "°F" {
            Ok(TemperatureUnits::Fahrenheit)
        } else if str == "Ra" || str == "°Ra" {
            Ok(TemperatureUnits::Rankine)
        } else {
            Err(RpnCalcError::ParseStackItem("failed to parse".to_string()))
        }
    }
}

impl UnitTrait for TemperatureUnits {
    fn convert_to_base_units(&self, n: MagnitudeType) -> MagnitudeType {
        return match self {
            TemperatureUnits::Kelvin(si) => n * si.multiplier(),
            TemperatureUnits::Celsius => n + 273.15,
            TemperatureUnits::Fahrenheit => (5.0 / 9.0) * (n + 459.67),
            TemperatureUnits::Rankine => (5.0 / 9.0) * n,
        };
    }

    fn convert_from_base_units(&self, n: MagnitudeType) -> MagnitudeType {
        return match self {
            TemperatureUnits::Kelvin(si) => n / si.multiplier(),
            TemperatureUnits::Celsius => n - 273.15,
            TemperatureUnits::Fahrenheit => (1.8 * n) - 459.67,
            TemperatureUnits::Rankine => 1.8 * n
        };
    }
}

impl Display for TemperatureUnits {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TemperatureUnits::Kelvin(si) => write!(f, "{}K", si),
            TemperatureUnits::Celsius => write!(f, "°C"),
            TemperatureUnits::Fahrenheit => write!(f, "°F"),
            TemperatureUnits::Rankine => write!(f, "°Ra"),
        }
    }
}