use std::cell::Cell;

use crate::{
    lox_error::{LoxError, Result},
    token::{Token, TokenType},
};

/*
    This section feels really gross. None of it is idiomatic Rust in the sense of
    opting for semi-mutable global state across the struct. There is a ton of overhead
    managing things like the start, current, and line values in addition to the poor
    handling of retrieving some optional values. For right now, it does in fact work
    for the purposes of building a "working" scanner.
*/

pub struct Scanner {
    source: String,
    start: Cell<usize>,
    current: Cell<usize>,
    line: Cell<isize>,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            start: Cell::new(0),
            current: Cell::new(0),
            line: Cell::new(1),
        }
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        while !self.is_at_end() {
            self.start.set(self.current.get());
            tokens.push(self.scan_token().unwrap());
        }

        tokens.push(Token::new(
            TokenType::EOF,
            String::from(""),
            String::from(""),
            self.line.get(),
        ));
        tokens
    }

    fn is_at_end(&self) -> bool {
        self.current.get() >= self.source.len()
    }

    fn iterate_current(&self) {
        self.current.set(self.current.get() + 1);
    }

    fn advance(&self) -> &str {
        let current = self.get_current_char();
        self.iterate_current();
        current
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        if let Some(text) = self.source.get(self.start.get()..self.current.get()) {
            Token::new(
                token_type,
                String::from(text),
                String::from(text),
                self.line.get(),
            )
        } else {
            // TODO: This is ugly, fix this
            panic!()
        }
    }

    fn get_current_char(&self) -> &str {
        if let Some(char) = self.source.get(self.current.get()..self.current.get() + 1) {
            char
        } else {
            // TODO: This is ugly, fix this
            panic!()
        }
    }

    fn is_match(&self, char: &str) -> bool {
        if !self.is_at_end() && self.get_current_char() == char {
            self.iterate_current();
            return true;
        }
        false
    }

    fn scan_token(&self) -> Result<Token> {
        match self.advance() {
            "(" => Ok(self.make_token(TokenType::LEFT_PAREN)),
            ")" => Ok(self.make_token(TokenType::RIGHT_PAREN)),
            "{" => Ok(self.make_token(TokenType::LEFT_BRACE)),
            "}" => Ok(self.make_token(TokenType::RIGHT_BRACE)),
            "," => Ok(self.make_token(TokenType::COMMA)),
            "." => Ok(self.make_token(TokenType::DOT)),
            "-" => Ok(self.make_token(TokenType::MINUS)),
            "+" => Ok(self.make_token(TokenType::PLUS)),
            ";" => Ok(self.make_token(TokenType::SEMICOLIN)),
            "*" => Ok(self.make_token(TokenType::STAR)),
            "!" if self.is_match("=") => Ok(self.make_token(TokenType::BANG_EQUAL)),
            "!" => Ok(self.make_token(TokenType::BANG)),
            "=" if self.is_match("=") => Ok(self.make_token(TokenType::EQUAL_EQUAL)),
            "=" => Ok(self.make_token(TokenType::EQUAL)),
            "<" if self.is_match("=") => Ok(self.make_token(TokenType::LESS_EQUAL)),
            "<" => Ok(self.make_token(TokenType::LESS)),
            ">" if self.is_match("=") => Ok(self.make_token(TokenType::GREATER_EQUAL)),
            ">" => Ok(self.make_token(TokenType::GREATER)),
            _ => Err(LoxError),
        }
    }
}
