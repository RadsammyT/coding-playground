use super::input_parse::parse_to_vec_d4;

pub fn p1(f: String) {
    let input = parse_to_vec_d4(f);
    println!("{:?}", input);

    let mut odd_pair: Vec<i32>;
    let mut even_pair: Vec<i32>;
    let mut count = 0;
    for (ind, _) in input.iter().enumerate() {
        if ind % 2 == 1 {
            odd_pair = input.get(ind - 1).unwrap().to_vec();
            even_pair = input.get(ind).unwrap().to_vec();
            // println!("Comparing {:?} against {:?}", odd_pair, even_pair);
            // if odd pair values are less and greater than the evens
            if odd_pair.get(0).unwrap() <= even_pair.get(0).unwrap() {
                if odd_pair.get(1).unwrap() >= even_pair.get(1).unwrap() {
                    count += 1;
                    continue;
                }
            }

            if even_pair.get(0).unwrap() <= odd_pair.get(0).unwrap() {
                if even_pair.get(1).unwrap() >= odd_pair.get(1).unwrap() {
                    count += 1;
                    continue;
                }
            }
        }
    }

    println!("Count: {}", count);
}

pub fn p2(f: String) {
    let input = parse_to_vec_d4(f);
    println!("{:?}", input);

    let mut odd_pair: Vec<i32>;
    let mut even_pair: Vec<i32>;
    let mut count = 0;
    for (ind, _) in input.iter().enumerate() {
        if ind % 2 == 1 {
            odd_pair = input.get(ind - 1).unwrap().to_vec();
            even_pair = input.get(ind).unwrap().to_vec();

            if p2_overlap(odd_pair, even_pair) {
                count += 1;
                continue;
            }
        }
    }

    println!("Count: {}", count);
}

fn p2_overlap(one: Vec<i32>, two: Vec<i32>) -> bool {
    let first: Vec<i32> = vec_between(one);
    let second: Vec<i32> = vec_between(two);
    for i in first {
        for j in &second { // i have to borrow this vector for some reason or error
            if i == *j { // ???????????????????
                return true;
            }
        }
    }
    return false;
}

fn vec_between(inp: Vec<i32>) -> Vec<i32> {
    let mut out: Vec<i32> = vec![]; 
    for i in inp.get(0).unwrap().to_owned()..=inp.get(1).unwrap().to_owned() {
        out.push(i);
    }

    return out;
}