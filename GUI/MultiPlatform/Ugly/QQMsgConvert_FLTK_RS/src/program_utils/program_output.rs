#![allow(dead_code)]

pub fn print_line(text: &str, length: usize) {
    for _ in 0..length {
        print!("{}", text);
    }
    println!();
}

pub fn bool_to_binary_str(b: bool) -> &'static str {
    if b {
        "1"
    } else {
        "0"
    }
}