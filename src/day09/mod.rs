use std::sync::mpsc;

use crate::lib::Solver;
use crate::lib::intcode_computer;

pub(crate) struct Day9Solver {}

impl Solver for Day9Solver {
    fn solve(&self, lines: Vec<String>, part_two: bool) -> String {
        let mut program = intcode_computer::read_program(&lines[0]);
        let (input_sender, input_receiver) = mpsc::channel();
        let (output_sender, output_receiver) = mpsc::channel();

        let input = if !part_two { 1 } else { 2 };

        input_sender.send(input).ok();
        intcode_computer::run_program(input_receiver, output_sender, &mut program);

        let mut latest = 0;
        loop {
            match output_receiver.recv() {
                Ok(o) => {
                    latest = o;
                    println!("Output={}", o)
                }
                _ => return latest.to_string()
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::test_solver;

    #[test]
    fn test_part_one() {
        let solver = Day9Solver {};
        test_solver(&solver, false, &["109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"], "99");
        test_solver(&solver, false, &["1102,34915192,34915192,7,4,7,99,0"], "1219070632396864");
        test_solver(&solver, false, &["104,1125899906842624,99"], "1125899906842624");
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let solver = Day9Solver {};

    }
}