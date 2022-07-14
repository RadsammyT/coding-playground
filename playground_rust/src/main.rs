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

    // select();
     static SEX_NUMBER: i32 = 69;
     println!("{}", SEX_NUMBER);

}

fn select() {
    println!("1: shitshuffler \n2: test show \n3: collatz \n4: quick fibb, 102 numbers");
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
     * you get the point... right?
     */
    let sel: i32 = try_read!().unwrap_or(-1);
    let mut mainTimer = Timer::new();
    Timer::start_timer(&mut mainTimer);
    match sel {
        
        1 => {
            rad::shit_shuffler::run();
        }
        2 => {
            rad::test::test();
        }
        /*
            func({println!("input"); read!()})
            what the fuck
        */
        3 => { // cursed
            rad::collatz::run({println!("input"); try_read!().unwrap()},
            {println!("print steps? (true/false)"); try_read!().unwrap()});
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
    Timer::end_timer(&mut mainTimer);
    println!("{} seconds", Timer::get_elapsed(&mut mainTimer).unwrap());
}

