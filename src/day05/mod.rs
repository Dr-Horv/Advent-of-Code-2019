

use crate::lib::Solver;
use crate::lib::intcode_computer;

pub(crate) struct Day5Solver {}

impl Solver for Day5Solver {
    fn solve(&self, lines: Vec<String>, part_two: bool) -> String {
        let program_raw = &lines[0];
        let orig_program: Vec<i32> = program_raw
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let mut program: Vec<i32> = orig_program.clone();
        let output = if !part_two {
            intcode_computer::run_program(vec![1], &mut program)
        } else {
            intcode_computer::run_program(vec![5], &mut program)
        };

        return output.to_string()

    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::test_solver;

    fn test_program(input: i32, program_raw: &[&str], expected_answer: &str, expected_output: i32) {
        let mut program: Vec<i32> = program_raw[0]
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let output = intcode_computer::run_program(vec![input], &mut program);
        let output_memory = program
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(",");

        assert_eq!(output, expected_output);
        assert_eq!(output_memory, expected_answer);
    }

    #[test]
    fn test_run_program() {
        test_program(42, &["3,0,4,0,99"], "42,0,4,0,99", 42);
    }

    #[test]
    fn test_part_two_examples() {
        test_program(8, &["3,9,8,9,10,9,4,9,99,-1,8"], "3,9,8,9,10,9,4,9,99,1,8", 1);
        test_program(7, &["3,9,8,9,10,9,4,9,99,-1,8"], "3,9,8,9,10,9,4,9,99,0,8", 0);
        test_program(7, &["3,9,7,9,10,9,4,9,99,-1,8"], "3,9,7,9,10,9,4,9,99,1,8", 1);
        test_program(9, &["3,9,7,9,10,9,4,9,99,-1,8"], "3,9,7,9,10,9,4,9,99,0,8", 0);
        test_program(8, &["3,3,1108,-1,8,3,4,3,99"], "3,3,1108,1,8,3,4,3,99", 1);
        test_program(7, &["3,3,1108,-1,8,3,4,3,99"], "3,3,1108,0,8,3,4,3,99", 0);
        test_program(7, &["3,3,1107,-1,8,3,4,3,99"], "3,3,1107,1,8,3,4,3,99", 1);
        test_program(9, &["3,3,1107,-1,8,3,4,3,99"], "3,3,1107,0,8,3,4,3,99", 0);
    }
}