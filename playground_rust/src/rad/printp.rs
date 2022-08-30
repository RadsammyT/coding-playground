use std::{time::{self, Duration}, io::{stdout, Write}};
pub fn printp(input: &'static str, delay: Duration) {
    input.chars().for_each(|c| {
        print!("{c}");
        stdout().flush().unwrap();
        std::thread::sleep(delay);
    });
    println!();
}