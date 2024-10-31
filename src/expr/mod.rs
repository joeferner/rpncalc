use std::{
    fmt::{self, Formatter},
    ops::Range,
};

use annotate_snippets::{Level, Renderer, Snippet};

use crate::stack::item::StackItem;

pub mod lexer;
pub mod parser;
pub mod reader;
pub mod run;

#[derive(Debug)]
pub enum Expr {
    StackItem(StackItem),
    Identifier(String),
    FunctionCall(String, Vec<Expr>),
    UnaryOp {
        op: String,
        rhs: Box<Expr>,
    },
    BinaryOp {
        lhs: Box<Expr>,
        op: String,
        rhs: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub struct ExprError {
    source: String,
    location: Option<Range<usize>>,
    message: String,
}

impl ExprError {
    fn new(source: &str, location: Option<Range<usize>>, message: &str) -> Self {
        Self {
            source: source.to_owned(),
            location,
            message: message.to_owned(),
        }
    }

    pub fn get_snippet(&self) -> Snippet {
        let mut s = Snippet::source(&self.source).line_start(1);
        if let Some(location) = &self.location {
            s = s.annotation(Level::Error.span(location.clone()).label(&self.message));
        }
        s
    }
}

impl std::error::Error for ExprError {}

impl fmt::Display for ExprError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = Level::Error.title("").snippet(self.get_snippet());
        f.write_str(&format!("{}", Renderer::styled().render(msg)))
    }
}

pub type ExprResult<T> = Result<T, ExprError>;

#[cfg(test)]
mod test {
    use run::run_expression;

    use crate::{state::RpnState, test_expr};

    use super::*;

    #[test]
    pub fn test_parse_string() {
        test_expr!("'test\\'asd'", StackItem::String("test'asd".to_string()));
    }

    #[test]
    pub fn test_parse_decimal() {
        test_expr!("-42.5", StackItem::Number(-42.5, 10));
    }

    #[test]
    pub fn test_parse_hex() {
        test_expr!("-0x1f2e", StackItem::Number(-0x1f2e as f64, 16));
    }

    #[test]
    pub fn test_bad_ident() {
        let mut state = RpnState::new().unwrap();
        let e = run_expression("bad_ident", &mut state).expect_err("expected error");
        assert_eq!(
            "unknown constant, variable, or function: bad_ident",
            e.to_string()
        );
    }

    #[test]
    pub fn test_parse_simple_expr() {
        test_expr!("1+2", StackItem::Number(1.0 + 2.0, 10));
    }

    #[test]
    pub fn test_order_of_operation() {
        test_expr!("2 + 3 * 4", StackItem::Number(2.0 + 3.0 * 4.0, 10));
        test_expr!("3 * 4 + 2", StackItem::Number(3.0 * 4.0 + 2.0, 10));
        test_expr!("(2 + 3) * 4", StackItem::Number((2.0 + 3.0) * 4.0, 10))
    }

    #[test]
    pub fn test_unary_expr() {
        test_expr!("+30", StackItem::Number(30.0, 10));
        test_expr!("-30", StackItem::Number(-30.0, 10));
    }
}
