use crate::token::Token;

pub struct Scanner<T>{
    source: String,
    tokens : Vec<Token<T>>,

    start_ptr : i32,
    current_ptr : i32,
    current_line : i32
}

// impl Scanner{
//     fn new(src: String) -> Scanner{
//         Scanner{src, };
// 
//     }
// }
