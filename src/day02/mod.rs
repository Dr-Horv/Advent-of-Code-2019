
use crate::lib::Solver;
use crate::lib::intcode_computer;

pub(crate) struct Day2Solver {}

impl Solver for Day2Solver {
    fn solve(&self, lines: Vec<String>, part_two: bool) -> String {
        let orig_program: Vec<i32> = intcode_computer::read_program(&lines[0]);

        if !part_two {
            let mut program: Vec<i32> = orig_program.clone();
            program[1] = 12;
            program[2] = 2;
            intcode_computer::run_program(vec![0], &mut program);
            return program
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(",");
        }

        for noun in 0..99 {
            for verb in 0..99 {
                let mut program: Vec<i32> = orig_program.clone();
                program[1] = noun;
                program[2] = verb;

                intcode_computer::run_program(vec![0], &mut program);

                if program[0] == 19_690_720 {
                    return (noun * 100 + verb).to_string()
                }

            }
        }

        String::from("Fail")
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::test_solver;

    fn test_program(input: &[&str], expected_answer: &str) {
        let mut program: Vec<i32> = input[0]
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        intcode_computer::run_program(vec![0], &mut program);
        let output = program
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(",");

        assert_eq!(output, expected_answer);
    }

    #[test]
    fn test_run_program() {
        test_program(&["1,9,10,3,2,3,11,0,99,30,40,50"], "3500,9,10,70,2,3,11,0,99,30,40,50");
        test_program(&["1,0,0,0,99"], "2,0,0,0,99");
        test_program( &["2,3,0,3,99"], "2,3,0,6,99");
        test_program( &["2,4,4,5,99,0"], "2,4,4,5,99,9801");
        test_program( &["1,1,1,4,99,5,6,0,99"], "30,1,1,4,2,5,6,0,99");
    }


    #[test]
    fn test_part_one() {
        let solver = Day2Solver{};
        test_solver(&solver, false, &["1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,10,19,2,9,19,23,2,23,10,27,1,6,27,31,1,31,6,35,2,35,10,39,1,39,5,43,2,6,43,47,2,47,10,51,1,51,6,55,1,55,6,59,1,9,59,63,1,63,9,67,1,67,6,71,2,71,13,75,1,75,5,79,1,79,9,83,2,6,83,87,1,87,5,91,2,6,91,95,1,95,9,99,2,6,99,103,1,5,103,107,1,6,107,111,1,111,10,115,2,115,13,119,1,119,6,123,1,123,2,127,1,127,5,0,99,2,14,0,0"], "3716250,12,2,2,1,1,2,3,1,3,4,3,1,5,0,3,2,1,10,48,2,9,19,144,2,23,10,576,1,6,27,578,1,31,6,580,2,35,10,2320,1,39,5,2321,2,6,43,4642,2,47,10,18568,1,51,6,18570,1,55,6,18572,1,9,59,18575,1,63,9,18578,1,67,6,18580,2,71,13,92900,1,75,5,92901,1,79,9,92904,2,6,83,185808,1,87,5,185809,2,6,91,371618,1,95,9,371621,2,6,99,743242,1,5,103,743243,1,6,107,743245,1,111,10,743249,2,115,13,3716245,1,119,6,3716247,1,123,2,3716249,1,127,5,0,99,2,14,0,0");
    }

    #[test]
    fn test_part_two() {
        let solver = Day2Solver{};
        test_solver(&solver, true, &["1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,10,19,2,9,19,23,2,23,10,27,1,6,27,31,1,31,6,35,2,35,10,39,1,39,5,43,2,6,43,47,2,47,10,51,1,51,6,55,1,55,6,59,1,9,59,63,1,63,9,67,1,67,6,71,2,71,13,75,1,75,5,79,1,79,9,83,2,6,83,87,1,87,5,91,2,6,91,95,1,95,9,99,2,6,99,103,1,5,103,107,1,6,107,111,1,111,10,115,2,115,13,119,1,119,6,123,1,123,2,127,1,127,5,0,99,2,14,0,0"], "6472");
    }


}

