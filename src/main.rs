use std::time::Instant;

mod lib;
use lib::{Solver, read_lines};

mod day01;
use day01::Day1Solver;
mod day02;
use day02::Day2Solver;

fn get_solver(day: i32) -> (Box<dyn Solver>, String) {
    let day_str = format!("{:02}", day);
    let file = format!("src/day{}/day{}", day_str, day_str);
    let solver: Box<dyn Solver> = match day {
        1 => Box::new(Day1Solver{}),
        2 => Box::new(Day2Solver{}),
        _ => panic!("Failed")
    };
    (solver, file)

}

fn main() {
    let day = 2;
    let (solver, file) = get_solver(day);
    let now = Instant::now();
    let answer: String = solver.solve(read_lines(file), true);
    println!("Answer: {}", answer);
    println!("Time: {}", now.elapsed().as_secs_f32());
}
