use std::ops::Range;

use regex::Regex;

use super::{
    reader::{InputReader, ReaderResult},
    ExprError, ExprResult,
};

#[derive(Debug, PartialEq)]
pub enum ExprTokenType {
    StartOfInput,
    EndOfInput,
    DecimalNumber,
    HexNumber,
    Operator,
    Identifier,
    String,
    LeftParen,
    RightParen,
    Comma,
}

#[derive(Debug)]
pub struct ExprToken {
    pub token_type: ExprTokenType,
    pub location: Range<usize>,
    pub text: String,
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
                char_re: Regex::new(r"^[+-/*%()^]").unwrap(),
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
        text: "".to_string(),
    });

    while reader.len() > 0 {
        if let Some(capture) = reader.try_take_str(",") {
            tokens.push(ExprToken {
                token_type: ExprTokenType::Comma,
                location: capture.location.clone(),
                text: capture.text.to_string(),
            });
        } else if let Some(captures) = reader.try_take_re(&state.hex_re) {
            lex_hex_number(&captures, tokens)?;
        } else if let Some(captures) = reader.try_take_re(&state.decimal_re) {
            lex_decimal_number(&captures, tokens)?;
        } else if let Some(captures) = reader.try_take_re(&state.identifier_re) {
            lex_identifier(&captures, tokens)?;
        } else if let Some(captures) = reader.try_take_re(&state.char_re) {
            lex_char(&captures, tokens)?;
        } else if let Some(captures) = reader.try_take_string()? {
            lex_string(&captures, tokens)?;
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
        text: "".to_string(),
    });

    Ok(())
}

fn lex_identifier(re_result: &ReaderResult, tokens: &mut Vec<ExprToken>) -> ExprResult<()> {
    tokens.push(ExprToken {
        token_type: ExprTokenType::Identifier,
        location: re_result.location.clone(),
        text: re_result.text.to_string(),
    });
    Ok(())
}

fn lex_string(re_result: &ReaderResult, tokens: &mut Vec<ExprToken>) -> ExprResult<()> {
    tokens.push(ExprToken {
        token_type: ExprTokenType::String,
        location: re_result.location.clone(),
        text: re_result.text.to_string(),
    });
    Ok(())
}

fn lex_char(re_result: &ReaderResult, tokens: &mut Vec<ExprToken>) -> ExprResult<()> {
    let token_type = if re_result.text == "(" {
        ExprTokenType::LeftParen
    } else if re_result.text == ")" {
        ExprTokenType::RightParen
    } else {
        ExprTokenType::Operator
    };
    tokens.push(ExprToken {
        token_type,
        location: re_result.location.clone(),
        text: re_result.text.to_string(),
    });
    Ok(())
}

fn lex_hex_number(re_result: &ReaderResult, tokens: &mut Vec<ExprToken>) -> ExprResult<()> {
    tokens.push(ExprToken {
        token_type: ExprTokenType::HexNumber,
        location: re_result.location.clone(),
        text: re_result.text.to_string(),
    });
    Ok(())
}

fn lex_decimal_number(re_result: &ReaderResult, tokens: &mut Vec<ExprToken>) -> ExprResult<()> {
    tokens.push(ExprToken {
        token_type: ExprTokenType::DecimalNumber,
        location: re_result.location.clone(),
        text: re_result.text.to_string(),
    });
    Ok(())
}
