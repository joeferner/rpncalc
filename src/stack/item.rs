use std::fmt::Display;
use std::fmt::{self};

use anyhow::{anyhow, Result};
use log::warn;
use num_format::ToFormattedString;

use crate::state::RpnState;

#[derive(Clone, Debug)]
pub enum StackItem {
    Number(f64),
    Undefined,
}

impl StackItem {
    pub fn from_str(s: &str) -> Result<StackItem> {
        if let Ok(v) = s.parse::<f64>() {
            return Ok(StackItem::Number(v));
        }
        Err(anyhow!("parse error: {s}"))
    }

    pub fn add(&self, other: &StackItem) -> Result<StackItem> {
        match self {
            StackItem::Number(value) => match other {
                StackItem::Number(other_value) => Ok(StackItem::Number(value + other_value)),
                StackItem::Undefined => Ok(StackItem::Undefined),
            },
            StackItem::Undefined => Ok(StackItem::Undefined),
        }
    }

    pub fn subtract(&self, other: &StackItem) -> Result<StackItem> {
        match self {
            StackItem::Number(value) => match other {
                StackItem::Number(other_value) => Ok(StackItem::Number(value - other_value)),
                StackItem::Undefined => Ok(StackItem::Undefined),
            },
            StackItem::Undefined => Ok(StackItem::Undefined),
        }
    }

    pub fn multiply(&self, other: &StackItem) -> Result<StackItem> {
        match self {
            StackItem::Number(value) => match other {
                StackItem::Number(other_value) => Ok(StackItem::Number(value * other_value)),
                StackItem::Undefined => Ok(StackItem::Undefined),
            },
            StackItem::Undefined => Ok(StackItem::Undefined),
        }
    }

    pub fn divide(&self, other: &StackItem) -> Result<StackItem> {
        match self {
            StackItem::Number(value) => match other {
                StackItem::Number(other_value) => {
                    if *other_value == 0.0 {
                        Ok(StackItem::Undefined)
                    } else {
                        Ok(StackItem::Number(value / other_value))
                    }
                }
                StackItem::Undefined => Ok(StackItem::Undefined),
            },
            StackItem::Undefined => Ok(StackItem::Undefined),
        }
    }

    pub fn to_string_opts(&self, opts: &StackItemToStringOpts, state: &RpnState) -> String {
        match self {
            StackItem::Number(n) => {
                let whole_part = *n as i128;
                let abs_whole_part = whole_part.abs();

                if opts.base == 10 {
                    return to_string_opts_base10(*n, opts, state);
                }

                if *n >= 0.0 && ((whole_part as f64) - n).abs() < f64::EPSILON * 1000.0 {
                    if opts.base == 2 {
                        return group_digits(format!("{:b}", abs_whole_part), 4, true);
                    } else if opts.base == 8 {
                        return group_digits(format!("{:o}", abs_whole_part), 4, false);
                    } else if opts.base == 16 {
                        return group_digits(format!("{:x}", abs_whole_part), 4, false);
                    }
                }
                "".to_string()
            }
            StackItem::Undefined => "Undefined".to_string(),
        }
    }
}

fn to_string_opts_base10(n: f64, opts: &StackItemToStringOpts, state: &RpnState) -> String {
    let whole_part = n as i128;
    let abs_whole_part = whole_part.abs();
    let sign = if n < 0.0 { "-" } else { "" };
    let abs_decimal_part = ((abs_whole_part as f64) - n.abs()).abs();
    if abs_decimal_part > state.scientific_notation_limit {
        format!("{:e}", n)
    } else if abs_decimal_part >= 1.0 {
        warn!(
            "decimal part should not be greater than or equal to 1.0: (n: {}, decimal part: {})",
            n, abs_decimal_part
        );
        format!("{}", n)
    } else if abs_decimal_part < f64::EPSILON * 1000.0 {
        format!(
            "{}{}",
            sign,
            abs_whole_part.to_formatted_string(&state.locale)
        )
    } else {
        format!(
            "{}{}.{}",
            sign,
            abs_whole_part.to_formatted_string(&state.locale),
            format!(
                "{:.1$}",
                abs_decimal_part.abs(),
                opts.precision.unwrap_or(state.precision)
            )
            .trim_start_matches("0.")
            .trim_end_matches("0"),
        )
    }
}

fn group_digits(s: String, digits_per_group: usize, pad_left_with_zeros: bool) -> String {
    let mut v = s.chars().collect::<Vec<char>>();
    v.reverse();
    let mut v = v
        .chunks(digits_per_group)
        .map(|c| {
            let mut s = c.iter().rev().collect::<String>();
            if pad_left_with_zeros {
                while s.len() < digits_per_group {
                    s = format!("0{s}");
                }
            }
            s
        })
        .collect::<Vec<String>>();
    v.reverse();
    v.join(" ")
}

impl Display for StackItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StackItem::Number(value) => write!(f, "{}", value),
            StackItem::Undefined => write!(f, "undefined"),
        }
    }
}

impl PartialEq for StackItem {
    fn eq(&self, other: &Self) -> bool {
        match self {
            StackItem::Number(value) => match other {
                StackItem::Number(other_value) => value == other_value,
                StackItem::Undefined => false,
            },
            StackItem::Undefined => match other {
                StackItem::Number(_) => false,
                StackItem::Undefined => true,
            },
        }
    }
}

