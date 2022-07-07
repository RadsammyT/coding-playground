

pub fn _line(prompt: String, keep_new_line: bool) -> String {
    println!("{}", prompt);
    let mut out: String = "".to_string();
    match std::io::stdin().read_line(&mut out) {
        Ok(_n) => {
            if !keep_new_line {
                out.pop();
            }
            return out;
        }
        Err(error) => {
            return error.to_string();
        }
            // WHY U FOOKEN CUNT
    }
}

/* 
 mismatched types
 expected struct `String`, found `()`

 AHAHAHHAHHAHAHHAHHAHAHHAAHHAHAHAHHAHAHHAHHAHAHHAHAHHAHAHHAHAHHAHAHHAHAHAHvhbtgjfynm hyujbk
 
*/