/*
    NOTE TO SELF:
        https://cheat.sh/rust/
        USE& LINK WHEN STUCK ON SHIT
*/


// use std::time::SystemTime;
use std::{*, fs::File, io::{Read, Write}};
use rad::timer::Timer;
use text_io::*;
use console::*;


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
    let term = console::Term::stdout();
    term.clear_screen().expect("uh oh, terminal should be clear on start");
    while is_bad {
        println!("{} \n1: shitshuffler \n2: test select \n3: collatz \n4: quick fibb \n5: random slices of string \n6: closure bullshittery \n7: shitshuffler, multithreading edition \n8: timer epoch \n9: file reading (change path in main.rs, fn select()) \n{}", style("welcome").underlined().fg(Color::Blue), style("0 TO EXIT").bg(Color::Red).blink());
        print!("Select an entry: ");
        sel = try_read!().unwrap_or(-1);
        main_timer.start_timer();
        match sel {
            0 => {
                is_bad = false;
            }
            1 => {
                
                rad::shit_shuffler::run();
                is_bad = false;
            }
            2 => {
                term.clear_screen().expect("uh oh, terminal should be clear when selecting entry 2.");
                rad::test::select();
                is_bad = false;
            }
            /*
                func({println!("input"); read!()})
                what the fuck
            */
            3 => { // cursed
                let inp: i128 = {print!("input: "); try_read!().unwrap()};
                let inp2: bool = {println!("print steps? (true/false): "); try_read!().unwrap()};
                let mut time = Timer::new();
                time.start_timer();
                let res = rad::collatz::run(inp,
                inp2);
                time.end_timer();

                println!("{:?}, {}", res, time.get_elapsed().unwrap());
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
                std::io::stdout().flush().unwrap();
                let range: i32 = {print!("range: ");
                                try_read!().unwrap_or(100)};
                std::io::stdout().flush().unwrap();
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

                let warn: bool;
                println!("\n\n\n{} \nThis uses a lot of your CPU depending on the number of threads deployed. \n{} (true/false)", style("WARNING:").bold().red(), style("Continue?").underlined().yellow());
                warn = try_read!().unwrap_or(false);

                if warn {
                    let mut timer = Timer::new();
                    let length = {print!("length?: "); try_read!().unwrap_or(15)};
                    let thread_num = {print!("repeat how many times? (# of threads): "); try_read!().unwrap_or(10)};
                    println!("TX = thread number");
                    let mut threads = vec![];
                    timer.start_timer();
                    for i in 0..thread_num {
                        threads.push(thread::spawn(move || {
                            let temp = rad::shit_shuffler::run_singular(length);
                            println!("T{}) {:?}, {}", i + 1, temp.ret_1, temp.ret_2 );
                        }));
                    }

                    for i in threads {
                        let _res = i.join();
                    }
                    timer.stop_timer();

                    println!("shitshuffler time: {}", timer.get_elapsed().unwrap());
                    is_bad = false;
                }
            }

            8 => {
                let mut timer = Timer::new();
                timer.start_timer();
                println!("{:?}", timer.get_epoch(true).unwrap());

                timer.end_timer();
                println!("{}", timer.get_elapsed().unwrap());

                is_bad = false;
            }

            9 => {
                let mut file = match File::open("E:/CODING WORKSPACE/coding-playground/test.txt") { // change this path for your machine
                    Ok(s) => s,
                    Err(e) => panic!("{}", e),
                };
                let mut read = String::new();

                let _res = match file.read_to_string(&mut read) {
                    Ok(r) => r,
                    Err(e) => panic!("{}", e),
                };

                println!("{}", read);

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
    user_halt();
}

fn user_halt() {
    if std::env::consts::OS == "windows" {
        let _ = std::process::Command::new("cmd.exe").arg("/c").arg("pause").status().expect("Something is very fucking wrong. \nOS should be windows and \nthe 'pause' command should be in the command list");
    } else {
        print!("type anything and press enter to continue... \ni didn't implement user_halt() for other platforms well so i hope this works for you");
        let _ = try_read!().unwrap_or(-1);
    }
}

