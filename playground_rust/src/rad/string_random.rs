use std::io::Write;

use rand::*;
use text_io::*;

pub fn run(mut input: String) -> String {
    let min = rand::thread_rng().gen_range(0..input.len());
    let len = input.len();
    return input.get_mut(min..rand::thread_rng().gen_range(min..=len)).unwrap().to_string();
}

pub fn test(input: &str, range: i32) {
    for i in 0..=range {
        println!("({}): |{}|", i, run((&input).to_string()));
    }
}