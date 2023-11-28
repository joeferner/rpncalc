use crate::error::RpnCalcError;
use crate::number::MagnitudeType;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SIPrefix {
    /// 10^30
    Quetta,
    /// 10^27
    Ronna,
    /// 10^24
    Yotta,
    /// 10^21
    Zetta,
    /// 10^18
    Exa,
    /// 10^15
    Peta,
    /// 10^12
    Tera,
    /// 10^9
    Giga,
    /// 10^6
    Mega,
    /// 10^3
    Kilo,
    /// 10^2
    Hecto,
    /// 10^1
    Deka,
    /// 10^0
    None,
    /// 10^-1
    Deci,
    /// 10^-2
    Centi,
    /// 10^-3
    Milli,
    /// 10^-6
    Micro,
    /// 10^-9
    Nano,
    /// 10^-12
    Pico,
    /// 10^-15
    Femto,
    /// 10^-18
    Atto,
    /// 10^-21
    Zepto,
    /// 10^-24
    Yocto,
    /// 10^-27
    Ronto,
    /// 10^-30
    Quecto,

    /// 2^10
    Kibi,
    /// 2^20
    Mebi,
    /// 2^30
    Gibi,
    /// 2^40
    Tebi,
    /// 2^50
    Pebi,
    /// 2^60
    Exbi,
    /// 2^70
    Zebi,
    /// 2^80
    Yobi,
}

impl SIPrefix {
    pub fn parse(str: &str) -> Result<SIPrefix, RpnCalcError> {
        return if str.is_empty() {
            Ok(SIPrefix::None)
        } else if str == "d" {
            Ok(SIPrefix::Deci)
        } else if str == "c" {
            Ok(SIPrefix::Centi)
        } else if str == "m" {
            Ok(SIPrefix::Milli)
        } else if str == "u" || str == "μ" {
            Ok(SIPrefix::Micro)
        } else if str == "n" {
            Ok(SIPrefix::Nano)
        } else if str == "p" {
            Ok(SIPrefix::Pico)
        } else if str == "f" {
            Ok(SIPrefix::Femto)
        } else if str == "a" {
            Ok(SIPrefix::Atto)
        } else if str == "z" {
            Ok(SIPrefix::Zepto)
        } else if str == "y" {
            Ok(SIPrefix::Yocto)
        } else if str == "r" {
            Ok(SIPrefix::Ronto)
        } else if str == "q" {
            Ok(SIPrefix::Quecto)
        } else if str == "da" {
            Ok(SIPrefix::Deka)
        } else if str == "h" {
            Ok(SIPrefix::Hecto)
        } else if str == "k" {
            Ok(SIPrefix::Kilo)
        } else if str == "M" {
            Ok(SIPrefix::Mega)
        } else if str == "G" {
            Ok(SIPrefix::Giga)
        } else if str == "T" {
            Ok(SIPrefix::Tera)
        } else if str == "P" {
            Ok(SIPrefix::Peta)
        } else if str == "E" {
            Ok(SIPrefix::Exa)
        } else if str == "Z" {
            Ok(SIPrefix::Zetta)
        } else if str == "Y" {
            Ok(SIPrefix::Yotta)
        } else if str == "R" {
            Ok(SIPrefix::Ronna)
        } else if str == "Q" {
            Ok(SIPrefix::Quetta)
        } else if str == "Ki" {
            Ok(SIPrefix::Kibi)
        } else if str == "Mi" {
            Ok(SIPrefix::Mebi)
        } else if str == "Gi" {
            Ok(SIPrefix::Gibi)
        } else if str == "Ti" {
            Ok(SIPrefix::Tebi)
        } else if str == "Pi" {
            Ok(SIPrefix::Pebi)
        } else if str == "Ei" {
            Ok(SIPrefix::Exbi)
        } else if str == "Zi" {
            Ok(SIPrefix::Zebi)
        } else if str == "Yi" {
            Ok(SIPrefix::Yobi)
        } else {
            Err(RpnCalcError::InvalidUnits(format!("unhandled SI prefix: {}", str)))
        };
    }

