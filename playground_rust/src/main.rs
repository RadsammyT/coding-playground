// use std::time::SystemTime;
use std::*;
mod test;
// use std::time::*;
// use ran::*;

fn main() {

println!("{}", std::i128::MAX);
let mut test:i128 = 1;
let mut step_mark:i128 = 0;
while test != std::i128::MAX{
    test *= 2;
    if test >= step_mark{
        println!("{}",test);
        step_mark += 1000000000;
    }
}
add();
}

// return int function
fn add() -> i32 {
   let mut ret = 1;
   for i in 0..10 {
         ret += i;
   }
    return ret;
}
