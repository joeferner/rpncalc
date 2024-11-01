use core::f64;
use std::fmt::Display;
use std::fmt::{self};

use anyhow::{anyhow, Result};
use log::warn;
use num_format::ToFormattedString;

use crate::state::{angle_mode::AngleMode, RpnState};

#[derive(Clone, Debug)]
pub enum StackItem {
    // value, display base
    Number(f64, u8),
    String(String),
    Undefined,
}

impl StackItem {
    pub fn add(&self, other: &StackItem) -> Result<StackItem> {
        match self {
            StackItem::Number(value, _) => match other {
                StackItem::Number(other_value, display_base) => {
                    Ok(StackItem::Number(value + other_value, *display_base))
                }
                StackItem::Undefined => Ok(StackItem::Undefined),
                StackItem::String(_) => Ok(StackItem::Undefined),
            },
            StackItem::Undefined => Ok(StackItem::Undefined),
            StackItem::String(s) => match other {
                StackItem::Number(_, _) => Ok(StackItem::Undefined),
                StackItem::Undefined => Ok(StackItem::Undefined),
                StackItem::String(other_s) => Ok(StackItem::String(format!("{s}{other_s}"))),
            },
        }
    }

    pub fn subtract(&self, other: &StackItem) -> Result<StackItem> {
        match self {
            StackItem::Number(value, _) => match other {
                StackItem::Number(other_value, display_base) => {
                    Ok(StackItem::Number(value - other_value, *display_base))
                }
                StackItem::Undefined => Ok(StackItem::Undefined),
                StackItem::String(_) => Ok(StackItem::Undefined),
            },
            StackItem::Undefined => Ok(StackItem::Undefined),
            StackItem::String(_) => Ok(StackItem::Undefined),
        }
    }

    pub fn multiply(&self, other: &StackItem) -> Result<StackItem> {
        match self {
            StackItem::Number(value, _) => match other {
                StackItem::Number(other_value, display_base) => {
                    Ok(StackItem::Number(value * other_value, *display_base))
                }
                StackItem::Undefined => Ok(StackItem::Undefined),
                StackItem::String(_) => Ok(StackItem::Undefined),
            },
            StackItem::Undefined => Ok(StackItem::Undefined),
            StackItem::String(_) => Ok(StackItem::Undefined),
        }
    }

    pub fn divide(&self, other: &StackItem) -> Result<StackItem> {
        match self {
            StackItem::Number(value, _) => match other {
                StackItem::Number(other_value, display_base) => {
                    if *other_value == 0.0 {
                        Ok(StackItem::Undefined)
                    } else {
                        Ok(StackItem::Number(value / other_value, *display_base))
                    }
                }
                StackItem::Undefined => Ok(StackItem::Undefined),
                StackItem::String(_) => Ok(StackItem::Undefined),
            },
            StackItem::Undefined => Ok(StackItem::Undefined),
            StackItem::String(_) => Ok(StackItem::Undefined),
        }
    }

    pub fn modulus(&self, other: &StackItem) -> Result<StackItem> {
        match self {
            StackItem::Number(value, _) => match other {
                StackItem::Number(other_value, display_base) => {
                    if *other_value == 0.0 {
                        Ok(StackItem::Undefined)
                    } else {
                        Ok(StackItem::Number(value % other_value, *display_base))
                    }
                }
                StackItem::Undefined => Ok(StackItem::Undefined),
                StackItem::String(_) => Ok(StackItem::Undefined),
            },
            StackItem::Undefined => Ok(StackItem::Undefined),
            StackItem::String(_) => Ok(StackItem::Undefined),
        }
    }

    pub fn pow(&self, other: &StackItem) -> Result<StackItem> {
        match self {
            StackItem::Number(value, display_base) => match other {
                StackItem::Number(other_value, _) => {
                    Ok(StackItem::Number(value.powf(*other_value), *display_base))
                }
                StackItem::Undefined => Ok(StackItem::Undefined),
                StackItem::String(_) => Ok(StackItem::Undefined),
            },
            StackItem::Undefined => Ok(StackItem::Undefined),
            StackItem::String(_) => Ok(StackItem::Undefined),
        }
    }

