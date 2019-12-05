

use crate::lib::Solver;

pub(crate) struct Day5Solver {}

fn get_position(program: &Vec<i32>, index: usize) -> i32 {
    return program[program[index] as usize]
}

fn get_immediate(program: &Vec<i32>, index: usize) -> i32 {
    return program[index]
}

fn get_next_mode(instruction: i32) -> (i32, i32) {
    let mode = instruction % 10;
    let new_inst = instruction / 10;
    return (mode, new_inst)
}

fn run_program(input: i32, program: &mut Vec<i32>) -> i32 {
    let mut index = 0;
    let mut io = input;
    //println!("program={:?}", program);
    loop {
        let mut instruction = program[index];
        println!("instruction={:?}", instruction);
        let op = instruction % 100;
        //println!("op={:?}", op);
        instruction = instruction / 100;
        match op {
            1 => {
                let (mode_a, instruction) = get_next_mode(instruction);
                let (mode_b, instruction) = get_next_mode(instruction);
                let (mode_res, instruction) = get_next_mode(instruction);
                let a = if mode_a == 0 {
                    get_position(program, index + 1)
                } else if mode_a == 1 {
                    get_immediate(program, index + 1)
                } else {
                    panic!("Invalid mode A")
                };
                let b = if mode_b == 0 {
                    get_position(program, index + 2)
                } else if mode_b == 1 {
                    get_immediate(program, index + 2)
                } else {
                    panic!("Invalid mode B")
                };
                if mode_res == 0 {
                    let res = a + b;
                    let modify_index = program[index + 3] as usize;
                    program[modify_index] = res;
                } else {
                    panic!("Invalid mode res")
                }
                index += 4;
            }
            2 => {
                let (mode_a, instruction) = get_next_mode(instruction);
                let (mode_b, instruction) = get_next_mode(instruction);
                let (mode_res, instruction) = get_next_mode(instruction);
                let a = if mode_a == 0 {
                    get_position(program, index + 1)
                } else if mode_a == 1 {
                    get_immediate(program, index + 1)
                } else {
                    panic!("Invalid mode A")
                };
                let b = if mode_b == 0 {
                    get_position(program, index + 2)
                } else if mode_b == 1 {
                    get_immediate(program, index + 2)
                } else {
                    panic!("Invalid mode B")
                };
                if mode_res == 0 {
                    let res = a * b;
                    let modify_index = program[index + 3] as usize;
                    program[modify_index] = res;
//                    println!("mode_a={:?}", mode_a);
//                    println!("a={:?}", a);
//                    println!("b={:?}", b);
//                    println!("modify_index={:?}", modify_index);
//                    println!("res={:?}", res);

                } else {
                    panic!("Invalid mode res")
                }
                index += 4;
            }
            3 => {
                let modify_index = program[index+1] as usize;
                program[modify_index] = io;
                index += 2;
                println!("io={} stored at={}", io, modify_index);
            }
            4 => {
                let modify_index = program[index+1] as usize;
                io = program[modify_index];
                index += 2;
                println!("io={} read from={}", io, modify_index);
            }
            5 => {
                let (mode_a, instruction) = get_next_mode(instruction);
                let (mode_b, instruction) = get_next_mode(instruction);
                let a = if mode_a == 0 {
                    get_position(program, index + 1)
                } else if mode_a == 1 {
                    get_immediate(program, index + 1)
                } else {
                    panic!("Invalid mode A")
                };
                let b = if mode_b == 0 {
                    get_position(program, index + 2)
                } else if mode_b == 1 {
                    get_immediate(program, index + 2)
                } else {
                    panic!("Invalid mode B")
                };
                if a != 0 {
                    index = b as usize;
                } else {
                    index += 3;
                }
            }
            6 => {
                let (mode_a, instruction) = get_next_mode(instruction);
                let (mode_b, instruction) = get_next_mode(instruction);
                let a = if mode_a == 0 {
                    get_position(program, index + 1)
                } else if mode_a == 1 {
                    get_immediate(program, index + 1)
                } else {
                    panic!("Invalid mode A")
                };
                let b = if mode_b == 0 {
                    get_position(program, index + 2)
                } else if mode_b == 1 {
                    get_immediate(program, index + 2)
                } else {
                    panic!("Invalid mode B")
                };
                if a == 0 {
                    index = b as usize;
                } else {
                    index += 3;
                }
            }
            7 => {
                let (mode_a, instruction) = get_next_mode(instruction);
                let (mode_b, instruction) = get_next_mode(instruction);
                let (_, instruction) = get_next_mode(instruction);
                let a = if mode_a == 0 {
                    get_position(program, index + 1)
                } else if mode_a == 1 {
                    get_immediate(program, index + 1)
                } else {
                    panic!("Invalid mode A")
                };
                let b = if mode_b == 0 {
                    get_position(program, index + 2)
                } else if mode_b == 1 {
                    get_immediate(program, index + 2)
                } else {
                    panic!("Invalid mode B")
                };
                let modify_index = program[index + 3] as usize;
                if a < b {
                    program[modify_index] = 1;
                } else {
                    program[modify_index] = 0;
                }
                index += 4;
            }
            8 => {
                let (mode_a, instruction) = get_next_mode(instruction);
                let (mode_b, instruction) = get_next_mode(instruction);
                let (_, instruction) = get_next_mode(instruction);
                let a = if mode_a == 0 {
                    get_position(program, index + 1)
                } else if mode_a == 1 {
                    get_immediate(program, index + 1)
                } else {
                    panic!("Invalid mode A")
                };
                let b = if mode_b == 0 {
                    get_position(program, index + 2)
                } else if mode_b == 1 {
                    get_immediate(program, index + 2)
                } else {
                    panic!("Invalid mode B")
                };
                let modify_index = program[index + 3] as usize;
                println!("a={} b={} index={}", a, b, index);
                if a == b {
                    program[modify_index] = 1;
                } else {
                    program[modify_index] = 0;
                }
                index += 4;
            }
            99 => {
                println!("Last output={}", io);
                return io;
            }
            _ => {
                println!("Op code={}", op);
                panic!("Invalid op code")
            }
        }

        println!("program={:?}", program);

    }
}




impl Solver for Day5Solver {
    fn solve(&self, lines: Vec<String>, part_two: bool) -> String {
        let program_raw = &lines[0];
        let orig_program: Vec<i32> = program_raw
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let mut program: Vec<i32> = orig_program.clone();
        if !part_two {
            run_program(1, &mut program);
        } else {
            run_program(5, &mut program);
        }

        return String::new();

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

        let output = run_program(input, &mut program);
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