mod test;
use std::time::{SystemTime, UNIX_EPOCH};

use ran::*;
fn main() {
    // seed number generator using system time in nanoseconds
    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos();
    
    println!("{}", add());
    println!("{}", test::test(1));
    // generate a random number from src
    println!("{:?}", generators::ranvvu8(1,1));
    

}

// return int function
fn add() -> i32 {
   let mut ret = 1;
   for i in 0..10 {
         ret += i;
   }
    return ret;
}
