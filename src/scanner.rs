use std::{cell::Cell, ops::Add};

use crate::{
    lox_error::{LoxError, Result},
    token::{Token, TokenType},
};

pub struct Scanner {
    source: String,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner { source }
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        let mut split_source = self.source.as_bytes().iter().peekable();
        let mut result: Vec<Token> = vec![];
        let mut line = 1;
        while let Some(current) = split_source.next() {
            match current {
                b'(' => result.push(Token::new(TokenType::LEFT_PAREN, "(", "(", line)),
                b')' => result.push(Token::new(TokenType::RIGHT_PAREN, ")", ")", line)),
                b'{' => result.push(Token::new(TokenType::LEFT_BRACE, "{", "{", line)),
                b'}' => result.push(Token::new(TokenType::RIGHT_BRACE, "}", "}", line)),
                b',' => result.push(Token::new(TokenType::COMMA, ",", ",", line)),
                b'.' => result.push(Token::new(TokenType::DOT, ".", ".", line)),
                b'-' => result.push(Token::new(TokenType::MINUS, "-", "-", line)),
                b'+' => result.push(Token::new(TokenType::PLUS, "+", "+", line)),
                b';' => result.push(Token::new(TokenType::SEMICOLIN, ";", ";", line)),
                b'*' => result.push(Token::new(TokenType::STAR, "*", "*", line)),
                b'!' => {
                    if let Some(peek) = split_source.peek() {
                        if **peek == b'=' {
                            result.push(Token::new(TokenType::BANG_EQUAL, "!=", "!=", line));
                            split_source.next();
                        } else {
                            result.push(Token::new(TokenType::BANG, "!", "!", line))
                        }
                    }
                }
                b'=' => {
                    if let Some(peek) = split_source.peek() {
                        if **peek == b'=' {
                            result.push(Token::new(TokenType::EQUAL_EQUAL, "==", "==", line));
                            split_source.next();
                        } else {
                            result.push(Token::new(TokenType::EQUAL, "=", "=", line))
                        }
                    }
                }
                b'<' => {
                    if let Some(peek) = split_source.peek() {
                        if **peek == b'=' {
                            result.push(Token::new(TokenType::LESS_EQUAL, "<=", "<=", line));
                            split_source.next();
                        } else {
                            result.push(Token::new(TokenType::LESS, "<", "<", line))
                        }
                    }
                }
                b'>' => {
                    if let Some(peek) = split_source.peek() {
                        if **peek == b'=' {
                            result.push(Token::new(TokenType::GREATER_EQUAL, ">=", ">=", line));
                            split_source.next();
                        } else {
                            result.push(Token::new(TokenType::GREATER, ">", ">", line))
                        }
                    }
                }
                b'/' => {
                    if let Some(peek) = split_source.peek() {
                        if **peek == b'/' {
                            while let Some(skipped) = split_source.next() {
                                if *skipped == b'\n' {
                                    line = line + 1;
                                    break;
                                }
                            }
                            split_source.next();
                        }
                    } else {
                        result.push(Token::new(TokenType::SLASH, "/", "/", line))
                    }
                }
                b' ' => (),
                b'\r' => (),
                b'\n' => {
                    line = line + 1;
                }
                _ => print!("why"),
            };
        }
        result
    }
}
