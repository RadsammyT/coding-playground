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
    select();
}

fn select() {
    println!("1: shitshuffler \n2: test show \n3: collatz \n4: test");
    /*
     * because println! counts indentation when calling it across multiple lines
     * example:
     * fn main() {
     *      println!("test
     *      test");
     * }
     * output:
     * test
     *      test
     * 
     * you get the point
     */
    let sel: i32 = try_read!().unwrap_or(-1);
    match sel {
        
        1 => {
            rad::shit_shuffler::run();
        }
        2 => {
            rad::test::test();
        }
        3 => {
            rad::collatz::run(try_read!().unwrap_or(15), try_read!().unwrap_or(true));
        }
        4 => {
            let mut vec: Vec<i128> = vec![];
            vec.push(0);
            vec.push(1);
            for i in 0..100 {
                vec.push(vec[i] + vec[i+1]);
            }
            println!("{:?}", vec);
        }
        _ => {
            println!("invalid");
            select();
        }
    }
}

