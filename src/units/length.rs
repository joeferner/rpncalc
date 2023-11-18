use std::fmt::{Display, Formatter};
use crate::number::MagnitudeType;
use crate::units::si_prefix::SIPrefix;
use crate::units::UnitTrait;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LengthUnits {
    Meter(SIPrefix),
    Inches,
    Feet,
}

impl UnitTrait for LengthUnits {
    fn convert_to_base_units(&self, n: MagnitudeType) -> MagnitudeType {
        return match self {
            LengthUnits::Meter(si) => n * si.multiplier(),
            LengthUnits::Feet => n * 0.3048,
            LengthUnits::Inches => n * 0.0254,
        };
    }

    fn convert_from_base_units(&self, n: MagnitudeType) -> MagnitudeType {
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