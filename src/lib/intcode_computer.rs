use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

use std::collections::HashMap;

const DEFAULT_MEMORY: i128 = 0;

fn get_position(memory: &mut HashMap<i128, i128>, index: i128) -> i128 {
    let i = memory.get(&index).unwrap_or(&DEFAULT_MEMORY);
    return *memory.get(i).unwrap_or(&DEFAULT_MEMORY);
}

fn get_immediate(memory: &mut HashMap<i128, i128>, index: i128) -> i128 {
    return *memory.get(&index).unwrap_or(&DEFAULT_MEMORY);
}

fn get_relative(memory: &mut HashMap<i128, i128>, index: i128, relative_base: i128) -> i128 {
    let i = memory.get(&index).unwrap_or(&DEFAULT_MEMORY) + relative_base;
    return *memory.get(&i).unwrap_or(&DEFAULT_MEMORY);
}

fn get_next_mode(instruction: i128) -> (i128, i128) {
    let mode = instruction % 10;
    let new_inst = instruction / 10;
    return (mode, new_inst)
}

fn get_parameter(memory: &mut HashMap<i128, i128>, index: i128, mode: i128, relative_base: i128) -> i128 {
    match mode {
        0 => get_position(memory, index),
        1 => get_immediate(memory, index),
        2 => get_relative(memory, index, relative_base),
        _ => panic!("Invalid mode")
    }
}

fn get_output_parameter(memory: &mut HashMap<i128, i128>, index: i128, mode: i128, relative_base: i128) -> i128 {
    match mode {
        0 => *memory.get(&(index)).unwrap_or(&DEFAULT_MEMORY),
        2 => {
            let parameter_index = *memory.get(&(index)).unwrap_or(&DEFAULT_MEMORY);
            parameter_index + relative_base
        },
        _ => panic!("Invalid mode")
    }
}

fn apply_binary_operator(a: i128, b: i128, op: fn(i128, i128) -> i128,  index: i128, memory: &mut HashMap<i128, i128>) {
    let res = op(a,b);
    memory.insert(index, res);
}


fn jump_if_check(index: i128, a: i128, b: i128, check: fn(i128) -> bool) -> i128 {
    if check(a) {
        b
    } else {
        index + 3
    }
}

fn perform_check(index: i128, a: i128, b: i128, check: fn(i128, i128) -> bool, memory: &mut HashMap<i128, i128>) {
    if check(a,b) {
        memory.insert(index, 1);
    } else {
        memory.insert(index, 0);
    };
}


fn equal_check(memory: &mut HashMap<i128, i128>, index: i128, relative_base: i128, instruction: i128) -> i128 {
    let (mode_a, instruction) = get_next_mode(instruction);
    let (mode_b, instruction) = get_next_mode(instruction);
    let (mode_c, instruction) = get_next_mode(instruction);
    let a = get_parameter(memory, index + 1, mode_a, relative_base);
    let b = get_parameter(memory, index + 2, mode_b, relative_base);
    let c = get_output_parameter(memory, index + 3, mode_c, relative_base);
    perform_check(c, a, b, |x, y| x == y, memory);
    index + 4
}

fn less_than_check(memory: &mut HashMap<i128, i128>, mut index: i128, relative_base: i128, instruction: i128) -> i128 {
    let (mode_a, instruction) = get_next_mode(instruction);
    let (mode_b, instruction) = get_next_mode(instruction);
    let (mode_c, instruction) = get_next_mode(instruction);
    let a = get_parameter(memory, index + 1, mode_a, relative_base);
    let b = get_parameter(memory, index + 2, mode_b, relative_base);
    let c = get_output_parameter(memory, index + 3, mode_c, relative_base);
    perform_check(c, a, b, |x, y| x < y, memory);
    index + 4
}

fn jump_if_equal(memory: &mut HashMap<i128, i128>, mut index: i128, relative_base: i128, instruction: i128) -> i128 {
    let (mode_a, instruction) = get_next_mode(instruction);
    let (mode_b, instruction) = get_next_mode(instruction);
    let a = get_parameter(memory, index + 1, mode_a, relative_base);
    let b = get_parameter(memory, index + 2, mode_b, relative_base);
    jump_if_check(index, a, b, |x| x == 0)
}

fn jump_if_ne(memory: &mut HashMap<i128, i128>, mut index: i128, relative_base: i128, instruction: i128) -> i128 {
    let (mode_a, instruction) = get_next_mode(instruction);
    let (mode_b, instruction) = get_next_mode(instruction);
    let a = get_parameter(memory, index + 1, mode_a, relative_base);
    let b = get_parameter(memory, index + 2, mode_b, relative_base);
    jump_if_check(index, a, b, |x| x != 0)
}

