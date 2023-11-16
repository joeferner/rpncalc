use std::fmt;
use std::fmt::Display;
use std::rc::Rc;
use crate::function::Function;
use crate::number::Number;

#[derive(Clone)]
pub enum StackItem {
    Number(Number),
    String(String),
    Function(Rc<dyn Function>),
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
