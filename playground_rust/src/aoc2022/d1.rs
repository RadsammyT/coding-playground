use std::fs;

use super::input_parse;

pub fn p1(file_path: String) -> i32 {
    let inp = input_parse::parse_to_vec(file_path);
    let mut best = 0;
    let mut temp = 0;
    for i in inp {

        for j in i {
            temp += j;
        }
        if temp > best {
            best = temp;
        }
        temp = 0;
    }
    println!("RES: {}", best);

    return 0;
}

pub fn p2(file_path: String) -> i32 {

    let inp = input_parse::parse_to_vec(file_path);
    let mut temp = 0;
    let mut res: Vec<i32> = vec![];
    for i in inp {

        for j in i {
            temp += j;
        }
        res.push(temp);
        temp = 0;
    }
    res.sort(); // least to greatest
    let last = res.get(res.len() - 1 as usize).unwrap() + res.get(res.len() - 2 as usize).unwrap() + res.get(res.len() - 3 as usize).unwrap();
    println!("LAST = {}",last);
    return last;

}
