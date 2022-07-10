/*
    NOTE TO SELF:
        https://cheat.sh/rust/
        USE LINK WHEN STUCK ON SHIT
*/

// use std::time::SystemTime;
use std::{*};
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

    // let mut timer = Timer {
    //     start: None,
    //     end: None,
    // };
    // Timer::start_timer(&mut timer);
    // rad::shit_shuffler::run();
    // Timer::end_timer(&mut timer);
    // println!("{}",Timer::get_elapsed(&mut timer).unwrap());
    
    //rad::test::test();
    select();
}

fn select() {
    println!("1: shitshuffler \n2: test print");
    let sel: i32 = try_read!().unwrap_or(-1);
    match sel {
        
        1 => {
            rad::shit_shuffler::run();
        }
        2 => {
            println!("cool beans");
        }
        _ => println!("invalid")
    }
}