pub struct StackItemToStringOpts {
    pub base: u8,
    pub precision: Option<usize>,
}

#[cfg(test)]
mod test {
    use num_format::SystemLocale;

    use crate::{
        init_logger,
        stack::item::{StackItem, StackItemToStringOpts},
        state::RpnState,
    };

    #[macro_export]
    macro_rules! assert_to_string_opts {
        ($expected: expr, $num: expr, $opts: expr, $state: expr) => {
            assert_eq!(
                $expected,
                StackItem::Number($num).to_string_opts(&$opts, &$state)
            );
        };
    }

    #[test]
    pub fn to_string_opts_base10() {
        init_logger(None).unwrap();

        let available_locale = SystemLocale::available_names().unwrap();
        let en_locale = available_locale
            .iter()
            .filter(|l| l.starts_with("en_US"))
            .next()
            .unwrap();

        let opts = StackItemToStringOpts {
            base: 10,
            precision: None,
        };
        let mut state = RpnState::new().unwrap();
        state.locale = SystemLocale::from_name(en_locale).unwrap();

        assert_to_string_opts!("0", 0.0, &opts, &state);

        assert_to_string_opts!("0.1", 0.1, &opts, &state);
        assert_to_string_opts!("0.0001", 0.0001, &opts, &state);
        assert_to_string_opts!("1", 1.0, &opts, &state);
        assert_to_string_opts!("1,000", 1000.0, &opts, &state);
        assert_to_string_opts!("1,000.1", 1000.1, &opts, &state);

        assert_to_string_opts!("-0.1", -0.1, &opts, &state);
        assert_to_string_opts!("-0.0001", -0.0001, &opts, &state);
        assert_to_string_opts!("-1", -1.0, &opts, &state);
        assert_to_string_opts!("-1,000", -1000.0, &opts, &state);
        assert_to_string_opts!("-1,000.1", -1000.1, &opts, &state);

        assert_to_string_opts!("1e103", 1000e100, &opts, &state);
        assert_to_string_opts!("1.123e100", 1.123e100, &opts, &state);
    }

    #[test]
    pub fn to_string_opts_base16() {
        init_logger(None).unwrap();

        let opts = StackItemToStringOpts {
            base: 16,
            precision: None,
        };
        let state = RpnState::new().unwrap();

        assert_to_string_opts!("0", 0.0, &opts, &state);

        assert_to_string_opts!("", 0.1, &opts, &state);
        assert_to_string_opts!("1", 1.0, &opts, &state);
        assert_to_string_opts!("3e8", 1000.0, &opts, &state);
        assert_to_string_opts!("", 1000.1, &opts, &state);

        assert_to_string_opts!("", -0.1, &opts, &state);
        assert_to_string_opts!("", -1.0, &opts, &state);

        assert_to_string_opts!("5f5 e100", 1e8, &opts, &state);

        assert_to_string_opts!("3e8", 1e3, &opts, &state);
        assert_to_string_opts!("", 1000e100, &opts, &state);
        assert_to_string_opts!("", 1.123e100, &opts, &state);
    }

    #[test]
    pub fn to_string_opts_base8() {
        init_logger(None).unwrap();

        let opts = StackItemToStringOpts {
            base: 8,
            precision: None,
        };
        let state = RpnState::new().unwrap();

        assert_to_string_opts!("0", 0.0, &opts, &state);

        assert_to_string_opts!("", 0.1, &opts, &state);
        assert_to_string_opts!("1", 1.0, &opts, &state);
        assert_to_string_opts!("1750", 1000.0, &opts, &state);
        assert_to_string_opts!("", 1000.1, &opts, &state);

        assert_to_string_opts!("", -0.1, &opts, &state);
        assert_to_string_opts!("", -1.0, &opts, &state);

        assert_to_string_opts!("5 7536 0400", 1e8, &opts, &state);

        assert_to_string_opts!("1750", 1e3, &opts, &state);
        assert_to_string_opts!("", 1000e100, &opts, &state);
        assert_to_string_opts!("", 1.123e100, &opts, &state);
    }

    #[test]
    pub fn to_string_opts_base2() {
        init_logger(None).unwrap();

        let opts = StackItemToStringOpts {
            base: 2,
            precision: None,
        };
        let state = RpnState::new().unwrap();

        assert_to_string_opts!("0000", 0.0, &opts, &state);

        assert_to_string_opts!("", 0.1, &opts, &state);
        assert_to_string_opts!("0001", 1.0, &opts, &state);
        assert_to_string_opts!("0011 1110 1000", 1000.0, &opts, &state);
        assert_to_string_opts!("", 1000.1, &opts, &state);

        assert_to_string_opts!("", -0.1, &opts, &state);
        assert_to_string_opts!("", -1.0, &opts, &state);

        assert_to_string_opts!("0101 1111 0101 1110 0001 0000 0000", 1e8, &opts, &state);

        assert_to_string_opts!("0011 1110 1000", 1e3, &opts, &state);
        assert_to_string_opts!("", 1000e100, &opts, &state);
        assert_to_string_opts!("", 1.123e100, &opts, &state);
    }
}
