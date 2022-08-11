use std::fs;
use std::path::Path;
use clap::{Arg, Command};
use crate::puzzle::Puzzle;
use crate::solver::solve_puzzle;

pub mod solver;
pub mod puzzle;
pub mod utils;

pub fn main(){
    let args = Command::new("sudoku_solver_rs")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Solve sudoku grids.")
        .arg(
        Arg::new("file")
            .long("file")
            .short('f')
            .takes_value(true)
        )
        .get_matches();
    let file_path = args.value_of("file").unwrap();
    let file_content = fs::read_to_string(file_path).unwrap();
    let rows = file_content.split("\n").collect::<Vec<&str>>();
    let mut puzzle = Vec::new();
    for row in rows{
        let columns = row.split(",").collect::<Vec<&str>>();
        let mut _row = Vec::new();
        for column in columns{
            let number = column.trim().parse::<i32>().unwrap();
            _row.push(number);
        }
        puzzle.push(_row);
    }
    let solution = solve_puzzle(puzzle, (1i32, 1i32));
    let solution_path = Path::new(file_path).file_stem().unwrap();
    // Write the solution to a file.
    fs::write(format!("{}_solution.txt", solution_path.to_str().unwrap()), solution.to_string()).unwrap();
}