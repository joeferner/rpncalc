use crate::stack::item::StackItem;

use super::{
    lexer::{ExprLexer, ExprToken, ExprTokenType},
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
                if t.token_type == ExprTokenType::Operator {
                    if operators.contains(&t.text.as_str()) {
                        let op = t.text.clone();
                        $tokenizer.take_token(ExprTokenType::Operator)?;
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
        if t.token_type == ExprTokenType::Operator {
            if t.text == "+" {
                tokenizer.take(); // skip
            } else if t.text == "-" {
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
    if tokenizer.len() >= 2
        && *tokenizer.peek_token_type(0).unwrap() == ExprTokenType::Identifier
        && *tokenizer.peek_token_type(1).unwrap() == ExprTokenType::LeftParen
    {
        let mut args = vec![];

        let fn_name = tokenizer
            .take_token(ExprTokenType::Identifier)
            .unwrap()
            .text;
        tokenizer.take_token(ExprTokenType::LeftParen)?;
        let mut first = true;
        while !tokenizer.is_next_token(ExprTokenType::RightParen) {
            if !first {
                tokenizer.take_token(ExprTokenType::Comma)?;
            }
            args.push(parse_additive(tokenizer)?);
            first = false;
        }
        tokenizer.take_token(ExprTokenType::RightParen)?;
        return Ok(Expr::FunctionCall(fn_name, args));
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
            ExprTokenType::DecimalNumber => parse_decimal_number(tokenizer, t),
            ExprTokenType::HexNumber => parse_hex_number(tokenizer, t),
            ExprTokenType::Identifier => Ok(Expr::Identifier(t.text)),
            ExprTokenType::String => parse_string(tokenizer, t),
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

fn parse_hex_number(tokenizer: &mut ExprLexer, t: ExprToken) -> ExprResult<Expr> {
    let s = t.text;
    let (s, neg) = if let Some(s) = s.strip_prefix("-") {
        (s, -1)
    } else if let Some(s) = s.strip_prefix("+") {
        (s, 1)
    } else {
        (s.as_str(), 1)
    };

    let s = s.trim_start_matches("0x");

    match i128::from_str_radix(s, 16) {
        Ok(v) => Ok(Expr::StackItem(StackItem::Number((neg * v) as f64, 16))),
        Err(e) => Err(ExprError::new(
            tokenizer.get_source(),
            Some(t.location.clone()),
            &format!("parse hexadecimal; error = {e}"),
        )),
    }
}

fn parse_string(tokenizer: &mut ExprLexer, t: ExprToken) -> ExprResult<Expr> {
    let mut s = t.text.as_str();
    if !s.starts_with("'") {
        return Err(ExprError::new(
            tokenizer.get_source(),
            Some(t.location.clone()),
            "invalid start of string",
        ));
    }
    s = &s[1..];

    let mut escape = false;
    let mut ch_count = 0;
    let mut value = "".to_string();
    for ch in s.chars() {
        ch_count += 1;
        if escape {
            escape = false;
            if ch == '\'' {
                value.push(ch);
            } else {
                return Err(ExprError::new(
                    tokenizer.get_source(),
                    Some(t.location.start + ch_count..t.location.start + ch_count),
                    &format!("invalid escape sequence \\{ch}"),
                ));
            }
        } else if ch == '\\' {
            escape = true;
        } else if ch == '\'' {
            break;
        } else {
            value.push(ch);
        }
    }

    Ok(Expr::StackItem(StackItem::String(value)))
}

fn parse_decimal_number(tokenizer: &ExprLexer, t: ExprToken) -> ExprResult<Expr> {
    let v = t.text.parse::<f64>().map_err(|e| {
        ExprError::new(
            tokenizer.get_source(),
            Some(t.location.clone()),
            &format!("parse decimal; error = {e}"),
        )
    })?;
    Ok(Expr::StackItem(StackItem::Number(v, 10)))
}
