use std::time::Instant;

mod lib;
use lib::{Solver, read_lines};

mod day01;
use day01::Day1Solver;
mod day02;
use day02::Day2Solver;

fn main() {
    let (solver, file) = (Day2Solver{}, "src/day02/day02");

    let now = Instant::now();
    let answer: String = solver.solve(read_lines(file), true);
    println!("Answer: {}", answer);
    println!("Time: {}", now.elapsed().as_secs_f32());
}
