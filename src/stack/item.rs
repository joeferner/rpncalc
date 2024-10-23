use std::fmt::Display;
use std::fmt::{self};

use anyhow::{anyhow, Result};

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
