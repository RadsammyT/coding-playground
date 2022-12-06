use std::collections::VecDeque;

use super::input_parse::parse_to_vec_d5;

pub fn p1(fp: String) {
    let (mut stack, list) = parse_to_vec_d5(fp);
    let mut crane: char;
    dbg!(&list.len());
    dbg!(&list);
    for i in list {
        // dbg!(&i);
        for _ in 0..*i.get(0).unwrap() {
            let a = stack.get_mut((*i.get(1).unwrap() - 1) as usize).unwrap(); // from stack
            // let b = stack.get_mut(*i.get(2).unwrap() as usize).unwrap(); // to stack

            //crane set to top crate of stack a
            crane = *a.get(a.len() - 1).unwrap(); 

            // pop top crate of stack a
            a.pop().unwrap(); 

            //crane drops crate to stack b
            stack.get_mut((*i.get(2).unwrap() - 1) as usize).unwrap().push(crane);

            // crane should clear?
            crane = ' ';

            
        
        }
    }

    let mut final_answer: String = String::new();
    dbg!(&stack);
    for i in stack {
        if i.len() != 0 {
            final_answer.push(*i.get(i.len() - 1).unwrap());
        }
    }

    println!("Answer P1: {}", final_answer);

}

pub fn p2(fp: String) {
    let (mut stack, list) = parse_to_vec_d5(fp);
    let mut crane: Vec<char> = vec![];
    dbg!(&list.len());
    dbg!(&list);
    for i in list {
        // dbg!(&i);
        // for _ in 0..*i.get(0).unwrap() {
        //     let a = stack.get_mut((*i.get(1).unwrap() - 1) as usize).unwrap(); // from stack
        //     // let b = stack.get_mut(*i.get(2).unwrap() as usize).unwrap(); // to stack

        //     //crane set to top crate of stack a
        //     crane = *a.get(a.len() - 1).unwrap(); 

        //     // pop top crate of stack a
        //     a.pop().unwrap(); 

        //     //crane drops crate to stack b
        //     stack.get_mut((*i.get(2).unwrap() - 1) as usize).unwrap().push(crane);

        //     // crane should clear?
        //     crane = ' ';

            
        
        // }

        // to load the crane
        for _ in 0..*i.get(0).unwrap() {
            let a = stack.get_mut((*i.get(1).unwrap() - 1) as usize).unwrap();

            crane.push(*a.get(a.len() - 1).unwrap());
            
            a.pop().unwrap();
        }

        // to unload the crane
        for _ in 0..*i.get(0).unwrap() {
            stack.get_mut((*i.get(2).unwrap() - 1) as usize).unwrap().push(*crane.get(crane.len() - 1).unwrap());

            crane.pop().unwrap();

        }
    }

    let mut final_answer: String = String::new();
    dbg!(&stack);
    for i in stack {
        if i.len() != 0 {
            final_answer.push(*i.get(i.len() - 1).unwrap());
        }
    }

    println!("Answer P1: {}", final_answer);

}