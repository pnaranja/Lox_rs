use crate::token::{Token, token_type, token_type_literal};

#[derive(Debug)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,

    start_ptr_token : i32,
    current_ptr : i32,
    current_line : i32
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
             current_line: 0
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
    /// and the character that was just passed over
    fn advance(self) -> Scanner {
         Scanner{
             current_ptr: self.current_ptr+1,
             ..self
         }
     }

    /// Return new Scanner with new token
    fn add_token(self, tok_type: token_type, tok_type_literal: Option<token_type_literal>) -> Scanner {
        let text = &self.source[self.start_ptr_token as usize..self.current_ptr as usize];
        let mut current_tokens = self.tokens;

        let mut new_token_vec = match tok_type_literal {
            Some(tok_type_literal) => vec!(Token::new(tok_type, text.to_string(), Some(tok_type_literal), self.current_line)),
            None => vec!(Token::new(tok_type, text.to_string(), None, self.current_line)),
        };

        current_tokens.append(&mut new_token_vec);
        Scanner { tokens: current_tokens, ..self }
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

        println!("tok_str: {:?}", tok_str_1);
        assert_eq!(scanner.current_ptr, 2);
        assert_eq!(tok_str_1.lexeme, "(");
        assert_eq!(tok_str_2.lexeme, ")");
    }
}
