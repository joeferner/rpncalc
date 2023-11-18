use std::fmt::{Display, Formatter};
use crate::number::MagnitudeType;
use crate::units::si_prefix::SIPrefix;
use crate::units::UnitTrait;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MassUnits {
    Gram(SIPrefix)
}

impl UnitTrait for MassUnits {
    fn convert_to_base_units(&self, n: MagnitudeType) -> MagnitudeType {
        match self {
            MassUnits::Gram(si) => n * si.multiplier()
        }
    }

    fn convert_from_base_units(&self, n: MagnitudeType) -> MagnitudeType {
        match self {
            MassUnits::Gram(si) => n / si.multiplier()
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
