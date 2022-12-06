use std::fs;
pub fn parse_to_vec_d1(file: String) -> Vec<Vec<i32>> {

    let parsed = fs::read_to_string(file).unwrap();
    /*
        assume following string:
        1000\n
        2000\n
        3000\n
        \n
        4000\n

        calculate the number string into int first:
            then if theres a \n, add the number to the buffer
            if theres another \n, add the buffer to the list. the added element is one elf.

            else, calculate new number string again
    */
    let mut num_buffer: String = String::new();
    let mut elf_buffer: Vec<i32> = vec![];
    let mut elf_list: Vec<Vec<i32>> = vec![vec![]];
    for i in parsed.chars() {
        // println!("({})", i);
            if  i == "\r".char_indices().next().unwrap().1 { //wtf?
            
                
                if !(num_buffer.chars().nth(0).unwrap_or("\r".chars().nth(0).unwrap()) == 
                    "\r".char_indices().next().unwrap().1) {
                    elf_buffer.push(rustils::parse::int::string_to_i32(num_buffer.to_owned()));
                }
                num_buffer.clear();
            } else if i == "\n".char_indices().next().unwrap().1 {
                
                elf_list.push(elf_buffer.to_owned());
                // dbg!(elf_list.to_owned());

                elf_buffer.clear();
            } else {
                num_buffer.push(i);
            }
    } 

    /*
        so, lets break down why we need to do the above again, kinda...

        the above code is bugged, and would put empty lines and separate 
        numbers in their own vector like so:
        -   [[], [100], [200], [300], [], [400], [500], []]
        but instead of fixing the vector above i decided to say fuck it and 
        took the bugged output and fix it in the following lines below.
        
        the only reason why this works is because of the empty vectors 
        SEPARATING the individual elf calories.
    */

    let mut finish_list: Vec<Vec<i32>> = vec![vec![]];
    let mut finish_buffer: Vec<i32> = vec![];
    for i in elf_list {
        if i.get(0) == None {
                finish_list.push(finish_buffer.to_owned());
                finish_buffer.clear();
        } else {
            finish_buffer.push(*i.get(0).unwrap());
        }
    }
    finish_list.remove(0); //remove empty vec
    println!("{:?}", finish_list);

    return finish_list;

}

pub fn parse_to_vec_d2(file: String) -> Vec<Vec<char>> {
    let parsed = fs::read_to_string(file).unwrap();

    let mut round_buffer: Vec<char> = vec![];
    let mut round_list: Vec<Vec<char>> = vec![vec![]];
    
    for i in parsed.chars() {
        /*
            "wait, theres carr returns? why?"
            windows (my OS) has two escape characters that act as a new line.
            CR, aka \r, returns the cursor to the first column.
            LF, aka \n, pushes the cursor down by one row.
            either of these symbols can be written in my code to indicate a new line, but I decided on \r as the marker for a new line.
        */
        if "\r".char_indices().next().unwrap().1 == i {
            round_list.push(round_buffer.to_owned());
            round_buffer.clear();
        } else if " ".char_indices().next().unwrap().1 == i {
        } else {
            if i != "\n".char_indices().next().unwrap().1 {
                round_buffer.push(i);
                
            }
        }


    }

    println!("{:?}", round_list);
    round_list.remove(0); // because its empty
    return round_list;

}

pub fn parse_to_vec_d3_p1(file: String) -> Vec<Vec<String>> {
    /*
        heres what im seeing:
            assume input being:
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw

        get each halve of the line to its own vector, then push that vec into the outter vec.
        all input lines have even amount of characters.
    */

    let parsed = fs::read_to_string(file).unwrap();
    // let mut sack_halves_buffer: Vec<String> = vec![];
    let mut sack_halves: Vec<Vec<String>> = vec![vec![]];

    for i in parsed.lines() {
        let (a,b) = i.split_at(i.len() / 2); //split works
        let res = vec![a.to_string(), b.to_string()];
        sack_halves.push(res);
    }

    sack_halves.remove(0);
    println!("{:?}", sack_halves);

    return sack_halves;
}

pub fn parse_to_vec_d3_p2(file: String) -> Vec<Vec<String>> {
    let parsed = fs::read_to_string(file).unwrap();
    // let mut sack_halves_buffer: Vec<String> = vec![];
    let mut sack_group: Vec<Vec<String>> = vec![];
    let mut sack_buffer: Vec<String> = vec![];
    for i in parsed.lines() {
        sack_buffer.push(i.to_string());
        if sack_buffer.len() == 3 {
            sack_group.push(sack_buffer.to_owned());
            sack_buffer.clear();
        }
    }

    println!("{:?}", sack_group);

    return sack_group;
}

