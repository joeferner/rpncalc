use std::ops::Range;

use regex::Regex;

use super::{
    reader::{InputReader, ReResult, ReaderResult},
    ExprError, ExprResult,
};

#[derive(Debug, PartialEq)]
pub enum ExprTokenType {
    StartOfInput,
    EndOfInput,
    DecimalNumber(f64),
    HexNumber(i128),
    Operator(String),
    Identifier(String),
    String(String),
    LeftParen,
    RightParen,
    Comma,
}

#[derive(Debug)]
pub struct ExprToken {
    pub token_type: ExprTokenType,
    pub location: Range<usize>,
}

pub struct ExprLexer {
    source: String,
    tokens: Vec<ExprToken>,
}

impl ExprLexer {
    pub fn new(s: &str) -> ExprResult<Self> {
        let source = s.to_string();
        let mut reader = InputReader::new(s);
        let mut tokens = vec![];
        lex_str(
            &mut reader,
            &mut tokens,
            &LexStrState {
                hex_re: Regex::new(r"^0x([0-9a-fA-F]+)").unwrap(),
                decimal_re: Regex::new(r"^([0-9]+)(\.[0-9]*)?").unwrap(),
                identifier_re: Regex::new(r"^[a-zA-Z][a-zA-Z0-9_]*").unwrap(),
                char_re: Regex::new(r"^[+-/*%()]").unwrap(),
            },
        )?;
        Ok(Self { source, tokens })
    }

    pub fn skip_start_of_input(&mut self) -> ExprResult<()> {
        self.take_token(ExprTokenType::StartOfInput)?;
        Ok(())
    }

    pub fn skip_end_of_input(&mut self) -> ExprResult<()> {
        self.take_token(ExprTokenType::EndOfInput)?;
        Ok(())
    }

    pub fn take_operator(&mut self) -> ExprResult<String> {
        self.take_token_map("operator", |t| match &t.token_type {
            ExprTokenType::Operator(op) => Some(op.clone()),
            _ => None,
        })
    }

    fn take_token_map<F, T>(&mut self, token_name: &str, test: F) -> ExprResult<T>
    where
        F: FnOnce(&ExprToken) -> Option<T>,
    {
        if let Some(t) = self.tokens.first() {
            if let Some(v) = test(t) {
                self.tokens.remove(0);
                Ok(v)
            } else {
                Err(ExprError::new(
                    &self.source,
                    Some(t.location.clone()),
                    &format!("expected {token_name} but found {:?}", t.token_type),
                ))
            }
        } else {
            Err(ExprError::new(&self.source, None, "reached end of input"))
        }
    }

