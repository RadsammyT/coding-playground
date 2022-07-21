use std::io::Write;

use rand::*;
use text_io::*;

pub fn run(mut input: String) -> String {
    let min = rand::thread_rng().gen_range(0..input.len());
    let len = input.len();
    return input.get_mut(min..rand::thread_rng().gen_range(min..=len)).unwrap().to_string();
}

pub fn test() {
    println!("input string to randomize a lot, PUT ONE PERIOD TO END THE MESSAGE: ");
    {let input: String = try_read!("{}.").unwrap_or("ERROR".to_string());
    println!("input: {}", input);
    for _ in 0..=100 {
        println!("{}", run((&input).to_string()));
    }}
    
}