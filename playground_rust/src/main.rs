/*
    NOTE TO SELF:
        https://cheat.sh/rust/
        USE& LINK WHEN STUCK ON SHIT
*/


use std::{thread, ffi::OsStr};

// use std::time::SystemTime;
use rad::timer::Timer;
use rustils::parse::string::ToStr;
use text_io::*;
use console::*;

mod rad;
mod aoc2022;

fn main() {
    select();
    println!("relaunch program, because idfk how to clear/switch rusts hardcoded stdin buffer");
    user_halt();
    
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
    let mut main_timer = Timer::default();
    let term = console::Term::stdout();
    term.clear_screen().expect("uh oh, terminal should be clear on start");

    while is_bad {
        // println!("{} \n1: shitshuffler \n2: test select \n3: collatz \n4: quick fibb \n5: random slices of string \n6: closure bullshittery \n7: shitshuffler, multithreading edition \n8: timer epoch \n9: file reading (change path in main.rs, fn select()) \n{}", style("welcome").underlined().fg(Color::Blue), style("0 TO EXIT").bg(Color::Red).blink());
        println!("{} \n1: test selection \n2: shitshuffler \n3: collatz \n4: quick fibb \n5: random slices, string\n{} ", style("welcome").underlined().fg(Color::Blue), style("0 TO EXIT").bg(Color::Red).blink());

        print!("Select an entry: ");
        sel = try_read!().unwrap_or(-1);
        main_timer.start_timer();
        match sel {
            0 => {
                is_bad = false;
            }
            
            1 => {
                main_timer.start_timer();
                rad::test::select(&mut main_timer);
                is_bad = false;
            }

            2 => {
                term.clear_screen().expect("uh oh");
                let mut shit_shuf_select: i32;
                let mut shuf_is_bad = true;
                println!("1: shitshuffler, no multithreading \n2: shitshuffler w/ multithreading \n3: shitshuffler in egui \n{}", style("0 TO EXIT").bg(Color::Red).blink());
                println!("{}", style("Its recommended to run shitshuffler with optimizations on. \nyou can do so by adding a --release tag to cargo run/build.").yellow());
                loop {
                    print!("select shitshuffler entry: ");
                    shit_shuf_select = try_read!().unwrap_or(1);
                    match shit_shuf_select {

                        0 => {
                            shuf_is_bad = false;
                        }

                        1 => {
                            rad::shit_shuffler::run();
                            shuf_is_bad = false;
                        }

                        2 => {
                            let length = {print!("length?: "); try_read!().unwrap_or(15)};
                            let threads = {print!("repeat how many times? (# of threads): "); try_read!().unwrap_or(10)};
                            println!("TX = thread number");

                            rad::shit_shuffler::run_multithread(length, threads);

                            shuf_is_bad = false;
                        }

                        3 => {
                            rad::egui::shit_shuffler::init();

                            shuf_is_bad = false;
                        }

                        _ => {

                        }

                    }

                    if !shuf_is_bad {
                        break;
                    }
                }

                is_bad = false;
            }

            3 => {

                println!("{:?}", rad::collatz::run({
                    print!("input for collatz?: ");
                    try_read!().unwrap_or(4)
                }, {
                    print!("print steps? true/false: ");
                    try_read!().unwrap_or(false)
                }));

                is_bad = false;
            }


            4=> {
                let mut vec: Vec<i128> = Vec::from([0,1]);
                let len = {
                    print!("how many numbers?: ");
                    try_read!().unwrap_or(10)
                };
                for i in 0..len {
                    vec.push(vec.get(i).unwrap() + vec.get(i+1).unwrap());
                }

                println!("{:?}", vec);
                
                is_bad = false;
            }

            5 => {
                rad::string_random::test({
                    print!("input: ");
                    try_read!("\n{}\n").unwrap()
                }, {
                    print!("how many?: ");
                    try_read!().unwrap_or(5)
                });

                is_bad = false;
            }


            _ => {
                term.clear_screen().expect("uh oh, terminal should be clear when selecting an invalid entry");
                
                println!("invalid");
                is_bad = true;
            }
        }
        if !(is_bad) {
            break;
        } else {
            term.clear_screen().expect("Uh oh, terminal should be cleared again when selecting an invalid entry.");
        }
    }
    main_timer.end_timer();
    println!("\n{} seconds overall", main_timer.get_elapsed().unwrap());

    

}

fn user_halt() {
    if std::env::consts::OS == "windows" {
        let _ = std::process::Command::new("cmd.exe").arg("/c").arg("pause").status().expect("Something is very fucking wrong. \nOS should be windows and \nthe 'pause' command should be in the command list");
    } else {
        print!("type anything and press enter to continue... \ni didn't implement user_halt() for other platforms so i hope this works for you");
        let mut str = String::new();
        match std::io::stdin().read_line(&mut str) {
            Ok(v) => {
                print!("usize: {}\n",v);
            }
            Err(e) => {
                panic!("fuck | {}", e);
            }
        }
    }
}

