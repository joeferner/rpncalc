use crate::error::RpnCalcError;
use crate::number::MagnitudeType;
use crate::units::si_prefix::SIPrefix;
use crate::units::UnitTrait;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BitUnits {
    Bit(SIPrefix),
}

impl FromStr for BitUnits {
    type Err = RpnCalcError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        if let Some(prefix) = str.strip_suffix('b') {
            Ok(BitUnits::Bit(SIPrefix::parse(prefix)?))
        } else {
            Err(RpnCalcError::ParseStackItem("failed to parse".to_string()))
        }
    }
}

impl UnitTrait for BitUnits {
    fn convert_to_base_units(&self, n: MagnitudeType) -> MagnitudeType {
        match self {
            BitUnits::Bit(si) => n * si.multiplier(),
        }
    }

    fn convert_from_base_units(&self, n: MagnitudeType) -> MagnitudeType {
        match self {
            BitUnits::Bit(si) => n / si.multiplier(),
        }
    }
}

impl Display for BitUnits {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BitUnits::Bit(si) => write!(f, "{}B", si),
        }
    }
}
