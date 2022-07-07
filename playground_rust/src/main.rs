/*
    NOTE TO SELF:
        https://cheat.sh/rust/
        USE LINK WHEN STUCK ON SHIT
*/

// use std::time::SystemTime;
use std::{*, time::SystemTime};
use rad::timer::Timer;
use text_io::*; // cool beans user input lib
use rand::*;
mod rad;


fn main() {
    // let mut vec: Vec<i128> = vec![];
    // for i in 0..10 {
    //     vec.push(rand::thread_rng().gen_range(0..11));
    // }
    // println!("{:?}", vec);
    let mut timer = Timer {
        start: None,
        end: None,
    };
    Timer::start_timer(&mut timer);
    rad::shit_shuffler::run();
    Timer::end_timer(&mut timer);
    println!("{}",Timer::get_elapsed(&mut timer).unwrap());
}

// return int function
fn _add() -> i32 {
    let mut ret = 1;
    for i in 0..10 {
        ret += i;
    }
    return ret;
}

