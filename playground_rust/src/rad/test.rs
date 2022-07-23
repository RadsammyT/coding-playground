use std::*;

pub fn test() {
    let sleep = || -> i32 {
        thread::sleep(time::Duration::from_secs_f32(3.5));
        return 0;
    };

    for _ in 0..=2 {
        println!("delay of 3.5 seconds between slides!!!");
    }
    println!("Normal arrays \n");
    let arr: [i32; 5] = [1,2,3,4,5];
    println!("{:?}", arr);
    let mut test = 0;
    for i in arr {
        test += i;
    }
    println!("{test}");

    sleep();

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

    sleep();

    println!("pointers \n");
    let pointee: i32 = 69;
    let pointer = &pointee as *const i32;
    let null = ptr::null() as *const i32;

    println!("pointee addr: {:p}", &pointee);
    println!("pointer addr: {:p}", pointer);
    println!("null addr: {:p}", null);




}