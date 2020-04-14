use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

use crate::lib::{intcode_computer, Position, Solver};

pub(crate) struct Day17Solver {}


fn print_image(image: &HashMap<Position, char>) {
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for (p, _) in image.iter() {
        if p.x < min_x {
            min_x = p.x;
        }
        if p.x > max_x {
            max_x = p.x;
        }
        if p.y < min_y {
            min_y = p.y;
        }
        if p.y > max_y {
            max_y = p.y;
        }
    }
    let mut output = String::new();
    output.push_str("\n");
    for y in (min_y..(max_y + 1)).rev() {
        for x in min_x..(max_x + 1) {
            let c = image.get(&Position { x, y }).unwrap();
            output.push(*c);
        }
        output.push_str("\n");
    }
    println!("{} {} {} {}\n{}", min_x, min_y, max_x, max_y, output.to_string());
}

impl Solver for Day17Solver {
    fn solve(&self, lines: Vec<String>, part_two: bool) -> String {
        let mut program = intcode_computer::read_program(&lines[0]);
        if !part_two {
            let (input_sender, input_receiver) = mpsc::channel();
            let (output_sender, output_receiver) = mpsc::channel();
            thread::spawn(move || {
                intcode_computer::run_program(input_receiver, output_sender, &mut program);
            });

            let mut image: HashMap<Position, char> = HashMap::new();
            let mut position = Position { x: 0, y: 0 };
            loop {
                match output_receiver.recv() {
                    Ok(o) => {
                        print!("{}", (o as u8) as char);
                        match o {
                            35 => {
                                image.insert(position, '#');
                                position = position.right();
                            }
                            10 => {
                                position = Position{ x: 0, y: position.y+1 };
                            },
                            46 => {
                                image.insert(position, '.');
                                position = position.right();
                            }
                            118 => {
                                image.insert(position, 'v');
                                position = position.right();
                            }
                            94 => {
                                image.insert(position, '^');
                                position = position.right();
                            }
                            60 => {
                                image.insert(position, '<');
                                position = position.right();
                            }
                            62 => {
                                image.insert(position, '>');
                                position = position.right();
                            }
                            _ => {
                                println!("Unknown: {}", o);
                                panic!("FAILURE")
                            }
                        }
                    }
                    _ => { break; }
                }
            }

            print_image(&image);


            let alignment_parameter_sum: i32 = image.iter()
                .filter(|&(p, c)| {
                    if *c != '#' {
                        return false;
                    }
                    let up = &p.up();
                    let down = &p.down();
                    let left = &p.left();
                    let right = &p.right();
                    image.contains_key(up) && *image.get(up).unwrap() == '#' &&
                        image.contains_key(down) && *image.get(down).unwrap() == '#' &&
                        image.contains_key(left) && *image.get(left).unwrap() == '#' &&
                        image.contains_key(right) && *image.get(right).unwrap() == '#'
                })
                .map(|(p, c)| {
                    return p.x * p.y;
                }).sum();

            return alignment_parameter_sum.to_string();
        }

        program[0] = 2;
        let (input_sender, input_receiver) = mpsc::channel();
        let (output_sender, output_receiver) = mpsc::channel();
        thread::spawn(move || {
            intcode_computer::run_program(input_receiver, output_sender, &mut program);
        });

        
        for i in vec![65,44,65,44,66,44,67,44,66,44,67,44,66,44,67,44,66,44,65] {
            input_sender.send(i).ok();
        }
        input_sender.send(10).ok();

        for i in vec![76,44,49,48,44,76,44,56,44,82,44,56,44,76,44,56,44,82,44,54] {
            input_sender.send(i).ok();
        }
        input_sender.send(10).ok();

        for i in vec![82,44,54,44,82,44,56,44,82,44,56] {
            input_sender.send(i).ok();
        }
        input_sender.send(10).ok();

        for i in vec![82,44,54,44,82,44,54,44,76,44,56,44,76,44,49,48] {
            input_sender.send(i).ok();
        }
        input_sender.send(10).ok();


        input_sender.send(110).ok();
        input_sender.send(10).ok();

        loop {
            match output_receiver.recv() {
                Ok(o) => {
                    if o < 1_000 {
                        print!("{}", (o as u8) as char);
                    } else {
                        println!("{}", o);
                    }
                }
                _ => { break; }
            }
        }

        return "".to_string();

    }
}

#[cfg(test)]
mod tests {
    use crate::lib::test_solver;

    use super::*;

    #[test]
    fn test_part_one() {
        let solver = Day17Solver {};
        test_solver(&solver, false, &[""], "");
    }

    #[test]
    fn test_part_two() {
        let solver = Day17Solver {};
        test_solver(&solver, true, &[""], "");
    }
}