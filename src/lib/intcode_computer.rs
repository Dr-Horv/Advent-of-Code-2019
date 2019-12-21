


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

fn get_parameter(program: &mut Vec<i32>, mut index: usize, mode: i32) -> i32 {
    match mode {
        0 => get_position(program, index),
        1 => get_immediate(program, index),
        _ => panic!("Invalid mode A")
    }
}

fn apply_binary_operator(a: i32, b: i32, op: fn(i32, i32) -> i32,  index: usize, program: &mut Vec<i32>) {
    let res = op(a,b);
    let modify_index = program[index + 3] as usize;
    program[modify_index] = res;
}


fn jump_if_check(index: usize, a: i32, b: i32, check: fn(i32) -> bool) -> usize {
    if check(a) {
        b as usize
    } else {
        index + 3
    }
}

fn perform_check(index: usize, a: i32, b: i32, check: fn(i32, i32) -> bool, program: &mut Vec<i32>) -> usize {
    let modify_index = program[index + 3] as usize;
    if check(a,b) {
        program[modify_index] = 1;
    } else {
        program[modify_index] = 0;
    }
    index + 4
}


fn equal_check(program: &mut Vec<i32>, mut index: usize, mut instruction: i32) -> usize {
    let (mode_a, instruction) = get_next_mode(instruction);
    let (mode_b, instruction) = get_next_mode(instruction);
    let (_, instruction) = get_next_mode(instruction);
    let a = get_parameter(program, index + 1, mode_a);
    let b = get_parameter(program, index + 2, mode_b);
    perform_check(index, a, b, |x, y| x == y, program)
}

fn less_than_check(program: &mut Vec<i32>, mut index: usize, mut instruction: i32) -> usize {
    let (mode_a, instruction) = get_next_mode(instruction);
    let (mode_b, instruction) = get_next_mode(instruction);
    let (_, instruction) = get_next_mode(instruction);
    let a = get_parameter(program, index + 1, mode_a);
    let b = get_parameter(program, index + 2, mode_b);
    perform_check(index, a, b, |x, y| x < y, program)
}

fn jump_if_equal(program: &mut Vec<i32>, mut index: usize, mut instruction: i32) -> usize {
    let (mode_a, instruction) = get_next_mode(instruction);
    let (mode_b, instruction) = get_next_mode(instruction);
    let a = get_parameter(program, index + 1, mode_a);
    let b = get_parameter(program, index + 2, mode_b);
    jump_if_check(index, a, b, |x| x == 0)
}

fn jump_if_ne(program: &mut Vec<i32>, mut index: usize, mut instruction: i32) -> usize {
    let (mode_a, instruction) = get_next_mode(instruction);
    let (mode_b, instruction) = get_next_mode(instruction);
    let a = get_parameter(program, index + 1, mode_a);
    let b = get_parameter(program, index + 2, mode_b);
    jump_if_check(index, a, b, |x| x != 0)
}

fn multiply(program: &mut Vec<i32>, index: usize, mut instruction: i32) -> usize {
    let (mode_a, instruction) = get_next_mode(instruction);
    let (mode_b, instruction) = get_next_mode(instruction);
    let (mode_res, instruction) = get_next_mode(instruction);
    let a = get_parameter(program, index + 1, mode_a);
    let b = get_parameter(program, index + 2, mode_b);
    if mode_res != 0 {
        panic!("Invalid mode res")
    }
    apply_binary_operator(a, b, |x, y| x * y, index, program);
    index + 4
}

fn add(program: &mut Vec<i32>, index: usize, mut instruction: i32) -> usize {
    let (mode_a, instruction) = get_next_mode(instruction);
    let (mode_b, instruction) = get_next_mode(instruction);
    let (mode_res, instruction) = get_next_mode(instruction);
    let a = get_parameter(program, index + 1, mode_a);
    let b = get_parameter(program, index + 2, mode_b);
    if mode_res != 0 {
        panic!("Invalid mode res")
    }
    apply_binary_operator(a, b, |x, y| x + y, index, program);
    index + 4
}

pub fn run_program(input: impl IntoIterator<Item = i32>, program: &mut Vec<i32>) -> i32 {
    let mut index = 0;
    let mut input_iterator = input.into_iter();
    let mut io = 0;
    //println!("program={:?}", program);
    loop {
        let mut instruction = program[index];
        println!("instruction={:?}", instruction);
        let op = instruction % 100;
        //println!("op={:?}", op);
        instruction = instruction / 100;
        index = match op {
            1 => add(program, index, instruction),
            2 => multiply(program, index, instruction),
            3 => {
                let modify_index = program[index+1] as usize;
                program[modify_index] = input_iterator.next().unwrap();
                index + 2
            }
            4 => {
                let modify_index = program[index+1] as usize;
                io = program[modify_index];
                println!("io={} read from={}", io, modify_index);
                index + 2
            }
            5 => jump_if_ne(program, index, instruction),
            6 => jump_if_equal(program, index, instruction),
            7 => less_than_check(program, index, instruction),
            8 => equal_check(program, index, instruction),
            99 => {
                println!("Last output={}", io);
                return io;
            }
            _ => {
                println!("Op code={}", op);
                panic!("Invalid op code")
            }
        };

        println!("program={:?}", program);

    }
}

