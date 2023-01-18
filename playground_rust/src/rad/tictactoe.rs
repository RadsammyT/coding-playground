// u *may* puke.
use std::vec;
use eframe::glow::FALSE;
use text_io::try_read;
use rustils;
use console;

/// used for check_col/row/diag
/// and maybe some other things
const DEBUG_SWITCH: bool = false;

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

    fn print_grid(&mut self, substitute_for_shapes: bool) {
        // if substituting for shapes:
        //  0 = empty/"E" (or "()" if im daring)
        //  1 = X
        //  2 = O
        self.arr.iter().for_each(|x| {
            x.iter().for_each(|y| {
                match y {
                    
                    0 => {
                        if substitute_for_shapes { print!("E "); }
                        else { print!("{y} "); }
                    }
                    
                    1 => {
                        if substitute_for_shapes { print!("X "); }
                        else { print!("{y} "); }
                    }
                    
                    2 => {
                        if substitute_for_shapes { print!("O "); }
                        else { print!("{y} "); }
                    }

                    _ => {
                        panic!("val not 0, 1, or 2");
                    }
                }
            });
            println!();
        });
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
        self.arr[y as usize][x as usize]
    }

    fn set(&mut self, x: i32, y: i32, val: i32) {
        self.arr[y as usize][x as usize] = val;
    }

    fn set_double(&mut self, coord: [i32; 2], val: i32) {
        self.arr[coord[1] as usize][coord[0] as usize] = val;
    }

    fn get_double(&mut self, coord: [i32; 2]) -> i32{
        self.arr[coord[1] as usize][coord[0] as usize]
    }

    ///loops over every vector in the outter array to see if every vector's length is the same as the provided length in self.size
    /// 
    /// # Panics
    /// Panics if any size is not equal to self.size
    fn assert_size(&mut self) {
        for i in &self.arr {
            assert_eq!(i.len(), self.size as usize, "something was inserted/removed in the grid. this is bad!");
        }
    }

}

impl Default for Grid {
    fn default() -> Grid {
        Grid {
            arr: vec![vec![]],
            size: 3, // needs a rework (maybe): if any element is inserted into any vector in the grid this will not save you.
        }
    }
}


// pub fn test_1() {
//     let mut grid: Grid = Grid::new(3);

//     grid.print_grid(false);
//     println!("{}", who_goes_first());

//     let select = player_input(&mut grid, 1);
//     println!("grid arr selected: {:?}, set to {:?}", select, grid.arr[select[0] as usize][select[1] as usize]);
//     grid.set(select[0] , select[1] , 69); // because haha funni
//     println!("grid arr now changed: {:?}, now set to {:?}", select, grid.arr[select[0] as usize][select[1] as usize]);

//     grid.get(0, 0);

//     grid.print_grid(false);
// } 

pub fn game() {
    let term = console::Term::stdout();
    term.clear_screen().expect("uh oh");
    dbg!(DEBUG_SWITCH);

    let mut grid = Grid::new(3);
    let mut turn: i32;
    let mut select: [i32; 2];
    print!("X to go first? (yes/no): "); 
    if who_goes_first() {
        turn = 1;
    } else {
        turn = 2;
    }
    grid.print_grid(true);
    
    loop {
        println!("TURN: {:?}", {
            if turn == 1 {
                "X"
            } else {
                "O"
            }
            
        });
        select = player_input(&mut grid, turn);
        grid.set_double(select, turn);
        grid.print_grid(true);
        if check_winner(&mut grid) != 0 {
            break;
        }

        change_turn(&mut turn);
        term.clear_screen().expect("uh oh");
        
    }
    
    match check_winner(&mut grid) {
        0 => {
            grid.print_grid(false);

            panic!("nothing won?");
        }

        1 => {
            println!("X wins");
        }

        2 => {
            println!("O wins");
        }

        3 => {
            println!("Tie");
        }

        _ => {
            panic!("Invalid winner");
        }
    }

    grid.assert_size();
}

