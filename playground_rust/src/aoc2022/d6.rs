use super::input_parse::parse_to_vec_d6;

pub fn p1(fp: String, len: Option<i32>) {
    let parsed = parse_to_vec_d6(fp);
    let l = len.unwrap_or(4);
    let mut char_buffer: Vec<char> = vec![];
    let mut unique;


    'main: for i in 0..=parsed.len() - l as usize {
        unique = true;
        for j in i..i+l as usize {
            char_buffer.push(parsed.get(j).unwrap().to_owned());
        }

        'out: for (ji, jc) in char_buffer.iter().enumerate() {
            for (ki, kc) in char_buffer.iter().enumerate() {
                if jc == kc && ji != ki {
                    unique = false;
                    break 'out;
                }
            }
        }

        // dbg!(&char_buffer);
        if unique {
            println!("Unique at {}", (i as i32) +l);
            break 'main;
        }
        
        char_buffer.clear();
    }
    
}

pub fn p2(fp: String) {
    p1(fp, Some(14));
}