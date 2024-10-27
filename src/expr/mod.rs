use crate::stack::item::StackItem;

pub mod parser;
pub mod run;

#[derive(Debug)]
pub enum Expr {
    StackItem(StackItem),
    Ident(String),
    BinaryOp {
        lhs: Box<Expr>,
        op: String,
        rhs: Box<Expr>,
    },
}

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
        run_expression("(2 + 3) * 4", &mut state).unwrap();
        assert_eq!(1, state.stack.len());
        assert_eq!(
            StackItem::Number((2.0 + 3.0) * 4.0, 10),
            state.stack.peek(0).unwrap().clone()
        );
    }
}
