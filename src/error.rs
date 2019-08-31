pub fn report(line_num: i8, msg: String) {
    eprintln!("[line {}] Error: {}", line_num, msg);
}
