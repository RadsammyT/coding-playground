use std::io::Write;

use super::timer::Timer;

pub struct Returned {
    iteration: i32,
    even: i32,
    odd: i32,
}

impl std::fmt::Debug for Returned {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Returned").field("iter", &self.iteration).field("even", &self.even).field("odd", &self.odd).finish()
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

pub fn loop_run(range_lesser: i128, range_greater: i128,print_steps: bool,) {
    let mut iter_marker: i128 = 1000000;
    let iter_marker_copy = iter_marker.to_owned();
    let mut clock = Timer::default();
    clock.start_timer();
    for i in range_lesser..range_greater {
        run(i, false);
        if i >= iter_marker {
            if print_steps { print!("{}\r", i); } 
            std::io::stdout().flush().expect("Clogged.");
            iter_marker += iter_marker_copy;
        }
    }
    clock.stop_timer();
    println!("loop_run time: {}", clock.get_elapsed().unwrap());
}