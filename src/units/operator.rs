use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub enum UnitsOperator {
    Divide,
    Multiply,
}

impl Display for UnitsOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UnitsOperator::Multiply => write!(f, "*"),
            UnitsOperator::Divide => write!(f, "/"),
        }
    }
}
