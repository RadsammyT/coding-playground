use std::*;

pub fn test() {
    println!("Normal arrays");
    let arr: [i32; 5] = [1,2,3,4,5];
    println!("{:?}", arr);
    let mut test = 0;
    for i in arr {
        test += i;
    }
    println!("{test}");

    thread::sleep(time::Duration::from_secs_f32(3.5));

    println!("Vector");
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




}