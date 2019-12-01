use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub trait Solver {
    fn solve(&self, lines: Vec<String>, part_two: bool) -> String;
}

pub fn read_lines<P>(filename: P) -> Vec<String>
    where P: AsRef<Path>, {
    let file = File::open(filename).expect("Failed to open file");
    io::BufReader::new(file).lines().filter_map(io::Result::ok).collect()
}

#[cfg(test)]
pub fn test_solver(solver: &impl Solver, part_two: bool, input: &[&str], expected_answer: &str) {
    let lines = input.into_iter()
        .map(|s| s.to_owned().to_string())
        .collect();

    let answer = solver.solve(lines, part_two);
    assert_eq!(answer, String::from(expected_answer));
}
