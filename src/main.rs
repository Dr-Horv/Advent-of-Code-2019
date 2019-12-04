use std::time::Instant;

mod lib;
use lib::{Solver, read_lines};

mod day01;
use day01::Day1Solver;
mod day02;
use day02::Day2Solver;
mod day03;
use day03::Day3Solver;
mod day04;
use day04::Day4Solver;

fn get_solver(day: i32) -> (Box<dyn Solver>, String) {
    let day_str = format!("{:02}", day);
    let file = format!("src/day{}/day{}", day_str, day_str);
    let solver: Box<dyn Solver> = match day {
        1 => Box::new(Day1Solver{}),
        2 => Box::new(Day2Solver{}),
        3 => Box::new(Day3Solver{}),
        4 => Box::new(Day4Solver{}),
        _ => panic!("Failed")
    };
    (solver, file)

}

fn main() {
    let day = 4;
    let (solver, file) = get_solver(day);
    let now = Instant::now();
    let answer: String = solver.solve(read_lines(file), true);
    println!("Answer: {}", answer);
    println!("Time: {}", now.elapsed().as_secs_f32());
}
