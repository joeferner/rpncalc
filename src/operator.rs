use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use thiserror::Error;

#[derive(Copy, Clone)]
pub enum Operator {
    Degrees,
    Radians,
    Add,
    Subtract,
    Multiply,
    Divide,
    Pow,
    SquareRoot,
    Sine,
    Cosine,
    Tangent,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operator::Degrees => write!(f, "deg"),
            Operator::Radians => write!(f, "rad"),
            Operator::Add => write!(f, "add"),
            Operator::Subtract => write!(f, "sub"),
            Operator::Multiply => write!(f, "mul"),
            Operator::Divide => write!(f, "div"),
            Operator::Pow => write!(f, "pow"),
            Operator::SquareRoot => write!(f, "sqrt"),
            Operator::Sine => write!(f, "sin"),
            Operator::Cosine => write!(f, "cos"),
            Operator::Tangent => write!(f, "tan"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Error)]
pub struct ParseOperatorError {
    str: String,
}

impl Display for ParseOperatorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse string \"{}\"", self.str)
    }
}

impl FromStr for Operator {
    type Err = ParseOperatorError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        if str == "deg" || str == "degrees" {
            return Ok(Operator::Degrees);
        }
        if str == "rad" || str == "radians" {
            return Ok(Operator::Radians);
        }
        if str == "+" || str == "add" {
            return Ok(Operator::Add);
        }
        if str == "-" || str == "sub" || str == "subtract" {
            return Ok(Operator::Subtract);
        }
        if str == "*" || str == "mul" || str == "multiply" {
            return Ok(Operator::Multiply);
        }
        if str == "/" || str == "div" || str == "divide" {
            return Ok(Operator::Divide);
        }
        if str == "^" || str == "pow" {
            return Ok(Operator::Pow);
        }
        if str == "sqrt" {
            return Ok(Operator::SquareRoot);
        }
        if str == "sin" || str == "sine" {
            return Ok(Operator::Sine);
        }
        if str == "cos" || str == "cosine" {
            return Ok(Operator::Cosine);
        }
        if str == "tan" || str == "tangent" {
            return Ok(Operator::Tangent);
        }
        return Err(ParseOperatorError {
            str: str.to_string(),
        });
    }
}
