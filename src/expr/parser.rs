use anyhow::{anyhow, Context, Result};
use pest::{
    iterators::{Pair, Pairs},
    pratt_parser::PrattParser,
    Parser,
};
use pest_derive::Parser;

use crate::stack::item::StackItem;

use super::Expr;

#[derive(Parser)]
#[grammar = "expr/expr.pest"]
struct ExpressionParser;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            // Addition and subtract have equal precedence
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left) | Op::infix(modulus, Left))
    };
}

pub fn parse_expression(s: &str) -> Result<Expr> {
    if s == "+" || s == "-" || s == "*" || s == "/" || s == "%" {
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
                Rule::modulus => "%".to_string(),
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
