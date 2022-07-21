use rand::*;

pub fn run(mut input: String) -> String {
    let min = rand::thread_rng().gen_range(0..input.len());
    let len = input.len();
    return input.get_mut(min..rand::thread_rng().gen_range(min..=len)).unwrap().to_string();
}

pub fn test(mut input: &String) {
    for _ in 0..=100 {
        println!("{}", run((&input).to_string()));
    }
}