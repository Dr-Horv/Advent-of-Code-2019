use std::time::{Duration, Instant};
use std::thread::sleep;

mod lib;
use lib::Solver;
use lib::read_lines;

mod day01;
use day01::Day1Solver;

fn main() {
    let (solver, file) = (Day1Solver{}, "day01");

    let now = Instant::now();
    let answer: String = solver.solve(read_lines(file), true);
    println!("Answer: {}", answer);
    println!("Time: {}", now.elapsed().as_secs_f32());
}
