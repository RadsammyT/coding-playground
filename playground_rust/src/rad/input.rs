

pub fn line(prompt: String) -> String {
    println!("{}", prompt);
    let mut out: String = String::new();
    match std::io::stdin().read_line(&mut out) {
        Ok(_n) => {
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