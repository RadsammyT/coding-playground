use text_io::*;
use rand::*;
pub fn shuffle(mut v: Vec<i128>) -> Vec<i128> {
    let test = v.len() as i128;

    for i in 0..test {
        v[i as usize] = rand::thread_rng().gen_range(0..test);
    }

    return v;
}

pub fn run() {

    let mut vec: Vec<i128> = vec![];
    for i in 0..10 {
        vec.push(i as i128);
    }
    println!("{:?}", vec);
    vec = shuffle(vec);
    println!("{:?}", vec);
    
    
    
    // println!("length? ");
    // let length: i64 = try_read!().unwrap_or(-1);
    // println!("repeat? ");
    // let repeat: i64 = try_read!().unwrap_or(-1);

    // let vec: Vec<i128> = Vec::with_capacity(length.try_into().unwrap());


    // let mut fail: u64 = 0;


}