use crate::error::RpnCalcError;
use crate::number::MagnitudeType;
use crate::units::si_prefix::SIPrefix;
use crate::units::UnitTrait;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MassUnits {
    Gram(SIPrefix),
}

impl FromStr for MassUnits {
    type Err = RpnCalcError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        if let Some(prefix) = str.strip_suffix("g") {
            Ok(MassUnits::Gram(SIPrefix::parse(prefix)?))
        } else {
            Err(RpnCalcError::ParseStackItem("failed to parse".to_string()))
        }
    }
}

impl UnitTrait for MassUnits {
    fn convert_to_base_units(&self, n: MagnitudeType) -> MagnitudeType {
        match self {
            MassUnits::Gram(si) => n * si.multiplier(),
        }
    }

    fn convert_from_base_units(&self, n: MagnitudeType) -> MagnitudeType {
        match self {
            MassUnits::Gram(si) => n / si.multiplier(),
        }
    }
}

impl Display for MassUnits {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MassUnits::Gram(si) => write!(f, "{}g", si),
        }
    }
}
