#![allow(dead_code)]

pub fn print_line(text: &str, length: usize) {
    for _ in 0..length {
        print!("{}", text);
    }
    println!();
}