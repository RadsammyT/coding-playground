/*
    NOTE TO SELF:
        https://cheat.sh/rust/
        USE LINK WHEN STUCK ON SHIT
*/

/*
text_io has an issue where print! won't work well with read! invocs
a guy on the git repo for text_io found the issue but wouldn't want to 
do a PR because the code is "complicated" (relatable).
so "fuck it" I said, and so I made 2 new macros that flush stdout before reading.
paste the following macros on text_io/src/lib.rs:
/// a version of ```read!()``` that flushes ```stdout``` before reading.
/// # Example
/// ```
/// print!("Xn = ");
/// let xn: f64 = readf!();
/// ```
#[macro_export]
macro_rules! readf {
	($($arg:tt)*) => {{
		use std::io::Write;
		std::io::stdout().flush().unwrap();
		read!($($arg)*)
	}};
}

/// A version of ```try_read!()``` that flushes ```stdout``` before reading.
/// # Example
/// ```rust,no_run
/// print!("Enter a number");
/// let num: i32 = try_readf!().unwrap();
/// ```
/// 
#[macro_export]
macro_rules! try_readf {
	($($arg:tt)*) => {{
		use std::io::Write;
		std::io::stdout().flush().unwrap();
		try_read!($($arg)*)
	}};
}

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
    main_timer.start_timer();
    while is_bad {
        println!("1: shitshuffler \n2: test show \n3: collatz \n4: quick fibb, 102 numbers \n5: random slices of string \n6: closure bullshittery \n");
        print!("Select an entry: ");
        sel = try_readf!().unwrap_or(-1);
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

