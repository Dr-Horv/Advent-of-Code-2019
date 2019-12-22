use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

use crate::lib::Solver;
use crate::lib::intcode_computer;

pub(crate) struct Day7Solver {}

fn valid(a: i128, b: i128, c: i128, d: i128, e: i128) -> bool {
    a != b && a != c && a != d && a != e &&
        b != c && b != d && b != e &&
        c != d && c != e &&
        d != e
}

impl Solver for Day7Solver {
    fn solve(&self, lines: Vec<String>, part_two: bool) -> String {
        let orig_program = intcode_computer::read_program(&lines[0]);

        let mut max: i128 = 0;
        let mut sequence = String::new();
        let low_bound: i128 = if !part_two { 0 } else { 5 };
        let upper_bound = if !part_two { 4 } else { 9 };


        for a in low_bound..=upper_bound {
            for b in low_bound..=upper_bound {
                for c in low_bound..=upper_bound {
                    for d in low_bound..=upper_bound {
                        for e in low_bound..=upper_bound {
                            if valid(a, b, c, d, e) {
                                let mut program_a = orig_program.clone();
                                let mut program_b = orig_program.clone();
                                let mut program_c = orig_program.clone();
                                let mut program_d = orig_program.clone();
                                let mut program_e = orig_program.clone();
                                let (a_input_sender, a_input_receiver): (Sender<i128>, Receiver<i128>) = mpsc::channel();
                                let (b_input_sender, b_input_receiver): (Sender<i128>, Receiver<i128>) = mpsc::channel();
                                let (c_input_sender, c_input_receiver): (Sender<i128>, Receiver<i128>) = mpsc::channel();
                                let (d_input_sender, d_input_receiver): (Sender<i128>, Receiver<i128>) = mpsc::channel();
                                let (e_input_sender, e_input_receiver): (Sender<i128>, Receiver<i128>) = mpsc::channel();
                                let (output_sender, output_receiver): (Sender<i128>, Receiver<i128>) = mpsc::channel();

                                a_input_sender.send(a);
                                b_input_sender.send(b);
                                c_input_sender.send(c);
                                d_input_sender.send(d);
                                e_input_sender.send(e);

                                let ah = thread::spawn(move || {
                                    intcode_computer::run_program(a_input_receiver, b_input_sender, &mut program_a)
                                });

                                let bh = thread::spawn(move || {
                                    intcode_computer::run_program(b_input_receiver, c_input_sender, &mut program_b)
                                });

                                let ch = thread::spawn(move || {
                                    intcode_computer::run_program(c_input_receiver, d_input_sender, &mut program_c)
                                });

                                let dh = thread::spawn(move || {
                                    intcode_computer::run_program(d_input_receiver, e_input_sender, &mut program_d)
                                });

                                let eh = thread::spawn(move || {
                                    intcode_computer::run_program(e_input_receiver, output_sender, &mut program_e)
                                });

                                if !part_two {
                                    a_input_sender.send(0);
                                    let out = output_receiver.recv().unwrap();
                                    if out > max {
                                        max = out;
                                        sequence = format!("{}{}{}{}{}", a, b, c, d, e);
                                    }
                                } else {
                                    thread::spawn(move || {
                                        a_input_sender.send(0);
                                        loop {
                                            let out_result = output_receiver.recv();
                                            if out_result.is_ok() {
                                                a_input_sender.send(out_result.unwrap());
                                            }
                                        }
                                    });

                                    let out = vec![ah, bh, ch, dh, eh].into_iter().map(|h| {
                                        let (io, _) = h.join().unwrap();
                                        io
                                    }).last().unwrap();

                                    if out > max {
                                        max = out;
                                        sequence = format!("{}{}{}{}{}", a, b, c, d, e);
                                    }
                                }

                            }
                        }
                    }
                }
            }
        }
        return max.to_string();
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::test_solver;

    #[test]
    #[ignore]
    fn test_part_one() {
        let solver = Day7Solver {};
        test_solver(&solver, false, &["3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"], "43210");
        test_solver(&solver, false, &["3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"], "54321");
        test_solver(&solver, false, &["3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"], "65210");
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let solver = Day7Solver {};
        test_solver(&solver, true, &["3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"], "139629729");
        test_solver(&solver, true, &["3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"], "18216");

    }
}
