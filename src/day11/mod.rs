use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

use crate::day11::Direction::{DOWN, LEFT, RIGHT, UP};
use crate::lib::{intcode_computer, Position, Solver};

pub(crate) struct Day11Solver {}

enum Direction {
    UP,
    LEFT,
    RIGHT,
    DOWN,
}

struct PaintRobot {
    pub position: Position,
    pub orientation: Direction,
}

fn turn_left(dir: &Direction) -> Direction {
    return match dir {
        UP => LEFT,
        LEFT => DOWN,
        RIGHT => UP,
        DOWN => RIGHT,
    }
}

fn turn_right(dir: &Direction) -> Direction {
    return match dir {
        UP => RIGHT,
        LEFT => UP,
        RIGHT => DOWN,
        DOWN => LEFT,
    }
}

fn move_bot(bot: &PaintRobot) -> Position {
    return match bot.orientation {
        UP => bot.position.up(),
        LEFT => bot.position.left(),
        RIGHT => bot.position.right(),
        DOWN => bot.position.down(),
    }
}

fn print_state(bot: &PaintRobot, painting: &HashMap<Position, bool>) -> String {
    let mut min_x = bot.position.x;
    let mut min_y = bot.position.y;
    let mut max_x = bot.position.x;
    let mut max_y = bot.position.y;
    for (p, b) in painting.iter() {
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
            if bot.position.x == x && bot.position.y == y {
                let char_to_print = match bot.orientation {
                    UP => '^',
                    LEFT => '<',
                    RIGHT => '>',
                    DOWN => 'v',
                };
                output.push(char_to_print)
            } else {
                let is_white = painting.get(&Position{x, y }).unwrap_or(&false);
                if *is_white {
                    output.push('#')
                } else {
                    output.push('.')
                }
            }
            output.push_str("  ")
        }
        output.push_str("\n");
    }
    return output.to_string();
}

