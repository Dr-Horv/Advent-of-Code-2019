//#![feature(crate_in_paths)]

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
mod day05;
use day05::Day5Solver;
mod day06;
use day06::Day6Solver;
mod day07;
use day07::Day7Solver;
mod day08;
use day08::Day8Solver;
mod day09;
use day09::Day9Solver;
mod day10;
use day10::Day10Solver;
mod day11;
use day11::Day11Solver;



fn get_solver(day: i32) -> (Box<dyn Solver>, String) {
    let day_str = format!("{:02}", day);
    let file = format!("src/day{}/day{}", day_str, day_str);
    let solver: Box<dyn Solver> = match day {
        1 => Box::new(Day1Solver{}),
        2 => Box::new(Day2Solver{}),
        3 => Box::new(Day3Solver{}),
        4 => Box::new(Day4Solver{}),
        5 => Box::new(Day5Solver{}),
        6 => Box::new(Day6Solver{}),
        7 => Box::new(Day7Solver{}),
        8 => Box::new(Day8Solver{}),
        9 => Box::new(Day9Solver{}),
        10 => Box::new(Day10Solver{}),
        11 => Box::new(Day11Solver{}),
        _ => panic!("Failed")
    };
    (solver, file)

}

fn main() {
    let day = 11;
    let (solver, file) = get_solver(day);
    let now = Instant::now();
    let answer: String = solver.solve(read_lines(file), true);
    println!("Answer: {}", answer);
    println!("Time: {}", now.elapsed().as_secs_f32());
}
