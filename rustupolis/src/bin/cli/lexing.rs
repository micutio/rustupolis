//! Module Lexing
//!
//! Parsing of strings into instances of Tuple
//! Heavily inspired by https://users.rust-lang.org/t/an-suggestions-improvements-for-my-lexer/6081

use rustupolis::tuple::{Tuple, E};

#[derive(Debug)]
enum TokenType<'a> {
    Integer,
    Float,
    String,
    Tuple(Vec<Option<Token<'a>>>),
    Wildcard,
}

#[derive(Debug)]
struct Token<'a> {
    typ: TokenType<'a>,
    val: &'a str,
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
        if let E::T(tuple) = Lexer::from_token(&self.match_next(&chars)) {
            Some(tuple)
        } else {
            Some(tuple![])
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(buffer: &str) -> Lexer {
        Lexer {
            buf: buffer,
            pos: 0,
        }
    }

    fn match_next(&mut self, chars: &[char]) -> Option<Token<'a>> {
        match chars[self.pos] {
            // parse numbers, which can be either negative or positive
            '-' | '0'..='9' => self.parse_number(&chars),
            // parse strings that are started and terminated by quote marks
            '\"' => self.parse_string(&chars),
            // use a special character for wildcards
            '_' => self.parse_wildcard(),
            // parse tuples which are surrounded by parentheses
            '(' => self.parse_tuple(&chars),
            ',' => {
                self.pos += 1;
                None
            }
            _ => {
                println!("invalid symbol {}", chars[self.pos]);
                self.pos += 1;
                None
            }
        }
    }

    fn parse_number(&mut self, chars: &[char]) -> Option<Token<'a>> {
        let start = self.pos;
        let mut is_float = false;
        while self.pos < chars.len() {
            match chars[self.pos] {
                '0'..='9' => self.pos += 1,
                '.' => {
                    if is_float {
                        // TODO: throw error
                        panic!("float number with double decimal points");
                    } else {
                        is_float = true;
                        self.pos += 1
                    }
                }
                _ => break,
            }
        }

        let typ = if is_float {
            TokenType::Float
        } else {
            TokenType::Integer
        };
        Some(Token {
            typ,
            val: &self.buf[start..self.pos],
        })
    }

    fn parse_string(&mut self, chars: &[char]) -> Option<Token<'a>> {
        self.pos += 1;
        let start = self.pos;
        while chars[self.pos] != '\"' {
            self.pos += 1;

            if self.pos >= chars.len() {
                eprintln!("error: incomplete string!");
                return None;
            }
        }
        self.pos += 1;

        // println!("found string from {} to {}", start, self.pos);
        // panic!();

        Some(Token {
            typ: TokenType::String,
            val: &self.buf[start..self.pos],
        })
    }

    fn parse_wildcard(&mut self) -> Option<Token<'a>> {
        let start = self.pos;
        self.pos += 1;
        Some(Token {
            typ: TokenType::Wildcard,
            val: &self.buf[start - 1..self.pos],
        })
    }

    fn parse_tuple(&mut self, chars: &[char]) -> Option<Token<'a>> {
        let start = self.pos;
        self.pos += 1;
        let mut tuple_items: Vec<Option<Token<'a>>> = Vec::new();
        while chars[self.pos] != ')' {
            if let Some(token_opt) = self.match_next(chars) {
                tuple_items.push(Some(token_opt));
            }
            if self.pos >= chars.len() {
                eprintln!("error: incomplete tuple");
                return None;
            }
            // eprintln!("pointer: {}", self.pos);
        }
        self.pos += 1;
        Some(Token {
            typ: TokenType::Tuple(tuple_items),
            val: &self.buf[start..self.pos],
        })
    }

    fn from_token(token_opt: &Option<Token>) -> E {
        match token_opt {
            Some(Token {
                typ: TokenType::Integer,
                val,
            }) => E::I(val.parse::<i32>().unwrap()),
            Some(Token {
                typ: TokenType::Float,
                val,
            }) => E::D(val.parse::<f64>().unwrap()),
            Some(Token {
                typ: TokenType::String,
                val,
            }) => E::S((*val).to_string()),
            Some(Token {
                typ: TokenType::Wildcard,
                val: _,
            }) => E::Any,
            Some(Token {
                typ: TokenType::Tuple(tokenlist),
                val: _,
            }) => E::T(Tuple::from_vec(
                tokenlist.iter().map(|t| Lexer::from_token(t)).collect(),
            )),
            None => E::None,
        }
    }
}
