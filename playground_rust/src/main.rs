// use std::time::SystemTime;
use std::{*, time::SystemTime};
mod test;
mod timer;


fn main() {
    let mut timer = timer::Timer{
        start: None,
        end: None,
    };

    let mut test: i128 = 1;
    timer::start_timer(&mut timer);

    println!("{}", i128::MAX);
    while test < 10000 {
        test += 1;
        println!("{}", test);
    }

    timer::end_timer(&mut timer);
    println!("{}", timer::get_elapsed(&mut timer).unwrap());

    timer::test(true);
    println!("Test");
}

// return int function
fn add() -> i32 {
   let mut ret = 1;
   for i in 0..10 {
         ret += i;
   }
    return ret;
}

