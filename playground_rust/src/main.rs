/*
    NOTE TO SELF:
        https://cheat.sh/rust/
        USE LINK WHEN STUCK ON SHIT
*/


// use std::time::SystemTime;
use std::{*};
use rad::{timer::Timer};
use text_io::*;
use rand::*;



mod rad;


fn main() {
    select();
    // rad::string_random::test();
}

fn select() {
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
    let mut sel: i32 = -1;
    let mut is_bad: bool = true;
    let mut main_timer = Timer::new();
    
    while is_bad {
        println!("1: shitshuffler \n2: test show \n3: collatz \n4: quick fibb, 102 numbers \n5: random slices of string \n6: closure bullshittery \n");
        print!("Select an entry: ");
        sel = try_read!().unwrap_or(-1);
        main_timer.start_timer();
        match sel {
            
            1 => {
                rad::shit_shuffler::run();
                is_bad = false;
            }
            2 => {
                rad::test::test();
                is_bad = false;
            }
            /*
                func({println!("input"); read!()})
                what the fuck
            */
            3 => { // cursed
                rad::collatz::run({println!("input"); try_read!().unwrap()},
                {println!("print steps? (true/false)"); try_read!().unwrap()});
                is_bad = false;
            }
            4 => {
                let mut vec: Vec<i128> = vec![];
                vec.push(0);
                vec.push(1);
                for i in 0..100 {
                    vec.push(vec[i] + vec[i+1]);
                }
                println!("{:?}", vec);
                is_bad = false;
            }

            5 => {
                rad::string_random::test();
                is_bad = false;
            }
            
            6 => {
                let closure = |a: i32| a * a;
                println!("square of 43: {}", closure(43));
                is_bad = false;
            }

            _ => {
                println!("invalid");
                is_bad = true;

            }
        }
        if !(is_bad) {
            break;
        }
    }
    main_timer.end_timer();
    println!("{} seconds", main_timer.get_elapsed().unwrap());
}

