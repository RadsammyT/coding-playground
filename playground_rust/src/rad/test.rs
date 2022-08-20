use std::{*, collections::{VecDeque, HashMap}};

use console::style;
use text_io::try_read;
use num2words::{self, Num2Words};
use open;

use crate::rad;



pub fn select() {
    let mut sel: i32;
    let mut is_bad: bool = true;

    println!("{} \n1: normal arrays \n2: vectors \n3: pointers \n4: vecdeques \n5: unions \n6: hashmaps (with num2words lib) \n7: console style \n8: Environment Constants + Arguments \n9: ShitShuffler, egui edition", style("  TEST SELECTION  ").underlined().yellow());
    while is_bad {
        print!("select a test entry: ");
        sel = try_read!().unwrap_or(-1);
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

                for i in 1..=6 {
                    vecdeq.push_back((*vecdeq.get(i - 1).unwrap()).pow(2));
                    vecdeq.push_front((*vecdeq.get(i + 1).unwrap()).pow(2));
                    println!("{:?}", vecdeq);


                }
                is_bad = false;

            }

            5 => {
                union Uni {
                    v1: u32,
                    v2: u16
                }
                let mut test = Uni {v1: 2000000000};
                test.v2 = u16::MAX;
                unsafe {
                    println!("Union: v1 as u32 (UNION SIZE), v2 as u16. \nv1 = 2000000000, v2 = u16::MAX");
                    println!("{:#32b} = {} \n{:#16b} = {}", test.v1, test.v1, test.v2, test.v2);
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

                println!("arguments passed: ");
                for i in std::env::args() {
                    println!("{}", i);
                }

                is_bad = false;
            }

            9 => {
                rad::egui::main();

                is_bad = false;
            }

            _ => {
                is_bad = true;
            }
        }
    }
}

