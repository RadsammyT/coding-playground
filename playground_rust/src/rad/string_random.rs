use std::io::Write;

use rand::*;
use text_io::*;

pub fn run(mut input: String) -> String {
    let min = rand::thread_rng().gen_range(0..input.len());
    let len = input.len();
    return input.get_mut(min..rand::thread_rng().gen_range(min..=len)).unwrap().to_string();
}

pub fn test() {
    println!("input string to randomize a lot, INSERT '<END>' TO END STRING (because text_io is weird and counts prevous newlines used in read!() macro invocations): ");
    let input: String = try_read!("{}<END>").unwrap_or("ERROR".to_string());
    println!("input: {}", input);
    for _ in 0..=100 {
        println!("{}", run((&input).to_string()));
    }
    
}