pub fn parse_to_vec_d4(file:String) -> Vec<Vec<i32>> {
    let parsed = fs::read_to_string(file).unwrap();
    let mut num_buffer: String = String::new();
    let mut pair_buffer: Vec<i32> = vec![];
    let mut pair_list: Vec<Vec<i32>> = vec![];

    for i in parsed.chars() {
        if i == '-' {
            pair_buffer.push(rustils::parse::int::string_to_i32(num_buffer.to_owned()));
            num_buffer.clear();
        } else if i == ',' { 
            pair_buffer.push(rustils::parse::int::string_to_i32(num_buffer.to_owned()));
            num_buffer.clear();

            pair_list.push(pair_buffer.to_owned());
            pair_buffer.clear();
        } else if i == '\n' {
            pair_buffer.push(rustils::parse::int::string_to_i32(num_buffer.to_owned()));
            num_buffer.clear();

            pair_list.push(pair_buffer.to_owned());
            pair_buffer.clear();
        } else  {
            if i != '\r' {
                num_buffer.push(i);
            }
        }

    }

    return pair_list;
}

pub fn parse_to_vec_d5(file: String) -> (Vec<Vec<char>>, Vec<Vec<i32>>) {
    let parsed = fs::read_to_string(file).unwrap();
    
    // stack

    /*
    every vector is a row of the crates
    i plan to basically turn it 90 degrees to the right
    so i can handle them better
     */
    let mut parsed_2d: Vec<String> = vec![];
    let mut buffer: String = String::new();

    for i in parsed.chars() {
        if i == '\n' {
            if buffer.is_empty() {
                break;
            }
            parsed_2d.push(buffer.to_owned());
            buffer.clear();
        } else {
            if i != '\r' {
                buffer.push(i);
            }
        }



    }
    // dbg!(&parsed_2d.get(0).unwrap().chars().nth(1).unwrap());
    // dbg!(&parsed_2d.get(1).unwrap().chars().nth(1).unwrap());
    // dbg!(&parsed_2d.get(2).unwrap().chars().nth(1).unwrap());
    // dbg!(&parsed_2d.get(3).unwrap().chars().nth(1).unwrap());
    
    let mut crates_final: Vec<Vec<char>> = vec![];
    let mut crates_buffer: Vec<char> = vec![];
    //stores the index the number and crates are on.
    let mut crates_string_indexes: Vec<i32> = vec![];
    
    for (ind, ele) in parsed_2d.get(parsed_2d.len() - 1).unwrap().chars().enumerate() {
        if ele != ' ' {
            crates_string_indexes.push(ind as i32);
        }
    }
    parsed_2d.reverse();
    // dbg!(&parsed_2d);
    for i in crates_string_indexes { // col
        for j in 1..parsed_2d.len() { // row
            
            
            // dbg!(&crates_buffer);
            if parsed_2d.get(j).unwrap().chars().nth(i as usize).unwrap() != ' ' {
                crates_buffer.push(parsed_2d.get(j).unwrap().chars().nth(*&i as usize).unwrap());
                if j == parsed_2d.len() - 1 {
                    crates_final.push(crates_buffer.to_owned());
                    crates_buffer.clear();
                    // dbg!(&crates_final);
                }

            } else {
                crates_final.push(crates_buffer.to_owned());
                crates_buffer.clear();
            }
        }
    }
    for _ in 0..10 {
        for i in 0..crates_final.len() {
            if crates_final.get(i).unwrap_or(&['a'].to_vec()).is_empty() {
                dbg!("removed");
                crates_final.remove(i);
            }
        }
        // dbg!("new loop");
    }
    // dbg!(&crates_final);

    // move instructions

    //convert parsed string into a vector of primitive strings
    // that have move instructions on them
    let mut test: Vec<&str> = parsed.split("\r\n").collect();
    for i in 0..test.len() - parsed_2d.len() {
        if test.get(0).unwrap().chars().nth(0).unwrap_or('a') != 'm' {
            test.remove(0);
        } else {
            break;
        }
    }
    // dbg!(&test);

    let mut num_buffer = String::new();
    let mut move_buffer: Vec<i32> = vec![];
    let mut move_list: Vec<Vec<i32>> = vec![vec![]];
    

    // convert that into a vector of move instructions (aka a vector)

    for i in test {
        // dbg!(&i);
        for j in i.chars() {
            if j.is_numeric() {
                num_buffer.push(j);
            }
            if j == ' ' {
                if !num_buffer.is_empty() { 
                    move_buffer.push(rustils::parse::int::string_to_i32(num_buffer.to_owned()));
                    num_buffer.clear();
                }
            }
            if move_buffer.len() >= 3 {
                move_list.push(move_buffer.to_owned());
                move_buffer.clear();
            }
        }
        // dbg!(&move_buffer);
    }
    
    move_list.remove(0);
    // dbg!(&move_list);
    return (crates_final, move_list);

}

pub fn parse_to_vec_d6(fs: String) -> Vec<char> {
    let parsed = fs::read_to_string(fs).unwrap();
    let ret: Vec<char> = parsed.chars().collect();
    ret
}