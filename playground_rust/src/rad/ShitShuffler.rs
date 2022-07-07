use text_io::*;
use rand::*;
fn shuffle(v: &mut Vec<i32>) -> Vec<i32>{
    let test = v.len() as i32;

    for i in 0..v.len() as i32 {
        v[i as usize] = rand::thread_rng().gen_range(0..test);
    }

    return v.to_vec();
}

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

    println!("length? ");
    let length: i32 = try_read!().unwrap_or(-1);
    println!("repeat? ");
    let repeat_max: i32 = try_read!().unwrap_or(-1);
    let mut repeat: i32  = 0;
    let mut fail: u64 = 0;

    let mut vec: Vec<i32> = vec![];
    for _ in 0..length {
        vec.push(0);
    }

    while !(repeat >= repeat_max) {
        vec = shuffle(&mut vec).to_vec();
        if is_unique(&vec) {
            println!("{:?} {}", vec, &fail);
            repeat += 1;
        } else {
            fail += 1;
        }
    }
    

}