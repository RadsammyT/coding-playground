use std::{vec, io::Error};

use eframe::glow::STENCIL_BACK_PASS_DEPTH_PASS;
use egui::Vec2;
use text_io::try_read;
use rustils;
struct Grid {
    arr: Vec<Vec<i32>>, 
    size: i32,
}

impl Grid {

    ///`init_grid()` is called when this function is called
    /// 
    /// this will return a 2d grid with the size of `len` squared
    fn new(len: i32) -> Grid {
        let mut test = Self { arr: vec![vec![]], size: len };
        test.init_grid();
        return test;
    }

    fn print_grid(&mut self) {
        
        for i in &self.arr {
            println!("{:?}", i);
        }
    }

    /// This must be called when a grid is created
    fn init_grid(&mut self) {
        self.arr = vec![vec![0; self.size as usize]; self.size as usize];
    }
    ///
    /// simple get method
    ///
    /// cant set values with this though
    /// 
    fn get(&mut self, x: i32, y: i32) -> i32 {
        self.arr[x as usize][y as usize]
    }

    fn set(&mut self, x: i32, y: i32, val: i32) {
        self.arr[x as usize][y as usize] = val;
    }

}

impl Default for Grid {
    fn default() -> Grid {
        Grid {
            arr: vec![vec![]],
            size: 3, // needs a rework: if any element is inserted into any vector in the grid this will not save you.
        }
    }
}


pub fn test_1() {
    let mut grid: Grid = Grid::new(3);

    grid.print_grid();
    println!("{}", who_goes_first());

    let select = player_input(&mut grid);
    println!("grid arr selected: {:?}, set to {:?}", select, grid.arr[select[0] as usize][select[1] as usize]);
    grid.set(select[0] , select[1] , 69); // because haha funni
    println!("grid arr now changed: {:?}, now set to {:?}", select, grid.arr[select[0] as usize][select[1] as usize]);

    grid.get(0, 0);

    grid.print_grid();
} 


pub fn game() {
    let mut grid = Grid::new(3);
    
    print!("X to go first? (yes/no)"); who_goes_first();
    let mut input: [i32; 2] = player_input(&mut grid);
    grid.set(input[0], input[1], 69);
}

fn player_input(grid: &mut Grid) -> [i32; 2]{
    let mut bad: bool = false;
    let mut res = [0,0];
    let mut inp_loop = |grid: &&mut Grid| -> i32 {
        loop {

            let x: String = try_read!().unwrap();
            let mut ret: i32 = 0;
            match rustils::parse::int::string_to_i32_res(x.to_owned()) { 
                Ok(n) => {
                    bad = false;
                    // println!("good input: got {}", rustils::parse::int::string_to_i32(x.to_owned()));
                    ret = n;
                },
                Err(_) => {
                    bad = true;
                    println!("Invalid");
                },
            };

            if !bad {
                if ret >= grid.size {
                    println!("bad number, out of bounds. input is {} (0-based btw) when max size is {} (1-based)", ret, grid.size);
                    // NOTE: DO NOT ADD 1 TO EITHER RET OR GRID SIZE OR ELSE USER WILL BE CONFUSED.
                    bad = true;
                } else if ret < 0 {
                    println!("bad number, {} is negative.", ret);
                    bad = true;
                } else {
                    println!("good input: got {}", rustils::parse::int::string_to_i32(x.to_owned()));
                    break ret;
                }
            }

        }
    };
    res[0] = inp_loop(&grid);
    res[1] = inp_loop(&grid);
    return res;
    
}

fn who_goes_first() -> bool {
    let mut bad_input: bool;
    let mut test: String;
    let mut output: bool;
    loop {
        bad_input = false;
        test = try_read!().unwrap();
        let x = &*test.to_lowercase();
        output = match x {
            "yes" => true,
            "no" => false,
            _ => {bad_input = true; false},
        };

        if !bad_input {
            break;
        }

    }
    output
}