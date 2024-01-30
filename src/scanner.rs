use std::collections::HashMap;

use crate::{
    lox_error::{LoxError, Result},
    token::{Lexeme, Token, TokenType},
};

pub struct Scanner {
    source: String,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner { source }
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        let keywords: HashMap<&str, TokenType> = HashMap::from([
            ("and", TokenType::AND),
            ("class", TokenType::CLASS),
            ("else", TokenType::ELSE),
            ("false", TokenType::FALSE),
            ("for", TokenType::FOR),
            ("fun", TokenType::FUN),
            ("if", TokenType::IF),
            ("nil", TokenType::NIL),
            ("or", TokenType::OR),
            ("print", TokenType::PRINT),
            ("return", TokenType::RETURN),
            ("super", TokenType::SUPER),
            ("this", TokenType::THIS),
            ("true", TokenType::TRUE),
            ("var", TokenType::VAR),
            ("while", TokenType::WHILE),
        ]);
        let mut split_source = self.source.as_bytes().iter().peekable();
        let mut result: Vec<Token> = vec![];
        let mut line = 1;
        while let Some(current) = split_source.next() {
            match current {
                b'(' => result.push(Token::new(
                    TokenType::LEFT_PAREN,
                    Lexeme::Char(String::from("(")),
                    String::from("("),
                    line,
                )),
                b')' => result.push(Token::new(
                    TokenType::RIGHT_PAREN,
                    Lexeme::Char(String::from(")")),
                    String::from(")"),
                    line,
                )),
                b'{' => result.push(Token::new(
                    TokenType::LEFT_BRACE,
                    Lexeme::Char(String::from("{")),
                    String::from("{"),
                    line,
                )),
                b'}' => result.push(Token::new(
                    TokenType::RIGHT_BRACE,
                    Lexeme::Char(String::from("}")),
                    String::from("}"),
                    line,
                )),
                b',' => result.push(Token::new(
                    TokenType::COMMA,
                    Lexeme::Char(String::from(",")),
                    String::from(","),
                    line,
                )),
                b'.' => {
                    // if the next token is a num, we don't want that
                    if let Some(peek) = split_source.peek() {
                        if Self::is_digit(peek) {
                            print!("BAD")
                        }
                    }
                    result.push(Token::new(
                        TokenType::DOT,
                        Lexeme::Char(String::from(".")),
                        String::from("."),
                        line,
                    ))
                }
                b'-' => result.push(Token::new(
                    TokenType::MINUS,
                    Lexeme::Char(String::from("-")),
                    String::from("-"),
                    line,
                )),
                b'+' => result.push(Token::new(
                    TokenType::PLUS,
                    Lexeme::Char(String::from("+")),
                    String::from("+"),
                    line,
                )),
                b';' => result.push(Token::new(
                    TokenType::SEMICOLIN,
                    Lexeme::Char(String::from(";")),
                    String::from(";"),
                    line,
                )),
                b'*' => result.push(Token::new(
                    TokenType::STAR,
                    Lexeme::Char(String::from("*")),
                    String::from("*"),
                    line,
                )),
                b'!' => {
                    if let Some(peek) = split_source.peek() {
                        if **peek == b'=' {
                            result.push(Token::new(
                                TokenType::BANG_EQUAL,
                                Lexeme::Char(String::from("!=")),
                                String::from("!="),
                                line,
                            ));
                            split_source.next();
                        } else {
                            result.push(Token::new(
                                TokenType::BANG,
                                Lexeme::Char(String::from("!")),
                                String::from("!"),
                                line,
                            ))
                        }
                    }
                }
                b'=' => {
                    if let Some(peek) = split_source.peek() {
                        if **peek == b'=' {
                            result.push(Token::new(
                                TokenType::EQUAL_EQUAL,
                                Lexeme::Char(String::from("==")),
                                String::from("=="),
                                line,
                            ));
                            split_source.next();
                        } else {
                            result.push(Token::new(
                                TokenType::EQUAL,
                                Lexeme::Char(String::from("=")),
                                String::from("="),
                                line,
                            ))
                        }
                    }
                }
                b'<' => {
                    if let Some(peek) = split_source.peek() {
                        if **peek == b'=' {
                            result.push(Token::new(
                                TokenType::LESS_EQUAL,
                                Lexeme::Char(String::from("<=")),
                                String::from("<="),
                                line,
                            ));
                            split_source.next();
                        } else {
                            result.push(Token::new(
                                TokenType::LESS,
                                Lexeme::Char(String::from("<")),
                                String::from("<"),
                                line,
                            ))
                        }
                    }
                }
                b'>' => {
                    if let Some(peek) = split_source.peek() {
                        if **peek == b'=' {
                            result.push(Token::new(
                                TokenType::GREATER_EQUAL,
                                Lexeme::Char(String::from(">=")),
                                String::from(">="),
                                line,
                            ));
                            split_source.next();
                        } else {
                            result.push(Token::new(
                                TokenType::GREATER,
                                Lexeme::Char(String::from(">")),
                                String::from(">"),
                                line,
                            ))
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
                        }
                    } else {
                        result.push(Token::new(
                            TokenType::SLASH,
                            Lexeme::Char(String::from("/")),
                            String::from("/"),
                            line,
                        ))
                    }
                }
                b' ' => (),
                b'\r' => (),
                b'\n' => {
                    line = line + 1;
                }
                b'"' => {
                    let mut current_string: Vec<u8> = vec![];
                    loop {
                        if let Some(char) = split_source.next() {
                            if *char == b'\n' {
                                line = line + 1;
                            }
                            if *char == b'"' {
                                break;
                            }
                            current_string.push(*char)
                        } else {
                            break;
                        }
                    }
                    let finished = String::from_utf8_lossy(current_string.as_slice());

                    result.push(Token::new(
                        TokenType::STRING,
                        Lexeme::Char(finished.to_string()),
                        finished.to_string(),
                        line,
                    ));
                }
                char if Self::is_digit(char) => {
                    let mut full_num: Vec<u8> = vec![];
                    let mut needs_next = false;
                    let mut needs_peek = false;
                    full_num.push(*char);
                    loop {
                        if let Some(peeked) = split_source.peek() {
                            if **peeked != b'.' && !Self::is_digit(*peeked) {
                                break;
                            }
                            if Self::is_digit(*peeked) {
                                full_num.push(**peeked);
                                needs_next = true;
                            }
                            if **peeked == b'.' {
                                needs_next = true;
                                needs_peek = true;
                            }
                        }
                        if needs_next && needs_peek {
                            if let Some(next) = split_source.next() {
                                full_num.push(*next);
                            }
                            if let Some(peeked) = split_source.peek() {
                                if **peeked == b'.' {
                                    full_num.push(**peeked);
                                }
                                if !Self::is_digit(peeked) {
                                    print!("BAD")
                                }
                            }
                            needs_next = false;
                            needs_peek = false;
                        }
                        if needs_next {
                            split_source.next();
                            needs_next = false;
                        }
                    }
                    let finished = String::from_utf8_lossy(full_num.as_slice());
                    result.push(Token::new(
                        TokenType::NUMBER,
                        Lexeme::Num(finished.clone().parse::<f64>().unwrap()),
                        finished.to_string(),
                        line,
                    ));
                }
                char if Self::is_alpha(char) => {
                    let mut identifier_list: Vec<u8> = vec![];
                    identifier_list.push(*char);
                    let mut needs_next = false;
                    loop {
                        if let Some(peeked) = split_source.peek() {
                            if Self::is_alpha_numeric(peeked) {
                                needs_next = true;
                            } else {
                                break;
                            }
                        }
                        if needs_next {
                            if let Some(next) = split_source.next() {
                                identifier_list.push(*next);
                                needs_next = false;
                            }
                        }
                    }
                    let identifier = String::from_utf8_lossy(identifier_list.as_slice());
                    if let Some(keyword) = keywords.get(identifier.to_string().as_str()) {
                        result.push(Token::new(
                            *keyword,
                            Lexeme::Char(identifier.to_string()),
                            identifier.to_string(),
                            line,
                        ))
                    } else {
                        result.push(Token::new(
                            TokenType::IDENTIFIER,
                            Lexeme::Char(identifier.to_string()),
                            identifier.to_string(),
                            line,
                        ))
                    }
                }
                _ => print!("why"),
            };
        }
        result
    }

    fn is_alpha_numeric(char: &u8) -> bool {
        Self::is_alpha(char) || Self::is_digit(char)
    }

    fn is_alpha(char: &u8) -> bool {
        char.is_ascii_alphabetic() || *char == b'_'
    }

    fn is_digit(char: &u8) -> bool {
        char.is_ascii_digit()
    }
}
