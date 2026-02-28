//! Module Lexing
//!
//! Parsing of strings into instances of Tuple
//! Heavily inspired by
//!     <https://users.rust-lang.org/t/an-suggestions-improvements-for-my-lexer/6081>

use crate::tuple::{Tuple, E};
use std::{error, fmt, result};

#[derive(Debug)]
enum TokenType<'a> {
    Integer,
    Float,
    String,
    Tuple(Vec<Token<'a>>),
    Wildcard,
}

#[derive(Debug)]
struct Token<'a> {
    typ: TokenType<'a>,
    val: &'a str,
}

type Result<Token> = result::Result<Token, ParseError>;

#[derive(Debug, Clone)]
struct ParseError;

impl error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unable to parse token")
    }
}

pub struct Lexer<'a> {
    buf: &'a str,
    pos: usize,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Tuple;

    fn next(&mut self) -> Option<Tuple> {
        let chars = self.buf.chars().collect::<Vec<char>>();

        if self.pos >= chars.len() {
            return None;
        }
        let next_token = &self.match_next(&chars);
        next_token.as_ref().map_or(None, |token| {
            if let E::T(tuple) = Self::from_token(token) {
                Some(tuple)
            } else {
                Some(tuple![])
            }
        })
    }
}

impl<'a> Lexer<'a> {
    #[must_use]
    pub const fn new(buffer: &str) -> Lexer<'_> {
        Lexer {
            buf: buffer,
            pos: 0,
        }
    }

    fn match_next(&mut self, chars: &[char]) -> Result<Token<'a>> {
        match chars[self.pos] {
            // parse numbers, which can be either negative or positive
            '-' | '0'..='9' => self.parse_number(chars),
            // parse strings that are started and terminated by quote marks
            '\"' => self.parse_string(chars),
            // use a special character for wildcards
            '_' => Ok(self.parse_wildcard()),
            // parse tuples which are surrounded by parentheses
            '(' => self.parse_tuple(chars),
            ',' | ' ' => {
                self.pos += 1;
                self.match_next(chars)
            }
            _ => {
                println!("invalid symbol {}", chars[self.pos]);
                self.pos += 1;
                self.match_next(chars)
            }
        }
    }

    fn parse_number(&mut self, chars: &[char]) -> Result<Token<'a>> {
        let start = self.pos;
        let mut is_float = false;
        while self.pos < chars.len() {
            match chars[self.pos] {
                '0'..='9' => self.pos += 1,
                '-' => {
                    if self.pos == start {
                        self.pos += 1;
                    } else {
                        // only allow a minus at the end of a number
                        break;
                    }
                }
                '.' => {
                    if is_float {
                        return Err(ParseError);
                    }
                    is_float = true;
                    self.pos += 1;
                }
                _ => break,
            }
        }

        let typ = if is_float {
            TokenType::Float
        } else {
            TokenType::Integer
        };
        Ok(Token {
            typ,
            val: &self.buf[start..self.pos],
        })
    }

    fn parse_string(&mut self, chars: &[char]) -> Result<Token<'a>> {
        self.pos += 1;
        let start = self.pos;
        while chars[self.pos] != '\"' {
            self.pos += 1;

            if self.pos >= chars.len() {
                eprintln!("error: incomplete string!");
                return Err(ParseError);
            }
        }
        let end = self.pos;
        self.pos += 1;

        // println!("found string from {} to {}", start, self.pos);
        // panic!();

        Ok(Token {
            typ: TokenType::String,
            val: &self.buf[start..end],
        })
    }

    fn parse_wildcard(&mut self) -> Token<'a> {
        let start = self.pos;
        self.pos += 1;
        Token {
            typ: TokenType::Wildcard,
            val: &self.buf[start - 1..self.pos],
        }
    }

    fn parse_tuple(&mut self, chars: &[char]) -> Result<Token<'a>> {
        let start = self.pos;
        self.pos += 1;
        let mut tuple_items: Vec<Token<'a>> = Vec::new();
        while chars[self.pos] != ')' {
            let token = self.match_next(chars)?;
            tuple_items.push(token);

            if self.pos >= chars.len() {
                eprintln!("error: incomplete tuple");
                return Err(ParseError);
            }
            // eprintln!("pointer: {}", self.pos);
        }
        self.pos += 1;
        Ok(Token {
            typ: TokenType::Tuple(tuple_items),
            val: &self.buf[start..self.pos],
        })
    }

    fn from_token(token: &Token<'a>) -> E {
        match token {
            Token {
                typ: TokenType::Integer,
                val,
            } => E::I(val.parse::<i32>().unwrap()),
            Token {
                typ: TokenType::Float,
                val,
            } => E::D(val.parse::<f64>().unwrap()),
            Token {
                typ: TokenType::String,
                val,
            } => E::S((*val).to_string()),
            Token {
                typ: TokenType::Wildcard,
                val: _,
            } => E::Any,
            Token {
                typ: TokenType::Tuple(tokenlist),
                val: _,
            } => E::T(Tuple::from_vec(
                tokenlist.iter().map(Self::from_token).collect(),
            )),
        }
    }
}
