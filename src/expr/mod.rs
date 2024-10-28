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

    fn get_snippet(&self) -> Snippet {
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

    use crate::{stack::item::StackItem, state::RpnState};

    use super::*;

    #[test]
    pub fn test_parse_string() {
        let mut state = RpnState::new().unwrap();
        run_expression("'test\\'asd'", &mut state).unwrap();
        assert_eq!(1, state.stack.len());
        assert_eq!(
            StackItem::String("test'asd".to_string()),
            state.stack.peek(0).unwrap().clone()
        );
    }

    #[test]
    pub fn test_parse_decimal() {
        let mut state = RpnState::new().unwrap();
        run_expression("-42.5", &mut state).unwrap();
        assert_eq!(1, state.stack.len());
        assert_eq!(
            StackItem::Number(-42.5, 10),
            state.stack.peek(0).unwrap().clone()
        );
    }

    #[test]
    pub fn test_parse_hex() {
        let mut state = RpnState::new().unwrap();
        run_expression("-0x1f2e", &mut state).unwrap();
        assert_eq!(1, state.stack.len());
        assert_eq!(
            StackItem::Number(-0x1f2e as f64, 16),
            state.stack.peek(0).unwrap().clone()
        );
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
    pub fn test_order_of_operation() {
        let mut state = RpnState::new().unwrap();
        run_expression("2 + 3 * 4", &mut state).unwrap();
        assert_eq!(1, state.stack.len());
        assert_eq!(
            StackItem::Number(2.0 + 3.0 * 4.0, 10),
            state.stack.peek(0).unwrap().clone()
        );

        let mut state = RpnState::new().unwrap();
        run_expression("3 * 4 + 2", &mut state).unwrap();
        assert_eq!(1, state.stack.len());
        assert_eq!(
            StackItem::Number(3.0 * 4.0 + 2.0, 10),
            state.stack.peek(0).unwrap().clone()
        );

        let mut state = RpnState::new().unwrap();
        run_expression("(2 + 3) * 4", &mut state).unwrap();
        assert_eq!(1, state.stack.len());
        assert_eq!(
            StackItem::Number((2.0 + 3.0) * 4.0, 10),
            state.stack.peek(0).unwrap().clone()
        );
    }
}
