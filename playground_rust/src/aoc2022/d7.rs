use super::input_parse::parse_to_vec_d7;

pub fn p1(fp: String) {
    let parsed = parse_to_vec_d7(fp);
    let mut sum: usize = 0;

    let mut list = parsed
        .iter()
        .map(|(_, size)| *size)
        .collect::<Vec<usize>>();
    list.sort();

    for size in list.iter() {
        if *size <= 100_000 {
            sum += size;
        }
    }

    println!("Answer P1: {}", sum);
}

pub fn p2(fp: String) {
    let parsed = parse_to_vec_d7(fp);
    let mut ans = 0;
    let total = parsed.get("/").unwrap();
    let req = 30_000_000 - (70_000_000 - total);

    let mut list = parsed
        .iter()
        .map(|(_, size)| *size)
        .collect::<Vec<usize>>();
    list.sort();

    for size in list.iter() {
        if *size >= req && ans == 0 {
            ans = *size;
        }
    }

    println!("Answer P2: {}", ans);
}