#![allow(non_camel_case_types, non_snake_case)]

use std::fmt::Debug;

#[derive(Debug)]
enum token_type {
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

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

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

struct Token<T> {
    tok_type: token_type,
    lexeme: String,
    literal: T,
    line: i32,
}

impl<T> Token<T> {
    fn new(tok_type: token_type, lexeme: String, literal: T, line: i32) -> Token<T> {
        Token { tok_type, lexeme, literal, line }
    }
}

impl<T> ToString for Token<T>
    where T: Debug {
    fn to_string(&self) -> String {
        format!("{:?} {} {:?}", self.tok_type, self.lexeme, self.literal)
    }
}