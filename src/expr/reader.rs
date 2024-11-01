use std::ops::Range;

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

    pub fn try_take_str(&mut self, s: &str) -> Option<ReaderResult> {
        if self.s.starts_with(s) {
            let len = s.len();
            let location = self.offset..self.offset + len;
            self.s = &self.s[len..];
            self.offset += len;
            self.skip_whitespace();
            Some(ReaderResult {
                location: location.clone(),
                text: &self.source[location.clone()],
            })
        } else {
            None
        }
    }

    pub fn try_take_re(&mut self, re: &regex::Regex) -> Option<ReaderResult> {
        if let Some(captures) = re.captures(self.s) {
            let len = captures.get(0).unwrap().len();
            let location = self.offset..self.offset + len;
            self.s = &self.s[len..];
            self.offset += len;
            self.skip_whitespace();
            Some(ReaderResult {
                location: location.clone(),
                text: &self.source[location],
            })
        } else {
            None
        }
    }

    pub fn try_take_string(&mut self) -> ExprResult<Option<ReaderResult>> {
        if !self.s.starts_with("'") {
            return Ok(None);
        }
        let start = self.offset;
        self.s = &self.s[1..];
        self.offset += 1;

        let mut escape = false;
        while !self.s.is_empty() {
            if escape {
                self.s = &self.s[1..];
                self.offset += 1;
                escape = false;
            } else if self.s.starts_with("'") {
                self.s = &self.s[1..];
                self.offset += 1;
                let location = start..self.offset;
                return Ok(Some(ReaderResult {
                    location: location.clone(),
                    text: &self.source[location.clone()],
                }));
            } else if self.s.starts_with("\\") {
                self.s = &self.s[1..];
                self.offset += 1;
                escape = true;
            } else {
                self.s = &self.s[1..];
                self.offset += 1;
            }
        }
        Err(ExprError {
            source: self.source.to_owned(),
            location: Some(self.offset..self.offset),
            message: "missing closing quote".to_string(),
        })
    }

    pub fn get_offset(&self) -> usize {
        self.offset
    }

    pub fn len(&self) -> usize {
        self.s.len()
    }

    pub fn get_source(&'a self) -> &'a str {
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
pub struct ReaderResult<'a> {
    pub location: Range<usize>,
    pub text: &'a str,
}