impl Solver for Day11Solver {
    fn solve(&self, lines: Vec<String>, part_two: bool) -> String {
        let mut program = intcode_computer::read_program(&lines[0]);
        let (input_sender, input_receiver) = mpsc::channel();
        let (output_sender, output_receiver) = mpsc::channel();
        if part_two {
            input_sender.send(1);
        } else {
            input_sender.send(0);
        }
        let bot_computer = thread::spawn(move || {
            intcode_computer::run_program(input_receiver, output_sender, &mut program);
        });

        let mut painting = HashMap::new();
        let mut bot = PaintRobot { position: Position { x: 0, y: 0 }, orientation: UP };
        let mut latest = 0;
        let mut painting_mode = true;
        let mut loops = 0;
        loop {
            // print_state(&bot, &painting);
            match output_receiver.recv() {
                Ok(o) => {
                    latest = o;
                    //println!("Output={} Painting_mode={}", o, painting_mode);
                    if painting_mode {
                        let painting_white = if o == 0 {
                            false
                        } else if o == 1 {
                            true
                        } else {
                            panic!("Unexpected input!")
                        };
                        painting.insert(bot.position, painting_white);
                        //println!("Painting={} at={:?}", painting_white, bot.position);
                        painting_mode = false;
                    } else {
                        if o == 0 {
                            bot.orientation = turn_left(&bot.orientation)
                        } else if o == 1 {
                            bot.orientation = turn_right(&bot.orientation)
                        }
                        let now = bot.position;
                        bot.position = move_bot(&bot);
                        let after = bot.position;
                        let is_white = painting.get(&bot.position).unwrap_or(&false);
                        //println!("Turned {} and moving from {:?} to {:?} reading {}", if o == 0 { "LEFT" } else { "RIGHT" }, now, after, *is_white);
                        if *is_white {
                            input_sender.send(1);
                        } else {
                            input_sender.send(0);
                        }

                        painting_mode = true;
                    }
                }
                _ => return if part_two { print_state(&bot, &painting) } else { painting.len().to_string() }
            }
            loops += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::test_solver;

    #[test]
    fn test_part_one() {
        let solver = Day11Solver {};
        test_solver(&solver, false, &["3,8,1005,8,306,1106,0,11,0,0,0,104,1,104,0,3,8,1002,8,-1,10,1001,10,1,10,4,10,108,1,8,10,4,10,1002,8,1,28,2,107,3,10,1,101,19,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,0,10,4,10,102,1,8,59,2,5,13,10,3,8,102,-1,8,10,1001,10,1,10,4,10,1008,8,0,10,4,10,1001,8,0,85,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,1,10,4,10,1001,8,0,107,1006,0,43,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,1,10,4,10,101,0,8,132,3,8,102,-1,8,10,1001,10,1,10,4,10,1008,8,0,10,4,10,1001,8,0,154,2,4,1,10,2,4,9,10,3,8,1002,8,-1,10,101,1,10,10,4,10,108,0,8,10,4,10,1001,8,0,183,1,1102,5,10,1,1102,1,10,1006,0,90,2,9,12,10,3,8,102,-1,8,10,1001,10,1,10,4,10,1008,8,0,10,4,10,1001,8,0,221,1006,0,76,1006,0,27,1,102,9,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,108,1,8,10,4,10,102,1,8,252,2,4,9,10,1006,0,66,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,1,10,4,10,101,0,8,282,1,102,19,10,101,1,9,9,1007,9,952,10,1005,10,15,99,109,628,104,0,104,1,21102,1,387240010644,1,21101,0,323,0,1105,1,427,21102,846541370112,1,1,21101,334,0,0,1106,0,427,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,21102,3425718295,1,1,21102,381,1,0,1105,1,427,21102,179410541715,1,1,21101,0,392,0,1106,0,427,3,10,104,0,104,0,3,10,104,0,104,0,21101,0,718078255872,1,21101,0,415,0,1105,1,427,21102,1,868494234468,1,21102,1,426,0,1105,1,427,99,109,2,21202,-1,1,1,21101,0,40,2,21101,458,0,3,21101,0,448,0,1106,0,491,109,-2,2106,0,0,0,1,0,0,1,109,2,3,10,204,-1,1001,453,454,469,4,0,1001,453,1,453,108,4,453,10,1006,10,485,1102,0,1,453,109,-2,2105,1,0,0,109,4,2102,1,-1,490,1207,-3,0,10,1006,10,508,21102,1,0,-3,22102,1,-3,1,22101,0,-2,2,21102,1,1,3,21102,1,527,0,1106,0,532,109,-4,2105,1,0,109,5,1207,-3,1,10,1006,10,555,2207,-4,-2,10,1006,10,555,22101,0,-4,-4,1105,1,623,22101,0,-4,1,21201,-3,-1,2,21202,-2,2,3,21101,574,0,0,1105,1,532,21202,1,1,-4,21102,1,1,-1,2207,-4,-2,10,1006,10,593,21102,0,1,-1,22202,-2,-1,-2,2107,0,-3,10,1006,10,615,21201,-1,0,1,21101,615,0,0,106,0,490,21202,-2,-1,-2,22201,-4,-2,-4,109,-5,2105,1,0"], "1686");
    }

    #[test]
    fn test_part_two() {
        let solver = Day11Solver {};
        test_solver(&solver, true, &["3,8,1005,8,306,1106,0,11,0,0,0,104,1,104,0,3,8,1002,8,-1,10,1001,10,1,10,4,10,108,1,8,10,4,10,1002,8,1,28,2,107,3,10,1,101,19,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,0,10,4,10,102,1,8,59,2,5,13,10,3,8,102,-1,8,10,1001,10,1,10,4,10,1008,8,0,10,4,10,1001,8,0,85,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,1,10,4,10,1001,8,0,107,1006,0,43,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,1,10,4,10,101,0,8,132,3,8,102,-1,8,10,1001,10,1,10,4,10,1008,8,0,10,4,10,1001,8,0,154,2,4,1,10,2,4,9,10,3,8,1002,8,-1,10,101,1,10,10,4,10,108,0,8,10,4,10,1001,8,0,183,1,1102,5,10,1,1102,1,10,1006,0,90,2,9,12,10,3,8,102,-1,8,10,1001,10,1,10,4,10,1008,8,0,10,4,10,1001,8,0,221,1006,0,76,1006,0,27,1,102,9,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,108,1,8,10,4,10,102,1,8,252,2,4,9,10,1006,0,66,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,1,10,4,10,101,0,8,282,1,102,19,10,101,1,9,9,1007,9,952,10,1005,10,15,99,109,628,104,0,104,1,21102,1,387240010644,1,21101,0,323,0,1105,1,427,21102,846541370112,1,1,21101,334,0,0,1106,0,427,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,21102,3425718295,1,1,21102,381,1,0,1105,1,427,21102,179410541715,1,1,21101,0,392,0,1106,0,427,3,10,104,0,104,0,3,10,104,0,104,0,21101,0,718078255872,1,21101,0,415,0,1105,1,427,21102,1,868494234468,1,21102,1,426,0,1105,1,427,99,109,2,21202,-1,1,1,21101,0,40,2,21101,458,0,3,21101,0,448,0,1106,0,491,109,-2,2106,0,0,0,1,0,0,1,109,2,3,10,204,-1,1001,453,454,469,4,0,1001,453,1,453,108,4,453,10,1006,10,485,1102,0,1,453,109,-2,2105,1,0,0,109,4,2102,1,-1,490,1207,-3,0,10,1006,10,508,21102,1,0,-3,22102,1,-3,1,22101,0,-2,2,21102,1,1,3,21102,1,527,0,1106,0,532,109,-4,2105,1,0,109,5,1207,-3,1,10,1006,10,555,2207,-4,-2,10,1006,10,555,22101,0,-4,-4,1105,1,623,22101,0,-4,1,21201,-3,-1,2,21202,-2,2,3,21101,574,0,0,1105,1,532,21202,1,1,-4,21102,1,1,-1,2207,-4,-2,10,1006,10,593,21102,0,1,-1,22202,-2,-1,-2,2107,0,-3,10,1006,10,615,21201,-1,0,1,21101,615,0,0,106,0,490,21202,-2,-1,-2,22201,-4,-2,-4,109,-5,2105,1,0"], "\n.  .  #  #  .  .  .  #  #  .  .  #  #  #  .  .  #  #  #  .  .  #  .  .  #  .  #  #  #  #  .  #  .  .  #  .  #  .  .  .  .  .  .  \n.  #  .  .  #  .  #  .  .  #  .  #  .  .  #  .  #  .  .  #  .  #  .  #  .  .  .  .  .  #  .  #  .  .  #  .  #  .  .  .  .  .  .  \n.  #  .  .  .  .  #  .  .  #  .  #  .  .  #  .  #  .  .  #  .  #  #  .  .  .  .  .  #  .  .  #  .  .  #  .  #  .  .  .  .  .  .  \n.  #  .  #  #  .  #  #  #  #  .  #  #  #  .  .  #  #  #  .  .  #  .  #  .  .  .  #  .  .  .  #  .  .  #  .  #  .  .  .  .  .  .  \n.  #  .  .  #  .  #  .  .  #  .  #  .  #  .  .  #  .  .  .  .  #  .  #  .  .  #  .  .  .  .  #  .  .  #  .  #  .  .  .  .  >  .  \n.  .  #  #  #  .  #  .  .  #  .  #  .  .  #  .  #  .  .  .  .  #  .  .  #  .  #  #  #  #  .  .  #  #  .  .  #  #  #  #  .  .  .  \n");
    }



}