    pub fn take_token(&mut self, token_type: ExprTokenType) -> ExprResult<ExprToken> {
        if let Some(t) = self.tokens.first() {
            if t.token_type == token_type {
                Ok(self.tokens.remove(0))
            } else {
                Err(ExprError::new(
                    &self.source,
                    Some(t.location.clone()),
                    &format!("expected {token_type:?} but found {:?}", t.token_type),
                ))
            }
        } else {
            Err(ExprError::new(&self.source, None, "reached end of input"))
        }
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn peek(&self, n: usize) -> Option<&ExprToken> {
        self.tokens.get(n)
    }

    pub fn peek_token_type(&self, n: usize) -> Option<&ExprTokenType> {
        self.tokens.get(n).map(|t| &t.token_type)
    }

    pub fn take(&mut self) -> Option<ExprToken> {
        if self.tokens.is_empty() {
            None
        } else {
            Some(self.tokens.remove(0))
        }
    }

    pub fn get_source(&self) -> &str {
        &self.source
    }

    pub fn is_next_token(&self, token_type: ExprTokenType) -> bool {
        if let Some(t) = self.peek(0) {
            t.token_type == token_type
        } else {
            false
        }
    }
}

struct LexStrState {
    hex_re: Regex,
    decimal_re: Regex,
    identifier_re: Regex,
    char_re: Regex,
}

fn lex_str(
    reader: &mut InputReader,
    tokens: &mut Vec<ExprToken>,
    state: &LexStrState,
) -> ExprResult<()> {
    tokens.push(ExprToken {
        token_type: ExprTokenType::StartOfInput,
        location: reader.get_offset()..reader.get_offset(),
    });

    let source = reader.get_source().to_string();

    while reader.len() > 0 {
        if let Some(location) = reader.try_take_str(",") {
            tokens.push(ExprToken {
                token_type: ExprTokenType::Comma,
                location,
            });
        } else if let Some(captures) = reader.try_take_re(&state.hex_re) {
            lex_hex_number(&source, captures, tokens)?;
        } else if let Some(captures) = reader.try_take_re(&state.decimal_re) {
            lex_decimal_number(&source, captures, tokens)?;
        } else if let Some(captures) = reader.try_take_re(&state.identifier_re) {
            lex_identifier(captures, tokens)?;
        } else if let Some(captures) = reader.try_take_re(&state.char_re) {
            lex_char(captures, tokens)?;
        } else if let Some(str) = reader.try_take_string()? {
            lex_string(str, tokens)?;
        } else {
            return Err(ExprError::new(
                reader.get_source(),
                Some(reader.get_offset()..reader.get_offset()),
                "unexpected character",
            ));
        }
    }

    tokens.push(ExprToken {
        token_type: ExprTokenType::EndOfInput,
        location: reader.get_offset()..reader.get_offset(),
    });

    Ok(())
}

fn lex_identifier(re_result: ReResult, tokens: &mut Vec<ExprToken>) -> ExprResult<()> {
    let s = re_result.value.get(0).unwrap().as_str();
    tokens.push(ExprToken {
        token_type: ExprTokenType::Identifier(s.to_string()),
        location: re_result.location,
    });
    Ok(())
}

fn lex_string(reader_result: ReaderResult<String>, tokens: &mut Vec<ExprToken>) -> ExprResult<()> {
    tokens.push(ExprToken {
        token_type: ExprTokenType::String(reader_result.value),
        location: reader_result.location,
    });
    Ok(())
}

fn lex_char(re_result: ReResult, tokens: &mut Vec<ExprToken>) -> ExprResult<()> {
    let s = re_result.value.get(0).unwrap().as_str();
    let token_type = if s == "(" {
        ExprTokenType::LeftParen
    } else if s == ")" {
        ExprTokenType::RightParen
    } else {
        ExprTokenType::Operator(s.to_string())
    };
    tokens.push(ExprToken {
        token_type,
        location: re_result.location,
    });
    Ok(())
}

fn lex_hex_number(
    source: &str,
    re_result: ReResult,
    tokens: &mut Vec<ExprToken>,
) -> ExprResult<()> {
    let s = re_result.value.get(0).unwrap().as_str();
    let (s, neg) = if let Some(s) = s.strip_prefix("-") {
        (s, -1)
    } else if let Some(s) = s.strip_prefix("+") {
        (s, 1)
    } else {
        (s, 1)
    };

    let s = s.trim_start_matches("0x");

    match i128::from_str_radix(s, 16) {
        Ok(v) => {
            tokens.push(ExprToken {
                token_type: ExprTokenType::HexNumber(neg * v),
                location: re_result.location,
            });
            Ok(())
        }
        Err(e) => Err(ExprError::new(
            source,
            Some(re_result.location.clone()),
            &format!("parse hexadecimal; error = {e}"),
        )),
    }
}

fn lex_decimal_number(
    source: &str,
    re_result: ReResult,
    tokens: &mut Vec<ExprToken>,
) -> ExprResult<()> {
    let s = re_result.value.get(0).unwrap().as_str();
    let v = s.parse::<f64>().map_err(|e| {
        ExprError::new(
            source,
            Some(re_result.location.clone()),
            &format!("parse decimal; error = {e}"),
        )
    })?;
    tokens.push(ExprToken {
        token_type: ExprTokenType::DecimalNumber(v),
        location: re_result.location,
    });
    Ok(())
}
