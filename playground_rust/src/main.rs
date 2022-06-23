// use std::time::SystemTime;
use std::{*, time::SystemTime};
mod rad;


fn main() {
    let mut timer = rad::timer::Timer{
        start: None,
        end: None,
    };

    let mut test: Option<i128> = Some(1);
    rad::timer::start_timer(&mut timer);

    println!("{}",test.unwrap());

    rad::timer::end_timer(&mut timer);
    println!("{}", rad::timer::get_elapsed(&mut timer).unwrap());

    rad::timer::test(true);
    println!("Test"); // if exit is true, this won't be called because the program is alreay exited
}

// return int function
fn add() -> i32 {
   let mut ret = 1;
   for i in 0..10 {
         ret += i;
   }
    return ret;
}

