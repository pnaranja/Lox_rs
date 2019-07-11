#![allow(non_camel_case_types, non_snake_case)]

use std::fmt::Debug;

#[derive(Debug)]
pub enum token_type_literal {
    IDENTIFIER,
    STRING,
    NUMBER,
}

#[derive(Debug)]
pub enum token_type {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Keywords.
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

#[derive(Debug)]
pub struct Token {
    tok_type: token_type,
    pub lexeme: String,
    literal: Option<token_type_literal>,
    // Tokens might not have literals
    line: i32,
}

impl Token {
    pub fn new(tok_type: token_type, lexeme: String, literal: Option<token_type_literal>, line: i32) -> Token {
        Token { tok_type, lexeme, literal, line }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        format!("{:?} {} {:?}", self.tok_type, self.lexeme, self.literal)
    }
}

