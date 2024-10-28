use std::ops::Range;

use regex::Captures;

use super::{ExprError, ExprResult};

#[derive(Debug)]
pub struct InputReader<'a> {
    source: &'a str,
    offset: usize,
    s: &'a str,
}

impl<'a> InputReader<'a> {
    pub fn new(s: &'a str) -> Self {
        let mut ret = Self {
            source: s,
            offset: 0,
            s,
        };
        ret.skip_whitespace();
        ret
    }

    pub fn try_take_re(&mut self, re: &regex::Regex) -> Option<ReResult> {
        if let Some(captures) = re.captures(self.s) {
            let len = captures.get(0).unwrap().len();
            let range = self.offset..self.offset + len;
            self.s = &self.s[len..];
            self.offset += len;
            self.skip_whitespace();
            Some(ReResult {
                location: range,
                value: captures,
            })
        } else {
            None
        }
    }

    pub fn try_take_string(&mut self) -> ExprResult<Option<ReaderResult<String>>> {
        if self.s.starts_with("'") {
            let start_offset = self.offset;
            let before_len = self.s.len();

            self.s = &self.s[1..];
            let mut escape = false;
            let mut ch_count = 0;
            let mut value = "".to_string();
            for ch in self.s.chars() {
                ch_count += 1;
                if escape {
                    escape = false;
                    if ch == '\'' {
                        value.push(ch);
                    } else {
                        let offset = start_offset + ch_count;
                        return Err(ExprError::new(
                            self.source,
                            Some(offset..offset),
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

            self.s = &self.s[ch_count..];

            let after_len = self.s.len();
            self.offset += before_len - after_len;
            self.skip_whitespace();
            Ok(Some(ReaderResult {
                location: start_offset..self.offset,
                value: value.to_string(),
            }))
        } else {
            Ok(None)
        }
    }

    pub fn get_offset(&self) -> usize {
        self.offset
    }

    pub fn len(&self) -> usize {
        self.s.len()
    }

    pub fn get_source(&self) -> &str {
        self.source
    }

    fn skip_whitespace(&mut self) {
        let before_len = self.s.len();
        self.s = self.s.trim_start();
        let after_len = self.s.len();
        self.offset += before_len - after_len;
    }
}

#[derive(Debug)]
pub struct ReaderResult<T> {
    pub location: Range<usize>,
    pub value: T,
}

pub type ReResult<'a> = ReaderResult<Captures<'a>>;
