pub struct Returned {
    iteration: i32,
    even: i32,
    odd: i32,
}

impl std::fmt::Debug for Returned {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Returned").field("iteration", &self.iteration).field("even", &self.even).field("odd", &self.odd).finish()
    }
}

pub fn run(mut input: i128, print_steps: bool) -> Returned {

    let mut ret = Returned {
        iteration: 0, even: 0, odd: 0
    };
    /*
    ret[0] = iteration
    ret[1] = odd
    ret[2] = even
    */
    if print_steps { println!("{input}"); }
    loop { // oh cool. 
        if input % 2 == 1 {
            input = (3*input) + 1;
            ret.iteration += 1; 
            ret.odd += 1;
            if print_steps {
                println!("{input}");
            }
        }

        if input % 2 == 0 {
            input /= 2;
            ret.iteration += 1;
            ret.even += 1;
            if print_steps  {
                println!("{input}");
            }
        }
        if input == 1 {break;}
    } 
    ret // return ret

}