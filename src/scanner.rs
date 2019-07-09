use crate::token::{Token, token_type, token_type_literal};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,

    start_ptr_token : i32,
    current_ptr : i32,
    current_line : i32
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
     fn at_the_end(&self) -> bool{
         self.current_ptr + 1 >= self.source.len() as i32
     }

     /// Get a particular character from the source using an index
     fn get_char_from_source(&self, index: i32) -> &str{
         let uindex = index as usize; 
         &self.source[uindex..(uindex+1)]
     }

     /// This should just return a new Scanner with the current pointer increased by 1
     fn advance(self) -> Scanner {
         Scanner{
             current_ptr: self.current_ptr+1,
             ..self
         }
     }

    /// Return new Scanner with new token
    fn add_token(self, tok_type: token_type, tok_type_literal: token_type_literal) -> Scanner {
        let text = &self.source[self.start_ptr_token as usize..self.current_ptr as usize];
        let mut old_tokens = self.tokens;
        let mut new_token_vec = vec!(Token::new(tok_type, text.to_string(), Some(tok_type_literal), self.current_line));
        old_tokens.append(&mut new_token_vec);
        Scanner { tokens: old_tokens, ..self }
    }

 }