    pub fn negate(&self) -> Result<StackItem> {
        match self {
            StackItem::Number(v, display_base) => Ok(StackItem::Number(-v, *display_base)),
            StackItem::Undefined => Ok(StackItem::Undefined),
            StackItem::String(_) => Ok(StackItem::Undefined),
        }
    }

    pub fn sqrt(&self) -> Result<StackItem> {
        match self {
            StackItem::Number(v, display_base) => {
                if *v < 0.0 {
                    Err(anyhow!("cannot take the square root of a negative number"))
                } else {
                    Ok(StackItem::Number(v.sqrt(), *display_base))
                }
            }
            StackItem::Undefined => Ok(StackItem::Undefined),
            StackItem::String(_) => Ok(StackItem::Undefined),
        }
    }

    pub fn asin(&self, angle_mode: AngleMode) -> Result<StackItem> {
        match self {
            StackItem::Number(v, display_base) => Ok(StackItem::Number(
                radians_to_angle_mode(v.asin(), angle_mode),
                *display_base,
            )),
            StackItem::Undefined => Ok(StackItem::Undefined),
            StackItem::String(_) => Ok(StackItem::Undefined),
        }
    }

    pub fn sin(&self, angle_mode: AngleMode) -> Result<StackItem> {
        let r = self.to_radians(angle_mode);
        match r {
            StackItem::Number(v, display_base) => Ok(StackItem::Number(v.sin(), display_base)),
            StackItem::Undefined => Ok(StackItem::Undefined),
            StackItem::String(_) => Ok(StackItem::Undefined),
        }
    }

    pub fn acos(&self, angle_mode: AngleMode) -> Result<StackItem> {
        match self {
            StackItem::Number(v, display_base) => Ok(StackItem::Number(
                radians_to_angle_mode(v.acos(), angle_mode),
                *display_base,
            )),
            StackItem::Undefined => Ok(StackItem::Undefined),
            StackItem::String(_) => Ok(StackItem::Undefined),
        }
    }

    pub fn cos(&self, angle_mode: AngleMode) -> Result<StackItem> {
        let r = self.to_radians(angle_mode);
        match r {
            StackItem::Number(v, display_base) => Ok(StackItem::Number(v.cos(), display_base)),
            StackItem::Undefined => Ok(StackItem::Undefined),
            StackItem::String(_) => Ok(StackItem::Undefined),
        }
    }

    pub fn atan(&self, angle_mode: AngleMode) -> Result<StackItem> {
        match self {
            StackItem::Number(v, display_base) => Ok(StackItem::Number(
                radians_to_angle_mode(v.atan(), angle_mode),
                *display_base,
            )),
            StackItem::Undefined => Ok(StackItem::Undefined),
            StackItem::String(_) => Ok(StackItem::Undefined),
        }
    }

    pub fn atan2(&self, other: &StackItem, angle_mode: AngleMode) -> Result<StackItem> {
        match self {
            StackItem::Number(v, display_base) => match other {
                StackItem::Number(other_v, _) => Ok(StackItem::Number(
                    radians_to_angle_mode(v.atan2(*other_v), angle_mode),
                    *display_base,
                )),
                StackItem::String(_) => Ok(StackItem::Undefined),
                StackItem::Undefined => Ok(StackItem::Undefined),
            },
            StackItem::Undefined => Ok(StackItem::Undefined),
            StackItem::String(_) => Ok(StackItem::Undefined),
        }
    }

    pub fn tan(&self, angle_mode: AngleMode) -> Result<StackItem> {
        let r = self.to_radians(angle_mode);
        match r {
            StackItem::Number(v, display_base) => Ok(StackItem::Number(v.tan(), display_base)),
            StackItem::Undefined => Ok(StackItem::Undefined),
            StackItem::String(_) => Ok(StackItem::Undefined),
        }
    }

    pub fn to_radians(&self, from_angle_mode: AngleMode) -> StackItem {
        match self {
            StackItem::Number(v, display_base) => match from_angle_mode {
                AngleMode::Degrees => StackItem::Number(degrees_to_radians(*v), *display_base),
                AngleMode::Radians => StackItem::Number(*v, *display_base),
            },
            StackItem::Undefined => StackItem::Undefined,
            StackItem::String(_) => StackItem::Undefined,
        }
    }

    pub fn is_integer(&self) -> bool {
        match self {
            StackItem::Number(v, _) => is_integer(*v),
            StackItem::Undefined => false,
            StackItem::String(_) => false,
        }
    }

