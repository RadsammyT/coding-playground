pub fn run(mut input: i128, print_steps: bool) -> [i32; 3] {
    
    let mut arr: [i32;3] = [0,0,0];
    /*
    arr[0] = iter
    arr[1] = odd
    arr[2] = even
    */
    if print_steps { println!("{input}"); }
    loop { // oh cool. 
        if input % 2 == 1 {
            input = (3*input) + 1;
            arr[0] += 1; 
            arr[1] += 1;
            if print_steps {
                println!("{input}");
            }
        }

        if input % 2 == 0 {
            input /= 2;
            arr[0] += 1;
            arr[2] += 1;
            if print_steps  {
                println!("{input}");
            }
        }
        if input == 1 {break;}
    } 
    arr // return arr

}