fn multiply(memory: &mut HashMap<i128, i128>, index: i128, relative_base: i128, instruction: i128) -> i128 {
    let (mode_a, instruction) = get_next_mode(instruction);
    let (mode_b, instruction) = get_next_mode(instruction);
    let (mode_res, instruction) = get_next_mode(instruction);
    let a = get_parameter(memory, index + 1, mode_a, relative_base);
    let b = get_parameter(memory, index + 2, mode_b, relative_base);
    let c = get_output_parameter(memory, index+3, mode_res, relative_base);
    apply_binary_operator(a, b, |x, y| x * y, c, memory);
    index + 4
}

fn add(memory: &mut HashMap<i128, i128>, index: i128, relative_base: i128, instruction: i128) -> i128 {
    let (mode_a, instruction) = get_next_mode(instruction);
    let (mode_b, instruction) = get_next_mode(instruction);
    let (mode_res, instruction) = get_next_mode(instruction);
    let a = get_parameter(memory, index + 1, mode_a, relative_base);
    let b = get_parameter(memory, index + 2, mode_b, relative_base);
    let c = get_output_parameter(memory, index+3, mode_res, relative_base);
    apply_binary_operator(a, b, |x, y| x + y, c, memory);
    index + 4
}

pub fn read_program(raw: &String) -> Vec<i128> {
    raw.split(',')
        .map(|s| s.parse::<i128>().unwrap())
        .collect()
}

pub fn run_program(input: Receiver<i128>, output: Sender<i128>, program: &Vec<i128>) -> (i128, HashMap<i128, i128>) {
    let mut memory = HashMap::new();
    for i in 0..program.len() {
        memory.insert(i as i128, program[i]);
    }

    let mut index: i128 = 0;
    let mut relative_base: i128 = 0;
    let mut io= 0;
    //println!("program={:?}", program);
    loop {
        let mut instruction = *memory.get(&index).unwrap_or(&DEFAULT_MEMORY);
        //println!("index={:?}", index);
        //println!("instruction={:?}", instruction);
        let op = instruction % 100;
        //println!("op={:?}", op);
        instruction = instruction / 100;
        index = match op {
            1 => add(&mut memory, index, relative_base, instruction),
            2 => multiply(&mut memory, index, relative_base, instruction),
            3 => {
                let (mode, instruction) = get_next_mode(instruction);
                match mode {
                    0 => {
                        let modify_index = *memory.get(&(index+1)).unwrap_or(&DEFAULT_MEMORY);
                        memory.insert(modify_index, input.recv().unwrap());
                    }
                    /*1 => {
                        let modify_index = memory.get(&(index+1)).unwrap_or(&DEFAULT_MEMORY);
                        memory.insert(*modify_index, input.recv().unwrap());
                    }*/
                    2 => {
                        let parameter_index = *memory.get(&(index+1)).unwrap_or(&DEFAULT_MEMORY);
                        let modify_index = parameter_index + relative_base;
                        memory.insert(modify_index, input.recv().unwrap());
                    }
                    _ => panic!("Invalid mode")
                }

                index + 2
            }
            4 => {
                let (mode, instruction) = get_next_mode(instruction);
                match mode {
                    0 => {
                        let modify_index = *memory.get(&(index+1)).unwrap_or(&DEFAULT_MEMORY);
                        io = *memory.get(&modify_index).unwrap_or(&DEFAULT_MEMORY);
                        output.send(io);
                    }
                    1 => {
                        io = *memory.get(&(index+1)).unwrap_or(&DEFAULT_MEMORY);
                        output.send(io);
                    }
                    2 => {
                        let parameter_index = *memory.get(&(index+1)).unwrap_or(&DEFAULT_MEMORY);
                        let modify_index = parameter_index + relative_base;
                        io = *memory.get(&modify_index).unwrap_or(&DEFAULT_MEMORY);
                        output.send(io);
                    }
                    _ => panic!("Invalid mode")
                }
                index + 2
            }
            5 => jump_if_ne(&mut memory, index, relative_base, instruction),
            6 => jump_if_equal(&mut memory, index, relative_base, instruction),
            7 => less_than_check(&mut memory, index, relative_base, instruction),
            8 => equal_check(&mut memory, index, relative_base, instruction),
            9 => {
                let (mode_a, instruction) = get_next_mode(instruction);
                let a = get_parameter(&mut memory, index + 1, mode_a, relative_base) as i128;
                relative_base += a;
                index + 2
            }
            99 => {
                //println!("Last output={}", io);
                return (io, memory);
            }
            _ => {
                println!("Op code={}", op);
                panic!("Invalid op code")
            }
        };

        //println!("program={:?}", program);

    }
}