    pub fn to_string_opts(&self, opts: &StackItemToStringOpts, state: &RpnState) -> String {
        match self {
            StackItem::Number(n, display_base) => {
                let base = opts.base.unwrap_or(*display_base);

                if is_integer(*n) {
                    if base == 2 {
                        return to_string_binary(*n, opts);
                    } else if base == 8 {
                        return to_string_octal(*n, opts);
                    } else if base == 16 {
                        return to_string_hex(*n, opts);
                    }
                }
                to_string_opts_base10(*n, opts, state)
            }
            StackItem::Undefined => "Undefined".to_string(),
            StackItem::String(s) => format!("'{s}'"),
        }
    }
}

impl Display for StackItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StackItem::Number(value, display_base) => {
                let opts = StackItemToStringOpts {
                    base: None,
                    precision: None,
                    left_pad_with_zeros: true,
                    include_base_prefix: true,
                };
                if *display_base == 2 {
                    write!(f, "{}", to_string_binary(*value, &opts))
                } else if *display_base == 8 {
                    write!(f, "{}", to_string_octal(*value, &opts))
                } else if *display_base == 16 {
                    write!(f, "{}", to_string_hex(*value, &opts))
                } else {
                    write!(f, "{}", value)
                }
            }
            StackItem::Undefined => write!(f, "Undefined"),
            StackItem::String(s) => write!(f, "'{s}'"),
        }
    }
}

impl PartialEq for StackItem {
    fn eq(&self, other: &Self) -> bool {
        match self {
            StackItem::Number(value, display_base) => match other {
                StackItem::Number(other_value, other_display_base) => {
                    value == other_value && display_base == other_display_base
                }
                StackItem::Undefined => false,
                StackItem::String(_) => false,
            },
            StackItem::Undefined => match other {
                StackItem::Number(_, _) => false,
                StackItem::Undefined => true,
                StackItem::String(_) => false,
            },
            StackItem::String(s) => match other {
                StackItem::Number(_, _) => false,
                StackItem::Undefined => false,
                StackItem::String(other_s) => s == other_s,
            },
        }
    }
}

pub struct StackItemToStringOpts {
    pub base: Option<u8>,
    pub precision: Option<usize>,
    pub left_pad_with_zeros: bool,
    pub include_base_prefix: bool,
}

fn to_string_binary(n: f64, opts: &StackItemToStringOpts) -> String {
    if !is_integer(n) {
        return format!("{}", n);
    }
    let whole_part = n as i128;
    let abs_whole_part = whole_part.abs();
    let sign = if n < 0.0 { "-" } else { "" };
    let base_prefix = if opts.include_base_prefix { "0b" } else { "" };
    format!(
        "{sign}{base_prefix}{}",
        group_digits(format!("{:b}", abs_whole_part), 4, opts.left_pad_with_zeros)
    )
}

fn to_string_octal(n: f64, opts: &StackItemToStringOpts) -> String {
    if !is_integer(n) {
        return format!("{}", n);
    }
    let whole_part = n as i128;
    let abs_whole_part = whole_part.abs();
    let sign = if n < 0.0 { "-" } else { "" };
    let base_prefix = if opts.include_base_prefix { "0o" } else { "" };
    format!(
        "{sign}{base_prefix}{}",
        group_digits(format!("{:o}", abs_whole_part), 4, opts.left_pad_with_zeros)
    )
}

