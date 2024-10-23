use anyhow::{anyhow, Result};

#[derive(Clone, Debug)]
pub enum StackItem {
    Number { value: f64 },
}

impl StackItem {
    pub fn from_str(s: &str) -> Result<StackItem> {
        if let Ok(v) = s.parse::<f64>() {
            return Ok(StackItem::Number { value: v });
        }
        Err(anyhow!("parse error: {s}"))
    }

    pub fn add(&self, other: &StackItem) -> Result<StackItem> {
        match self {
            StackItem::Number { value } => match other {
                StackItem::Number { value: other_value } => Ok(StackItem::Number {
                    value: value + other_value,
                }),
            },
        }
    }
}

impl PartialEq for StackItem {
    fn eq(&self, other: &Self) -> bool {
        match self {
            StackItem::Number { value } => match other {
                StackItem::Number { value: other_value } => value == other_value,
            },
        }
    }
}
