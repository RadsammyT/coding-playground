/*
    NOTE TO SELF:
        https://cheat.sh/rust/
        USE LINK WHEN STUCK ON SHIT
*/

// use std::time::SystemTime;
use std::{*};
use rad::{timer::Timer, text_io_readf};
use text_io::*;
use rand::*;



mod rad;
/*
???
    FIXME:
    copy pasting the below code into the lib file for text_io
    and calling the macro from there returns an error
*/
// macro_rules! readf {
// 	($($arg:tt)*) => {{
// 		use std::io::Write;
// 		std::io::stdout().flush().unwrap();
// 		read!($($arg)*)
// 	}};
// }

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
    let mut isBad: bool = true;
    let mut main_timer = Timer::new();
    main_timer.start_timer();
    while isBad {
        println!("1: shitshuffler \n2: test show \n3: collatz \n4: quick fibb, 102 numbers \n5: random slices of string \n6: closure bullshittery \n");
        print!("Select an entry: ");
        sel = readf!().unwrap_or(-1);
        match sel {
            
            1 => {
                rad::shit_shuffler::run();
                isBad = false;
            }
            2 => {
                rad::test::test();
                isBad = false;
            }
            /*
                func({println!("input"); read!()})
                what the fuck
            */
            3 => { // cursed
                rad::collatz::run({println!("input"); try_read!().unwrap()},
                {println!("print steps? (true/false)"); try_read!().unwrap()});
                isBad = false;
            }
            4 => {
                let mut vec: Vec<i128> = vec![];
                vec.push(0);
                vec.push(1);
                for i in 0..100 {
                    vec.push(vec[i] + vec[i+1]);
                }
                println!("{:?}", vec);
                isBad = false;
            }

            5 => {
                rad::string_random::test();
                isBad = false;
            }
            
            6 => {
                let closure = |a: i32| a * a;
                println!("square of 43: {}", closure(43));
                isBad = false;
            }

            _ => {
                println!("invalid");
                isBad = true;

            }
        }
        if !(isBad) {
            break;
        }
    }
    main_timer.end_timer();
    println!("{} seconds", main_timer.get_elapsed().unwrap());
}

