// use std::time::SystemTime;
use std::{*, time::SystemTime};
mod rad;


fn main() {
    let pain = rad::input::line("PAIN: ".to_string());
    println!("{}", pain);

    let test: i32 = rad::input::line("integer? :".to_string()).parse::<i32>().unwrap();
}

// return int function
fn add() -> i32 {
    let mut ret = 1;
    for i in 0..10 {
        ret += i;
    }
    return ret;
}

