use anyhow::{anyhow, Context, Result};
use pest::{
    iterators::{Pair, Pairs},
    pratt_parser::PrattParser,
    Parser,
};
use pest_derive::Parser;

use crate::{
    stack::item::StackItem,
    state::RpnState,
    undo_action::{multi::MultiUndoEvent, push::PushUndoEvent, UndoEvent},
};

#[derive(Parser)]
#[grammar = "parser/expr.pest"]
struct ExpressionParser;

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

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            // Addition and subtract have equal precedence
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left))
    };
}

fn parse_expression(s: &str) -> Result<Expr> {
    if s == "+" || s == "-" || s == "*" || s == "/" {
        return Ok(Expr::Ident(s.to_string()));
    }

    let mut pairs =
        ExpressionParser::parse(Rule::expr, s).with_context(|| format!("failed to parse: {s}"))?;

    // discard SOI and EOI
    let pairs = pairs.next().unwrap().into_inner();

    do_parse(pairs)
}

fn do_parse(pairs: Pairs<Rule>) -> Result<Expr> {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::primary => parse_primary(primary.into_inner().next().unwrap()),
            Rule::primary_paren => do_parse(primary.into_inner()),
            Rule::binary_expr => do_parse(primary.into_inner()),
            rule => unreachable!("Expr::parse expected primary, found {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::add => "+".to_string(),
                Rule::subtract => "-".to_string(),
                Rule::multiply => "*".to_string(),
                Rule::divide => "/".to_string(),
                rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
            };
            Ok(Expr::BinaryOp {
                lhs: Box::new(lhs?),
                op,
                rhs: Box::new(rhs?),
            })
        })
        .parse(pairs)
}

pub fn run_expression(s: &str, state: &mut RpnState) -> Result<()> {
    let expr = parse_expression(s)?;

    let mut undos: Vec<Box<dyn UndoEvent>> = vec![];
    match run_expr(&expr, state, &mut undos) {
        Ok(_) => {
            if undos.is_empty() {
                // do nothing
                Ok(())
            } else if undos.len() == 1 {
                let undo = undos.remove(0);
                state.undo_stack.push_undo_stack(undo);
                Ok(())
            } else {
                state
                    .undo_stack
                    .push_undo_stack(Box::new(MultiUndoEvent::new(undos)));
                Ok(())
            }
        }
        Err(e) => Err(e),
    }
}

fn run_expr(expr: &Expr, state: &mut RpnState, undos: &mut Vec<Box<dyn UndoEvent>>) -> Result<()> {
    match expr {
        Expr::StackItem(stack_item) => {
            state.stack.push(stack_item.clone());
            undos.push(Box::new(PushUndoEvent::new(stack_item.clone())));
            Ok(())
        }
        Expr::Ident(ident) => run_ident(ident, state, undos),
        Expr::BinaryOp { lhs, op, rhs } => run_binary_op(lhs, op, rhs, state, undos),
    }
}

fn run_ident(ident: &str, state: &mut RpnState, undos: &mut Vec<Box<dyn UndoEvent>>) -> Result<()> {
    if let Some(c) = state.constants.get(ident) {
        let stack_item = StackItem::Number(c.value, 10);
        state.stack.push(stack_item.clone());
        undos.push(Box::new(PushUndoEvent::new(stack_item)));
        Ok(())
    } else if let Some(stack_item) = state.variables.get(ident) {
        state.stack.push(stack_item.clone());
        undos.push(Box::new(PushUndoEvent::new(stack_item.clone())));
        Ok(())
    } else if let Some(f) = state.functions.get(ident) {
        let undo = f.clone().execute(state)?;
        undos.push(undo);
        Ok(())
    } else {
        Err(anyhow!("unknown constant, variable, or function: {ident}"))
    }
}

fn run_binary_op(
    lhs: &Expr,
    op: &str,
    rhs: &Expr,
    state: &mut RpnState,
    undos: &mut Vec<Box<dyn UndoEvent>>,
) -> Result<()> {
    run_expr(lhs, state, undos)?;
    run_expr(rhs, state, undos)?;
    run_ident(op, state, undos)
}

fn parse_primary(pair: Pair<Rule>) -> Result<Expr> {
    match pair.as_rule() {
        Rule::decimal => parse_decimal(pair),
        Rule::hex => parse_hex(pair),
        Rule::ident => parse_ident(pair),
        Rule::string => parse_string(pair),
        _ => Err(anyhow!(
            "only primary expressions expected, but found {pair:?}"
        )),
    }
}

fn parse_decimal(pair: Pair<Rule>) -> Result<Expr> {
    let s = pair.as_str();
    let v = s
        .parse::<f64>()
        .with_context(|| format!("failed to parse: {s}"))?;
    Ok(Expr::StackItem(StackItem::Number(v, 10)))
}

fn parse_hex(pair: Pair<Rule>) -> Result<Expr> {
    let s = pair.as_str();
    let (s, neg) = if let Some(s) = s.strip_prefix("-") {
        (s, -1.0)
    } else if let Some(s) = s.strip_prefix("+") {
        (s, 1.0)
    } else {
        (s, 1.0)
    };

    let s = s.trim_start_matches("0x");

    let stack_item = match i128::from_str_radix(s, 16) {
        Ok(v) => StackItem::Number(neg * (v as f64), 16),
        Err(e) => return Err(anyhow!("parse error: {e}")),
    };
    Ok(Expr::StackItem(stack_item))
}

fn parse_ident(pair: Pair<Rule>) -> Result<Expr> {
    let s = pair.as_str();
    Ok(Expr::Ident(s.to_string()))
}

fn parse_string(pair: Pair<Rule>) -> Result<Expr> {
    let s = pair.as_str();
    let s = s[1..s.len() - 1].replace("\\'", "'");
    Ok(Expr::StackItem(StackItem::String(s.to_string())))
}

#[cfg(test)]
mod test {
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
