/*
    NOTE TO SELF:
        https://cheat.sh/rust/
        USE LINK WHEN STUCK ON SHIT
*/


// use std::time::SystemTime;
use std::{*, io::{Write, Read}};
use rad::timer::Timer;
use text_io::*;


mod rad;


fn main() {
    select();
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
    let mut sel: i32;
    let mut is_bad: bool = true;
    let mut main_timer = Timer::new();
    
    while is_bad {
        println!("1: shitshuffler \n2: test show \n3: collatz \n4: quick fibb \n5: random slices of string \n6: closure bullshittery \n7: Union test \n8: shitshuffler, multithreading edition \n9: timer epoch");
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
                rad::collatz::run({print!("input: "); try_read!().unwrap()},
                {println!("print steps? (true/false): "); try_read!().unwrap()});
                is_bad = false;
            }
            4 => {
                let mut vec: Vec<i128> = vec![];
                vec.push(0);
                vec.push(1);
                let size = {
                    print!("How many numbers?: "); 
                    try_read!().unwrap_or(50) - 2
                };
                for i in 0..size {
                    vec.push(vec[i] + vec[i+1]);
                }
                println!("{:?}", vec);
                is_bad = false;
            }

            5 => {
                let input: String = {print!("input: "); 
                                    try_read!("\n{}\n").unwrap()};
                let range: i32 = {print!("range: ");
                                try_read!().unwrap_or(100)};

                rad::string_random::test(input.as_str(), range);
                is_bad = false;
            }
            
            6 => {
                let closure = |a: i32| a * a;
                let other_closure = |b: i32| -> i32 {
                    return b + b;
                };
                println!("square of 43: {}", closure(43));
                println!("43 times 2: {}", other_closure(43));
                is_bad = false;
            }

            7 => {
                union Test {
                    v1: u32,
                    v2: u16
                }
                let mut test = Test {v1: 2000000000};
                test.v2 = u16::MAX;
                unsafe {
                    println!("Union: v1 as u32 (UNION SIZE), v2 as u16. \nv1 = 2000000000, v2 = u16::MAX");
                    println!("{:#32b} = {} \n{:#16b} = {}", test.v1, test.v1, test.v2, test.v2);
                }
                is_bad = false;
            }

            8 => {
                let length = {print!("length?: "); try_read!().unwrap_or(15)};
                let thread_num = {print!("repeat how many times? (# of threads): "); try_read!().unwrap_or(10)};
                let mut threads = vec![];
                for i in 0..thread_num {
                    threads.push(thread::spawn(move || {
                        println!("{:?}", rad::shit_shuffler::run_singular(length));
                    }));
                }

                for i in threads {
                    let _res = i.join();
                }
                is_bad = false;
            }

            9 => {
                let mut timer = Timer::new();
                timer.start_timer();
                println!("{:?}", timer.get_epoch(true));

                timer.end_timer();

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
    user_halt();
}

fn user_halt() {
    if std::env::consts::OS == "windows" {
        let _ = std::process::Command::new("cmd.exe").arg("/c").arg("pause").status();
    } else {
        print!("press enter/return to continue... ");
        let _ = try_read!("\n{}\n").unwrap_or(-1);
    }

}

