pub fn test() {
    println!("Normal arrays");
    let arr: [i32; 5] = [1,2,3,4,5];
    let mut fuck = 0;
    for i in arr {
        fuck += i;
    }

    println!("{fuck}");
}