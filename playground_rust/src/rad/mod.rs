// because having all the rust source in the same directory is ugly to me

pub mod timer;
pub mod test;
pub mod printp;

/// ### (alternatively named bogoshuffler) Aka the worst shuffler in existence:
/// - shit shuffler randomizes an array of a set length between 0/1 to the lengths array
/// - if the array has no duplicate numbers, return the array
/// - otherwise, redo the randomization process again

pub mod shit_shuffler;
pub mod collatz;
pub mod string_random;
pub mod egui;
pub mod tictactoe;