fn to_string_hex(n: f64, opts: &StackItemToStringOpts) -> String {
    if !is_integer(n) {
        return format!("{}", n);
    }
    let whole_part = n as i128;
    let abs_whole_part = whole_part.abs();
    let sign = if n < 0.0 { "-" } else { "" };
    let base_prefix = if opts.include_base_prefix { "0x" } else { "" };
    format!(
        "{sign}{base_prefix}{}",
        group_digits(format!("{:x}", abs_whole_part), 4, opts.left_pad_with_zeros)
    )
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

fn is_integer(n: f64) -> bool {
    let whole_part = n as i128;
    ((whole_part as f64) - n).abs() < f64::EPSILON * 1000.0
}

fn degrees_to_radians(v: f64) -> f64 {
    v * f64::consts::PI / 180.0
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
                StackItem::Number($num, 10).to_string_opts(&$opts, &$state)
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
            base: Some(10),
            precision: None,
            left_pad_with_zeros: false,
            include_base_prefix: false,
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
            base: Some(16),
            precision: None,
            left_pad_with_zeros: false,
            include_base_prefix: true,
        };
        let state = RpnState::new().unwrap();

        assert_to_string_opts!("0x0", 0.0, &opts, &state);

        assert_to_string_opts!("0.1", 0.1, &opts, &state);
        assert_to_string_opts!("0x1", 1.0, &opts, &state);
        assert_to_string_opts!("0x3e8", 1000.0, &opts, &state);
        assert_to_string_opts!("1,000.1", 1000.1, &opts, &state);

        assert_to_string_opts!("-0.1", -0.1, &opts, &state);
        assert_to_string_opts!("-0x1", -1.0, &opts, &state);

        assert_to_string_opts!("0x5f5 e100", 1e8, &opts, &state);

        assert_to_string_opts!("0x3e8", 1e3, &opts, &state);
        assert_to_string_opts!("1e103", 1000e100, &opts, &state);
        assert_to_string_opts!("1.123e100", 1.123e100, &opts, &state);
    }

    #[test]
    pub fn to_string_opts_base8() {
        init_logger(None).unwrap();

        let opts = StackItemToStringOpts {
            base: Some(8),
            precision: None,
            left_pad_with_zeros: false,
            include_base_prefix: true,
        };
        let state = RpnState::new().unwrap();

        assert_to_string_opts!("0o0", 0.0, &opts, &state);

        assert_to_string_opts!("0.1", 0.1, &opts, &state);
        assert_to_string_opts!("0o1", 1.0, &opts, &state);
        assert_to_string_opts!("0o1750", 1000.0, &opts, &state);
        assert_to_string_opts!("1,000.1", 1000.1, &opts, &state);

        assert_to_string_opts!("-0.1", -0.1, &opts, &state);
        assert_to_string_opts!("-0o1", -1.0, &opts, &state);

        assert_to_string_opts!("0o5 7536 0400", 1e8, &opts, &state);

        assert_to_string_opts!("0o1750", 1e3, &opts, &state);
        assert_to_string_opts!("1e103", 1000e100, &opts, &state);
        assert_to_string_opts!("1.123e100", 1.123e100, &opts, &state);
    }

    #[test]
    pub fn to_string_opts_base2() {
        init_logger(None).unwrap();

        let opts = StackItemToStringOpts {
            base: Some(2),
            precision: None,
            left_pad_with_zeros: true,
            include_base_prefix: true,
        };
        let state = RpnState::new().unwrap();

        assert_to_string_opts!("0b0000", 0.0, &opts, &state);

        assert_to_string_opts!("0.1", 0.1, &opts, &state);
        assert_to_string_opts!("0b0001", 1.0, &opts, &state);
        assert_to_string_opts!("0b0011 1110 1000", 1000.0, &opts, &state);
        assert_to_string_opts!("1,000.1", 1000.1, &opts, &state);

        assert_to_string_opts!("-0.1", -0.1, &opts, &state);
        assert_to_string_opts!("-0b0001", -1.0, &opts, &state);

        assert_to_string_opts!("0b0101 1111 0101 1110 0001 0000 0000", 1e8, &opts, &state);

        assert_to_string_opts!("0b0011 1110 1000", 1e3, &opts, &state);
        assert_to_string_opts!("1e103", 1000e100, &opts, &state);
        assert_to_string_opts!("1.123e100", 1.123e100, &opts, &state);
    }

    #[test]
    pub fn to_add_number_to_number() {
        let v = StackItem::Number(42.0, 10)
            .add(&StackItem::Number(13.0, 16))
            .unwrap();
        assert_eq!(StackItem::Number(42.0 + 13.0, 16), v);
    }

    #[test]
    pub fn to_add_string_to_string() {
        let v = StackItem::String("te".to_string())
            .add(&&StackItem::String("st".to_string()))
            .unwrap();
        assert_eq!(StackItem::String("test".to_string()), v);
    }
}

pub fn radians_to_angle_mode(v: f64, angle_mode: AngleMode) -> f64 {
    match angle_mode {
        AngleMode::Degrees => v * 180.0 / f64::consts::PI,
        AngleMode::Radians => v,
    }
}
