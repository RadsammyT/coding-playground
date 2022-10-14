struct Grid {
    arr: Vec<Vec<i32>>, 
    size: i32,
}

impl Grid {
    fn print_grid(&mut self) {
        
        for i in &self.arr {
            println!("{:?}", i);
        }
    }

}

impl Default for Grid {
    fn default() -> Grid {
        Grid {
            arr: vec![vec![]],
            size: 0,
        }
    }
}


pub fn test_1() {
    let mut grid: Grid = Grid::default();
    grid.arr = vec![vec![0,1,2], vec![3,4,5], vec![6,7,8]];

    grid.print_grid();
}
