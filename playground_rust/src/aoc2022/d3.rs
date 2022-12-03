use super::input_parse;



pub fn p1(fp: String) {
    let input = input_parse::parse_to_vec_d3(fp);
    let mut common_letters: Vec<char> = vec![];
    let test = input.get(0).unwrap().get(0);
    for v in input {
        let i = v.get(0).unwrap();
        let j = v.get(1).unwrap();
        'out: for ic in i.chars() {
            for jc in j.chars() {
                if ic == jc {
                    println!("MATCH! {}, {}", ic, jc);
                    common_letters.push(jc);
                    break 'out;
                }
            }
        }
    }

    println!("{:?}", common_letters);
    let out = get_priority(common_letters);
    let mut sum = 0;
    for i in out {
        sum += i;
    }
    println!("priority is {}", sum);

}

fn get_priority(inp: Vec<char>) -> Vec<i32> {
    let alphabet = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', ];
    let mut output: Vec<i32> = vec![];
    for i in inp {
        for j in 0..alphabet.len() {
            if i.eq_ignore_ascii_case(&alphabet.get(j).unwrap()) {
                if i.is_uppercase() {
                    output.push(j as i32 + 1 + 26);
                } else {
                    output.push(j as i32 + 1);
                }
            }
        }
    }

    return output;
}