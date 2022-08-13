use std::{*, collections::VecDeque};

use text_io::try_read;



pub fn select() {
    let mut sel: i32;
    let mut is_bad: bool = true;

    println!("--TEST SELECTION-- \n1: normal arrays \n2: vectors \n3: pointers \n4: vecdeques \n5: unions \n");
    while is_bad {
        print!("select a test entry: ");
        sel = try_read!().unwrap_or(-1);
        match sel {
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

            }

            _ => {
                is_bad = true;
            }
        }
    }
}

