use std::fs::{self, File};

pub fn parse_to_vec(file: String) -> Vec<Vec<i32>> {

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
    let mut first_skip = false;
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

    let mut string_round: String = String::new();
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

pub fn parse_to_vec_d3(file: String) -> Vec<Vec<String>> {
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