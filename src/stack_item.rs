use crate::operator::Operator;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use thiserror::Error;

pub type NumberType = f64;

#[derive(Clone)]
pub enum StackItem {
    Number(NumberType),
    String(String),
    Operator(Operator),
}

impl Display for StackItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StackItem::Number(n) => write!(f, "{}", n),
            StackItem::String(s) => write!(f, "`{}`", s),
            StackItem::Operator(o) => write!(f, "{}", o),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Error)]
pub struct ParseStackItemError {
    str: String,
}

impl Display for ParseStackItemError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse string \"{}\"", self.str)
    }
}

impl FromStr for StackItem {
    type Err = ParseStackItemError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        if let Ok(n) = str.parse::<f64>() {
            return Ok(StackItem::Number(n));
        }
        if let Ok(o) = str.parse::<Operator>() {
            return Ok(StackItem::Operator(o));
        }
        if let Ok(s) = parse_string(str) {
            return Ok(StackItem::String(s));
        }
        return Err(ParseStackItemError {
            str: str.to_string(),
        });
    }
}

fn parse_string(str: &str) -> Result<String, ParseStackItemError> {
    let str = str.to_string();
    if str.starts_with("`") && str.ends_with("`") {
        return Ok(str[1..str.len() - 1].to_string());
    }
    return Err(ParseStackItemError { str });
}