fn player_input(grid: &mut Grid, turn: i32) -> [i32; 2]{
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
                    // bad = true;
                } else if ret < 0 {
                    println!("bad number, {} is negative.", ret);
                    // bad = true;
                } else {
                    println!("good input: got {}", rustils::parse::int::string_to_i32(x.to_owned()));
                    break ret;
                }
            }

        }
    };
    loop {
        print!("X coordinate: ");
        res[0] = inp_loop(&grid);
        print!("Y coordinate: ");
        res[1] = inp_loop(&grid);
        if grid.get_double(res) != 0 {
            println!("Cell is taken! {} at {:?}", grid.get_double(res), res);
        } else {
            break;
        }
    }
    grid.set(res[0], res[1], turn);
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

fn change_turn(turn: &mut i32) {
    match turn {
        1 => {
            *turn = 2;
        }

        2 => {
            *turn = 1;
        }
        _ => {
            eprintln!("uh oh, {}", turn);
            panic!()
        }
    }
}

// here comes the most shittiest check-winner code youll ever see
// i32 return/cell guide:
// 0 = no winner/empty, 1 = X wins, 2 = O wins, 3 = tie.

fn check_row(grid: &mut Grid, row: i32) -> i32 {
    let mut res: String = String::new();
    let mut x_ex: String = String::new();
    let mut o_ex: String = String::new();

    for i in 0..grid.size {
        res.push_str(rustils::parse::string::ToStr::to_str(
            grid.get(i, row).to_owned().to_string()
        ));
        x_ex.push_str("1");
        o_ex.push_str("2");
    }
    
    
    if DEBUG_SWITCH { 
        println!("checkRow: {} compared to {} or {}", res, x_ex, o_ex);
     } 
    if res.eq(&x_ex) {
        return 1;
    } else if res.eq(&o_ex) {
        return 2;
    } else { 
        return 0;
    }
}
fn check_col(on: &mut Grid, col: i32) -> i32 { // funni
    let mut res: String = String::new();
    let mut x_ex: String = String::new();
    let mut o_ex: String = String::new();

    for i in 0..on.size {
        res.push_str(rustils::parse::string::ToStr::to_str(
            on.get(col, i).to_owned().to_string()
        ));
        x_ex.push_str("1");
        o_ex.push_str("2");
    }
    if DEBUG_SWITCH {
        println!("checkCol: {} compared to {} or {}", res, x_ex, o_ex);
    }
    if res.eq(&x_ex) {
        return 1;
    } else if res.eq(&o_ex) {
        return 2;
    } else { 
        return 0;
    }
}
///if reverse is true, diag is top left to bottom right.
/// 
///if reverse is false, diag is top right to bottom left.
fn check_diag(grid: &mut Grid, reverse: bool) -> i32 {
    let mut res: String = String::new();
    let mut x_ex: String = String::new();
    let mut o_ex: String = String::new();

    for i in 0..grid.size {
        x_ex.push_str("1");
        o_ex.push_str("2");
        if reverse {
            res.push_str(rustils::parse::string::ToStr::to_str(
                grid.get(i, i).to_owned().to_string()
            ));
        } else {
            res.push_str(rustils::parse::string::ToStr::to_str(
                grid.get(grid.size - i - 1, i).to_owned().to_string()
            ));
        }
    }
        if DEBUG_SWITCH {
            println!("checkDiag: {} compared to {} or {} as {}", res, x_ex, o_ex, reverse);
        }

        if res.eq(&x_ex) {
            return 1;
        } else if res.eq(&o_ex) {
            return 2;
        } else { 
            return 0;
        }
}

fn check_winner(grid: &mut Grid) -> i32 {
    
    for i in 0..grid.size {
        
        if DEBUG_SWITCH {
            println!("checking for {}", i);
        }
        
        match check_row(grid, i) {
            1 => return 1,
            2 => return 2,
            _ => {},
        }

        match check_col(grid, i) {
            1 => return 1,
            2 => return 2,
            _ => {},
        }

        match check_diag(grid, true) {
            1 => return 1,
            2 => return 2,
            _ => {},
        }

        match check_diag(grid, false) {
            1 => return 1,
            2 => return 2,
            _ => {},
        }

        
    }
    for i in 0..grid.size {
        for j in 0..grid.size {
            if grid.get(j, i) == 0 {
                return 0;
                // Despite all the checks for diagonals, horizontals, and verticals,
                // if none of the checks returned anything, and theres a 0 somewhere in the grid then return a 0, thus panic.
            }
        }
    }
    return 3;
}