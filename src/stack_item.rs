use crate::function::Function;
use crate::number::Number;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;

#[derive(Clone)]
pub enum StackItem {
    Number(Number),
    String(String),
    Function(Rc<dyn Function>),
}

impl StackItem {
    pub fn to_string_format(&self, width: usize, base: u16) -> String {
        return match self {
            StackItem::Number(n) => n.to_string_format(width, base),
            _ => format!("{}", self)
        };
    }
}

impl PartialEq for StackItem {
    fn eq(&self, other: &Self) -> bool {
        return match self {
            StackItem::Number(n) => {
                if let StackItem::Number(other) = other {
                    n == other
                } else {
                    false
                }
            }
            StackItem::String(s) => {
                if let StackItem::String(other) = other {
                    s == other
                } else {
                    false
                }
            }
            StackItem::Function(func) => {
                if let StackItem::Function(other) = other {
                    Rc::ptr_eq(func, other)
                } else {
                    false
                }
            }
        };
    }
}

impl Debug for StackItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            StackItem::Number(n) => write!(f, "{:?}", n),
            StackItem::String(s) => write!(f, "`{:?}`", s),
            StackItem::Function(func) => write!(f, "{}", func),
        }
    }
}

impl Display for StackItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StackItem::Number(n) => write!(f, "{}", n),
            StackItem::String(s) => write!(f, "`{}`", s),
            StackItem::Function(func) => write!(f, "{}", func),
        }
    }
}
