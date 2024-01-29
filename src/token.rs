use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum TokenType {
    // Single character tokens
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLIN,
    SLASH,
    STAR,
    // 1-2 character tokens
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,
    // Literals
    IDENTIFIER,
    STRING,
    NUMBER,
    // Keywords
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
    line: isize,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: String, line: isize) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
    pub fn to_string(&self) -> String {
        format!("{} {} {}", self.token_type, self.lexeme, self.literal)
    }
}
