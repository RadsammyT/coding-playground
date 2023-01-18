use std::{*, collections::{VecDeque, HashMap}, fs::File};

use console::style;
use rustils::string::StringUtils;
use text_io::{try_read, read};
use num2words::{self, Num2Words};
use open;
use crate::rad::{self, timer::Timer};

pub fn select(timer: &mut Timer) {
    let mut sel: i32;
    let mut is_bad: bool = true;
    let term = console::Term::stdout();
    term.clear_screen().expect("bruh");
    println!("{} \n1: normal arrays  2: vectors \n3: pointers 4: vecdeques \n5: unions 6: hashmaps (with num2words lib) \n7: console style 8: Environment Constants + Arguments \n9: timer epoch 10: file reading (change path in /rad/test.rs) \n11: print one char at a time 12: test egui \n13:2d grid, tictactoe? 14: enums \n15: collatz on loop", style("  TEST SELECTION  ").underlined().yellow());
    while is_bad {
        print!("select a test entry: ");
        sel = try_read!().unwrap_or(-1);
        timer.start_timer();
        match sel {
            0 => {
                match open::that("https://www.youtube.com/watch?v=GGHE7IR_vyM") {
                    Ok(_) => {},
                    Err(x) => panic!("{} | ????????????", x),
                }
                is_bad = false;
            }
            1 => {
                println!("Normal arrays \n");
                let arr: [i32; 5] = [1,2,3,4,5];
                println!("{:?}", arr);
                let mut test = 0;
                for i in arr {
                    test += i;
                }
                println!("{test}");

                is_bad = false;
            }
            
            2 => {
                let mut test: i32;
                println!("Vector \n");
                let mut vec: Vec<i32> = vec![];
                test = 0;
                for i in 1..11 {
                    vec.push(i);
                }
                println!("{:?}", vec);
                for i in vec {
                    test += i;
                }
                println!("{}", test);


                is_bad = false;

            }

            3 => {
                println!("pointers \n");
                let pointee: i32 = 69;
                let pointer = &pointee as *const i32;
                let null = ptr::null() as *const i32;
                        
                println!("pointee addr: {:p}", &pointee);
                println!("pointer addr: {:p}", pointer);
                println!("null addr: {:p}", null);

                is_bad = false;

            }

            4 => {
                println!("VecDeque \n");

                let mut vecdeq: VecDeque<i128> = VecDeque::from([2]);

                for _ in 1..=6 {
                    vecdeq.push_back(-1);
                    vecdeq.push_front(1);
                    println!("{:?}", vecdeq);


                }
                dbg!(vecdeq.iter().max().unwrap_or(&0));
                is_bad = false;

            }

            5 => {
                union Uni32And16 {
                    v1: u32,
                    v2: u16
                }

                union UniSignedAndUnsigned {
                    v1: i16,
                    v2: u16,
                }

                let mut dif_bits = Uni32And16 {v1: 2000000000};
                dif_bits.v2 = 0b1010101010101010;

                let dif_signs = UniSignedAndUnsigned {
                    v2: 0b1111111111111111
                }; 
                unsafe {
                    
                    // unsigned vars with different bits
                    println!("Union: v1 as u32 (UNION SIZE), v2 as u16. \nv1 = 2000000000, v2 = u16::MAX");
                    println!("{:#32b} = {} \n{:#16b} = {}", dif_bits.v1, dif_bits.v1, dif_bits.v2, dif_bits.v2);

                    // two 32bit integers, differing from signed or unsigned
                    println!("Union: v1 as i16, v2 as u16");
                    println!("{:#32b} = {} \n{:#16b} = {}", dif_signs.v1, dif_signs.v1, dif_signs.v2, dif_signs.v2);
                }

                is_bad = false;
            }

            6 => {
                let mut test_map: HashMap<i32, String> = HashMap::new();
                for i in 0..=10 {
                    test_map.insert(i, Num2Words::new(i as i64).currency(num2words::Currency::GBP).to_words().unwrap());
                    // not casting i as an i64 will throw an error
                    // and why it occurs is so dumb i have to cast it
                    /*
                    the trait bound `num2words::number::Number: From<i32>` is not satisfied
                    the following other types implement trait `Into<T>`:
                    f64
                    i64
                    required because of the requirements on the impl of `Into<num2words::number::Number>` for `i32`
                    */
                    // just lib author things i guess
                }
                dbg!(test_map);

                is_bad = false;
            }

            7 => {
                let _ = console::Term::stdout();
                println!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
                style("COOL TERMINAL LIBRARY RED").red(),
                style("COOL TERMINAL LIBRARY YELLOW").yellow(),
                style("COOL TERMINAL LIBRARY GREEN").green(),
                style("COOL TERMINAL LIBRARY MEGENTA").magenta(),
                style("COOL TERMINAL LIBRARY BLINK, UNDERLINE, BOLD, ITALTIC").blink().underlined().bold().italic(),
                style("COOL TERMINAL LIBRARY BRIGHT").bright(),
                style("COOL TERMINAL LIBRARY DIM").dim() );

                is_bad = false;
            }

            8 => {
                print!("Architecture: {} \n", std::env::consts::ARCH);
                print!("DLL Ext: {} \n", std::env::consts::DLL_EXTENSION);
                print!("DLL Pre: {} \n", std::env::consts::DLL_PREFIX);
                print!("DLL Suf: {} \n", std::env::consts::DLL_SUFFIX);
                print!("EXE Ext: {} \n", std::env::consts::EXE_EXTENSION);
                print!("EXE Suf: {} \n", std::env::consts::EXE_SUFFIX);
                print!("OS Family: {} \n", std::env::consts::FAMILY);
                print!("OS: {} \n\n", std::env::consts::OS);
                std::env::vars_os().into_iter().for_each(|(x,y)| {
                    println!("{:?}: {:?}", x, y);
                });
                println!("arguments passed: ");
                for i in std::env::args() {
                    println!("{}", i);
                }

                is_bad = false;
            }

            9 => {
                let mut timer = Timer::new();
                timer.start_timer();
                println!("{:?}", timer.get_epoch(true).unwrap());

                timer.end_timer();
                println!("{}", timer.get_elapsed().unwrap());

                is_bad = false;
            }
            
            10 => {
                // let inp = try_read!().unwrap_or({
                //     println!("Error! Defaulting to hardcoded path...");
                //     "E:/CODING WORKSPACE/coding-playground/test.txt".to_string()
                // }).to_str();
                // let  test = File::try_from(inp);
                let mut file = match File::open("E:/CODING WORKSPACE/coding-playground/test.txt") { // change this path for your machine
                    Ok(s) => s,
                    Err(e) => panic!("{} | Is the file path correct for your machine? Check ->", e),
                };
                let mut read = String::new();

                let _res = match io::Read::read_to_string(&mut file, &mut read) {
                    Ok(r) => r,
                    Err(e) => panic!("{}", e),
                };

                println!("{}", read);

                is_bad = false;
            }
            11 => {
                rad::printp::printp("this is a big big test message WOOHOOO\nAlso this is another message in a new line\n", time::Duration::from_millis(100));

                is_bad = false;
            }

            12 => {
                rad::egui::egui_test::init();

                is_bad = false;
            }

            13 => {
                rad::tictactoe::game();

                is_bad = false;
            }

            14 => {
                #[derive(Debug)]
                enum Test {
                    Num(i32),
                    Str(String),
                }



                let pee: Vec<Test> = vec![Test::Num(123), Test::Str("CBT".to_string())];
                print!("{:?}", pee);

                is_bad = false;
            }

            15 => {
                let less: i128 = {
                    print!("range_lesser (int): ");
                    read!()
                };

                let great: i128 = {
                    print!("range_greater (int): ");
                    read!()
                };

                let print: bool = {
                    print!("print_steps (bool): ");
                    read!()
                };
                rad::collatz::loop_run(less,great,print);

                is_bad = false;
            }

            16 => {
                let string: &'static str = "Test message";
                let test = string.split(" ").map(|x| {
                    if x.contains_any_char(&['a']) {
                        return x
                    } else {
                        ""
                    }
                }).collect::<Vec<_>>();
                test.iter().for_each(|f| {
                    println!("{}", f);
                });
                is_bad = false;
            }

            _ => {
                is_bad = true;
            }
        }
    }
}