    pub fn multiplier(&self) -> MagnitudeType {
        let two: MagnitudeType = 2.0;
        return match self {
            SIPrefix::Quetta => 1e30,
            SIPrefix::Ronna => 1e27,
            SIPrefix::Yotta => 1e24,
            SIPrefix::Zetta => 1e21,
            SIPrefix::Exa => 1e18,
            SIPrefix::Peta => 1e15,
            SIPrefix::Tera => 1e12,
            SIPrefix::Giga => 1e9,
            SIPrefix::Mega => 1e6,
            SIPrefix::Kilo => 1e3,
            SIPrefix::Hecto => 1e2,
            SIPrefix::Deka => 1e1,
            SIPrefix::None => 1.0,
            SIPrefix::Deci => 1e-1,
            SIPrefix::Centi => 1e-2,
            SIPrefix::Milli => 1e-3,
            SIPrefix::Micro => 1e-6,
            SIPrefix::Nano => 1e-9,
            SIPrefix::Pico => 1e-12,
            SIPrefix::Femto => 1e-15,
            SIPrefix::Atto => 1e-18,
            SIPrefix::Zepto => 1e-21,
            SIPrefix::Yocto => 1e-24,
            SIPrefix::Ronto => 1e-27,
            SIPrefix::Quecto => 1e-30,

            SIPrefix::Kibi => two.powf(10.0),
            SIPrefix::Mebi => two.powf(20.0),
            SIPrefix::Gibi => two.powf(30.0),
            SIPrefix::Tebi => two.powf(40.0),
            SIPrefix::Pebi => two.powf(50.0),
            SIPrefix::Exbi => two.powf(60.0),
            SIPrefix::Zebi => two.powf(70.0),
            SIPrefix::Yobi => two.powf(80.0),
        };
    }
}

impl Display for SIPrefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SIPrefix::Quetta => write!(f, "Q"),
            SIPrefix::Ronna => write!(f, "R"),
            SIPrefix::Yotta => write!(f, "Y"),
            SIPrefix::Zetta => write!(f, "Z"),
            SIPrefix::Exa => write!(f, "E"),
            SIPrefix::Peta => write!(f, "P"),
            SIPrefix::Tera => write!(f, "T"),
            SIPrefix::Giga => write!(f, "G"),
            SIPrefix::Mega => write!(f, "M"),
            SIPrefix::Kilo => write!(f, "k"),
            SIPrefix::Hecto => write!(f, "h"),
            SIPrefix::Deka => write!(f, "da"),
            SIPrefix::None => write!(f, ""),
            SIPrefix::Deci => write!(f, "d"),
            SIPrefix::Centi => write!(f, "c"),
            SIPrefix::Milli => write!(f, "m"),
            SIPrefix::Micro => write!(f, "μ"),
            SIPrefix::Nano => write!(f, "n"),
            SIPrefix::Pico => write!(f, "p"),
            SIPrefix::Femto => write!(f, "f"),
            SIPrefix::Atto => write!(f, "a"),
            SIPrefix::Zepto => write!(f, "z"),
            SIPrefix::Yocto => write!(f, "y"),
            SIPrefix::Ronto => write!(f, "r"),
            SIPrefix::Quecto => write!(f, "q"),

            SIPrefix::Kibi => write!(f, "Ki"),
            SIPrefix::Mebi => write!(f, "Mi"),
            SIPrefix::Gibi => write!(f, "Gi"),
            SIPrefix::Tebi => write!(f, "Ti"),
            SIPrefix::Pebi => write!(f, "Pi"),
            SIPrefix::Exbi => write!(f, "Ei"),
            SIPrefix::Zebi => write!(f, "Zi"),
            SIPrefix::Yobi => write!(f, "Yi"),
        }
    }
}
