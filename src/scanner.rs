use crate::token::token_type::FALSE;
use crate::token::{token_type, token_type_literal, Token};

#[derive(Debug)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,

    start_ptr_token: i32,
    current_ptr: i32,
    current_line: i32,
}

impl ToString for Scanner {
    fn to_string(&self) -> String {
        format!("{} - {:?}", self.source, self.tokens)
    }
}

impl Scanner {
    pub fn new(the_source: String) -> Scanner {
        Scanner {
            source: the_source,
            tokens: vec![],
            start_ptr_token: 0,
            current_ptr: 0,
            current_line: 0,
        }
    }

    /// Finished scanning the source when the current pointer is at the last character of the
    /// source
    fn at_the_end(&self) -> bool {
        self.current_ptr + 1 >= self.source.len() as i32
    }

    /// Get a particular character from the source using an index
    fn get_char_from_source(&self) -> String {
        self.source[self.start_ptr_token as usize..self.current_ptr as usize].to_owned()
    }

    /// Set start pointer = current pointer
    fn reset_start_ptr(self) -> Scanner {
        Scanner {
            start_ptr_token: self.current_ptr,
            ..self
        }
    }

    /// This should return a new Scanner with the current pointer increased by 1
    fn advance(self) -> Scanner {
        Scanner {
            current_ptr: self.current_ptr + 1,
            ..self
        }
    }

    /// Check if next token will match expected character
    fn check_ahead(self, expected: char) -> bool {
        if self.at_the_end() {
            false
        } else {
            if expected != self.get_char_from_source() {
                false
            }
            true
        }
    }

    /// Return new Scanner with new token
    fn add_token(
        self,
        tok_type: token_type,
        tok_type_literal: Option<token_type_literal>,
    ) -> Scanner {
        let text = &self.source[self.start_ptr_token as usize..self.current_ptr as usize];
        let mut current_tokens = self.tokens;

        let mut new_token_vec = match tok_type_literal {
            Some(tok_type_literal) => vec![Token::new(
                tok_type,
                text.to_string(),
                Some(tok_type_literal),
                self.current_line,
            )],
            None => vec![Token::new(
                tok_type,
                text.to_string(),
                None,
                self.current_line,
            )],
        };

        current_tokens.append(&mut new_token_vec);
        Scanner {
            tokens: current_tokens,
            ..self
        }
    }

    /// PreCondition: start and current pointers are the same
    fn scan_token(self) -> Scanner {
        let scanner = self.advance();
        let c = scanner.get_char_from_source();

        match c.as_str() {
            "(" => scanner.add_token(token_type::LEFT_PAREN, None),
            ")" => scanner.add_token(token_type::RIGHT_PAREN, None),
            "{" => scanner.add_token(token_type::RIGHT_BRACE, None),
            "}" => scanner.add_token(token_type::LEFT_BRACE, None),
            "." => scanner.add_token(token_type::DOT, None),
            "-" => scanner.add_token(token_type::MINUS, None),
            "+" => scanner.add_token(token_type::PLUS, None),
            ";" => scanner.add_token(token_type::SEMICOLON, None),
            "*" => scanner.add_token(token_type::STAR, None),
            _ => scanner,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_tokens() {
        let scanner = Scanner::new("()".to_string()).advance();

        let scanner = scanner.add_token(token_type::LEFT_PAREN, None);
        let scanner = scanner.reset_start_ptr().advance();
        let scanner = scanner.add_token(token_type::RIGHT_PAREN, None);

        let tok_str_1: &Token = scanner.tokens.get(0).unwrap();
        let tok_str_2: &Token = scanner.tokens.get(1).unwrap();

        println!("Tokens: {:?}", scanner.tokens);
        assert_eq!(scanner.current_ptr, 2);
        assert_eq!(tok_str_1.lexeme, "(");
        assert_eq!(tok_str_2.lexeme, ")");
    }

    #[test]
    fn test_scan_single_character_tokens() {
        let scanner = Scanner::new("()}{;*+--+".to_string())
            .scan_token()
            .reset_start_ptr()
            .scan_token()
            .reset_start_ptr()
            .scan_token()
            .reset_start_ptr()
            .scan_token()
            .reset_start_ptr()
            .scan_token()
            .reset_start_ptr()
            .scan_token()
            .reset_start_ptr()
            .scan_token()
            .reset_start_ptr()
            .scan_token()
            .reset_start_ptr()
            .scan_token()
            .reset_start_ptr()
            .scan_token()
            .reset_start_ptr();

        let tok_str_beginning: &Token = scanner.tokens.get(0).unwrap();
        let tok_str_middle: &Token = scanner.tokens.get(4).unwrap();
        let tok_str_end: &Token = scanner.tokens.get(9).unwrap();

        println!("Tokens: {:?}", scanner.tokens);
        assert_eq!(scanner.current_ptr, 10);
        assert_eq!(tok_str_beginning.lexeme, "(");
        assert_eq!(tok_str_middle.lexeme, ";");
        assert_eq!(tok_str_end.lexeme, "+");
    }
}
