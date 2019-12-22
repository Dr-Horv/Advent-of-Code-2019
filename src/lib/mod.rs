pub mod intcode_computer;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::Eq;

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

    let answer = solver.solve(lines, part_two);;
    assert_eq!(answer, expected_answer);
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

impl Position {
    pub fn left(&self) -> Position {
        return Position{x: self.x - 1, y: self.y}
    }
    pub fn right(&self) -> Position {
        return Position{x: self.x + 1, y: self.y}
    }
    pub fn down(&self) -> Position {
        return Position{x: self.x, y: self.y - 1}
    }
    pub fn up(&self) -> Position {
        return Position{x: self.x, y: self.y + 1}
    }
}

pub fn manhattan_distance(p1: &Position, p2: &Position) -> i32 {
    return (p1.x - p2.x).abs() + (p1.y - p2.y).abs();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_manhattan_distance() {
        assert_eq!(manhattan_distance(&Position{x: 1, y: 2}, &Position{x: 0, y: 0}), 3);
        assert_eq!(manhattan_distance(&Position{x: 1, y: 2}, &Position{x: -1, y: 5}), 5);
    }

}

