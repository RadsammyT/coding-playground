pub fn test(mut a: i64) -> i64 {
    let b = a;
    for i in 0..b {
        a += i;
    }

    return a;
}