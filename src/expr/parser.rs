use crate::stack::item::StackItem;

use super::{
    lexer::{ExprLexer, ExprTokenType},
    Expr, ExprError, ExprResult,
};
use crate::parse_binary_expression;

pub(super) fn parse_expression_from_tokenizer(mut tokenizer: ExprLexer) -> ExprResult<Expr> {
    tokenizer.skip_start_of_input()?;
    let expr = parse_additive(&mut tokenizer)?;
    tokenizer.skip_end_of_input()?;
    Ok(expr)
}

fn parse_additive(tokenizer: &mut ExprLexer) -> ExprResult<Expr> {
    parse_binary_expression!(tokenizer, ["+", "-"], parse_multiplicative)
}

fn parse_multiplicative(tokenizer: &mut ExprLexer) -> ExprResult<Expr> {
    parse_binary_expression!(tokenizer, ["*", "/", "%"], parse_unary)
}

#[macro_export]
macro_rules! parse_binary_expression {
    ($tokenizer: expr, $operators: expr, $parse_child: ident) => {{
        let operators = $operators;
        let mut lhs = $parse_child($tokenizer)?;

        loop {
            if let Some(t) = $tokenizer.peek(0) {
                if let ExprTokenType::Operator(op) = &t.token_type {
                    if operators.contains(&op.as_str()) {
                        let op = $tokenizer.take_operator()?;
                        let rhs = $parse_child($tokenizer)?;
                        lhs = Expr::BinaryOp {
                            lhs: Box::new(lhs),
                            op,
                            rhs: Box::new(rhs),
                        };
                        continue;
                    }
                }
            }
            return Ok(lhs);
        }
    }};
}

fn parse_unary(tokenizer: &mut ExprLexer) -> ExprResult<Expr> {
    if let Some(t) = tokenizer.peek(0) {
        if let ExprTokenType::Operator(op) = &t.token_type {
            if op == "+" {
                tokenizer.take(); // skip
            } else if op == "-" {
                tokenizer.take(); // skip
                let rhs = parse_unary(tokenizer)?;
                return Ok(Expr::UnaryOp {
                    op: "neg".to_string(),
                    rhs: Box::new(rhs),
                });
            }
        }
    }

    parse_function_invocation(tokenizer)
}

fn parse_function_invocation(tokenizer: &mut ExprLexer) -> ExprResult<Expr> {
    if tokenizer.len() >= 2 && *tokenizer.peek_token_type(1).unwrap() == ExprTokenType::LeftParen {
        if let ExprTokenType::Identifier(fn_name) = tokenizer.peek_token_type(0).unwrap() {
            let fn_name = fn_name.to_owned();
            let mut args = vec![];

            tokenizer.take(); // skip identifier
            tokenizer.take(); // skip left paren
            loop {
                if let Some(t) = tokenizer.peek(0) {
                    match t.token_type {
                        ExprTokenType::RightParen => {
                            tokenizer.take();
                            break;
                        }
                        _ => {
                            args.push(parse_additive(tokenizer)?);
                        }
                    }
                } else {
                    return Err(ExprError::new(
                        tokenizer.get_source(),
                        None,
                        "expected argument list",
                    ));
                }
            }
            return Ok(Expr::FunctionCall(fn_name, args));
        }
    }

    parse_primary_paren(tokenizer)
}

fn parse_primary_paren(tokenizer: &mut ExprLexer) -> ExprResult<Expr> {
    if tokenizer.len() >= 1 && *tokenizer.peek_token_type(0).unwrap() == ExprTokenType::LeftParen {
        tokenizer.take_token(ExprTokenType::LeftParen)?;
        let result = parse_additive(tokenizer)?;
        tokenizer.take_token(ExprTokenType::RightParen)?;
        Ok(result)
    } else {
        parse_primary(tokenizer)
    }
}

fn parse_primary(tokenizer: &mut ExprLexer) -> ExprResult<Expr> {
    if let Some(t) = tokenizer.take() {
        match t.token_type {
            ExprTokenType::DecimalNumber(v) => Ok(Expr::StackItem(StackItem::Number(v, 10))),
            ExprTokenType::HexNumber(v) => Ok(Expr::StackItem(StackItem::Number(v as f64, 16))),
            ExprTokenType::Identifier(s) => Ok(Expr::Identifier(s)),
            ExprTokenType::String(s) => Ok(Expr::StackItem(StackItem::String(s))),
            _ => Err(ExprError::new(
                tokenizer.get_source(),
                Some(t.location),
                "unexpected token",
            )),
        }
    } else {
        Err(ExprError::new(
            tokenizer.get_source(),
            None,
            "unexpected end of input",
        ))
    }
}
