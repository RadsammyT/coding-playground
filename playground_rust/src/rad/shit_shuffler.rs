use text_io::*;
use rand::*;
use super::timer::Timer;
/// All elements of the vector are set to a random number from 0 to the length of the vector (Exclusive)
/// # Example
/// ```
/// vec = shuffle(&mut vec).to_vec();
/// ```
fn shuffle(v: &mut Vec<i32>) -> Vec<i32>{
    let test = v.len() as i32;

    for i in 0..v.len() as i32 {
        v[i as usize] = rand::thread_rng().gen_range(0..test);
    }

    return v.to_vec();
}

/// Checks if all elements in the vector are unique, and are never used again in the vector.
/// 
/// 
fn is_unique(v: &[i32]) -> bool {

    for i in 0..v.len() {
        for j in i+1..v.len() {
            if v[i] == v[j] && i != j {
                return false;
            }
        }
    }
    return true;
}

pub fn run() {

    let mut timer = Timer::new();

    println!("length? (15 or less is recommended)");
    let length: i32 = try_read!().unwrap_or(-1);
    println!("repeat for how many times? (for larger lengths, less is recommended)");
    let repeat_max: i32 = try_read!().unwrap_or(-1);
    let mut repeat: i32  = 0;
    let mut fail: u64 = 0;
    let mut fail_per: u64 = 0;
    let mut vec: Vec<i32> = vec![];
    for _ in 0..length {
        vec.push(0);
    }
    Timer::start_timer(&mut timer);
    
    while !(repeat >= repeat_max) {
        vec = shuffle(&mut vec).to_vec();
        if is_unique(&vec) {
            println!("{:?} {}", vec, &fail_per);
            fail += fail_per;
            fail_per = 0;
            repeat += 1;
        } else {
            fail_per += 1;
        }
    }
    Timer::end_timer(&mut timer);
    println!("{} total fails, {} seconds", fail, Timer::get_elapsed(&mut timer).unwrap());

}