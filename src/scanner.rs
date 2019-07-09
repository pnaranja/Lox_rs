use crate::token::{Token, token_type, token_type_literal};

pub struct Scanner {
    source: String,
    tokens: Option<Vec<Token>>,

    start_ptr_token : i32,
    current_ptr : i32,
    current_line : i32
}

impl Scanner {
    pub fn new(the_source: String) -> Scanner {
         Scanner {
             source: the_source, 
             tokens: None,
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

 }
