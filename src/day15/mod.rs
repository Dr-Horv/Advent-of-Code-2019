

use crate::lib::{intcode_computer, Position, Solver};
use std::sync::mpsc;
use std::thread;
use std::collections::{HashMap, VecDeque, HashSet};
use crate::day15::LocationType::{Empty, Wall, OxygenSystem, Oxygen};
use crate::day15::Direction::{NORTH, WEST, EAST, SOUTH};

pub(crate) struct Day15Solver {}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
enum LocationType {
    Empty,
    Wall,
    OxygenSystem,
    Oxygen,
}

fn create_location_type(i: i128) -> LocationType {
    return match i {
        0 => Wall,
        1 => Empty,
        2 => OxygenSystem,
        _ => panic!("Invalid location type")
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
enum Direction {
    NORTH,
    WEST,
    EAST,
    SOUTH
}

impl Direction {
    fn get_command(&self) -> i32 {
        return match self {
            NORTH => 1,
            WEST => 3,
            EAST => 4,
            SOUTH => 2,
        }
    }
}

fn get_neighbours(position: &Position, map: &HashMap<Position, LocationType>) -> Vec<Position> {
    let mut neighbours = Vec::new();
    for d in vec![NORTH, WEST, EAST, SOUTH] {
        let n = match d {
            NORTH => position.up(),
            WEST => position.left(),
            EAST => position.right(),
            SOUTH => position.down(),
        };
        if map.contains_key(&n) {
            let location_type = map.get(&n).unwrap();
            match location_type {
                Empty => neighbours.push(n),
                OxygenSystem => neighbours.push(n),
                _ => {},
            }
        } else {
            neighbours.push(n)
        }
    }
    return neighbours;
}

fn get_direction(p1: &Position, p2: &Position) -> Direction {
    return if p1.x != p2.x {
        if p1.x < p2.x {
            EAST
        } else {
            WEST
        }
    } else {
        if p1.y < p2.y {
            NORTH
        } else {
            SOUTH
        }
    }
}

fn reconstruct_path(goal: &Position, from: &HashMap<Position, Position>) -> Vec<Direction> {
    let mut path = Vec::new();
    path.push(goal);
    let mut curr = goal;
    while from.contains_key(&curr) {
        curr = from.get(curr).unwrap();
        path.push(curr);
    }
    path.reverse();
    let mut directions = Vec::new();
    for i in 0..(path.len()-1) {
        let p1 = path[i];
        let p2 = path[i+1];
        let dir = get_direction(p1,p2);
        directions.push(dir);
    }
    return directions;
}

fn find_min<'a>(f_score: &HashMap<Position, i32>, set: &'a HashSet<Position>) -> Option<&'a Position> {
    return set.into_iter()
        .min_by(|p1, p2| {
            let i1 = f_score.get(p1).unwrap_or(&10_000);
            let i2 = f_score.get(p2).unwrap_or(&10_000);
            return i1.cmp(i2);}
        );
}

fn search(position: Position, map: &HashMap<Position, LocationType>, goal_function: &dyn Fn(&HashMap<Position, LocationType>, &Position) -> bool) -> Vec<Direction> {
    let h = |_: Position| 1;
    let mut open_set = HashSet::new();
    open_set.insert(position);
    let mut came_from:HashMap<Position, Position> = HashMap::new();
    let mut g_score: HashMap<Position, i32> = HashMap::new();
    g_score.insert(position, 0);
    let mut f_score: HashMap<Position, i32> = HashMap::new();
    f_score.insert(position, h(position));

    loop {
        let current_opt = find_min(&f_score, &open_set);
        if current_opt.is_none() {
            break;
        }
        let current = *current_opt.unwrap();
        if goal_function(map, &current) {
            return reconstruct_path(&current, &came_from);
        }
        open_set.remove(&current);
        for neighbour in get_neighbours(&current, &map) {
            let tentative_g_score = g_score.get(&current).unwrap_or(&10_000) + 1;
            let neighbour_g_score =  *g_score.get(&neighbour).unwrap_or(&10_000);
            if tentative_g_score < neighbour_g_score {
                came_from.insert(neighbour, current);
                g_score.insert(neighbour, tentative_g_score);
                f_score.insert(neighbour, neighbour_g_score + h(neighbour));
                if !open_set.contains(&neighbour) {
                    open_set.insert(neighbour);
                }
            }

        }
    }

    return vec![];
}

fn oxygenate(map: &HashMap<Position, LocationType>) -> i32 {
    let (oxygen_position, _) = map.into_iter()
        .find(|(_,&l)| l == OxygenSystem)
        .unwrap();
    let mut oxygen_map: HashMap<Position, LocationType> = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((*oxygen_position, 0));
    oxygen_map.insert(*oxygen_position, Oxygen);
    let mut time = 0;
    while !queue.is_empty() {
        let (next, d) = queue.pop_front().unwrap();
        if d > time {
            time = d;
        }
        for n in get_neighbours(&next, map) {
            if !oxygen_map.contains_key(&n) {
                oxygen_map.insert(n, Oxygen);
                queue.push_back((n, d+1));
            }
        }
    }
    return time;
}



impl Solver for Day15Solver {
    fn solve(&self, lines: Vec<String>, part_two: bool) -> String {
        let mut program = intcode_computer::read_program(&lines[0]);
        let (input_sender, input_receiver) = mpsc::channel();
        let (output_sender, output_receiver) = mpsc::channel();

        thread::spawn(move || {
            intcode_computer::run_program(input_receiver, output_sender, &mut program);
        });

        let mut droid_location = Position{x: 0, y: 0};
        let mut map = HashMap::new();
        map.insert(droid_location, Empty);
        let find_unknown = |map: &HashMap<Position, LocationType>, p: &Position| {
            return !map.contains_key(p);
        };
        loop {
            let path = search(droid_location, &map, &find_unknown);
            if path.is_empty() {
                break;
            }
            for d in path {
                input_sender.send(d.get_command() as i128).ok();
                let next_pos = match d {
                    NORTH => droid_location.up(),
                    WEST => droid_location.left(),
                    EAST => droid_location.right(),
                    SOUTH => droid_location.down(),
                };
                match output_receiver.recv() {
                    Ok(o) => {
                        let lt = create_location_type(o);
                        match lt {
                            Empty => {
                                map.insert(next_pos, Empty);
                                droid_location = next_pos;
                            },
                            Wall => {map.insert(next_pos, Wall);},
                            OxygenSystem => {
                                map.insert(next_pos, OxygenSystem);
                                droid_location = next_pos;
                            },
                            Oxygen => {
                                panic!("Should not happen")
                            }
                        }
                    }
                    _ => { panic!("END OF PROGRAM") }
                }
            }
        }

        return if !part_two {
            let find_oxygen_system = |map: &HashMap<Position, LocationType>, p: &Position| {
                return map.contains_key(p) && *map.get(p).unwrap() == OxygenSystem;
            };
            let path = search(Position { x: 0, y: 0 }, &map, &find_oxygen_system);

            path.len().to_string()
        } else {
            let time = oxygenate(&map);
            time.to_string()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::test_solver;

    #[test]
    fn test_part_one() {
        let solver = Day15Solver {};
        test_solver(&solver, false, &["3,1033,1008,1033,1,1032,1005,1032,31,1008,1033,2,1032,1005,1032,58,1008,1033,3,1032,1005,1032,81,1008,1033,4,1032,1005,1032,104,99,1002,1034,1,1039,1001,1036,0,1041,1001,1035,-1,1040,1008,1038,0,1043,102,-1,1043,1032,1,1037,1032,1042,1105,1,124,102,1,1034,1039,1001,1036,0,1041,1001,1035,1,1040,1008,1038,0,1043,1,1037,1038,1042,1106,0,124,1001,1034,-1,1039,1008,1036,0,1041,1002,1035,1,1040,1002,1038,1,1043,101,0,1037,1042,1106,0,124,1001,1034,1,1039,1008,1036,0,1041,101,0,1035,1040,1002,1038,1,1043,101,0,1037,1042,1006,1039,217,1006,1040,217,1008,1039,40,1032,1005,1032,217,1008,1040,40,1032,1005,1032,217,1008,1039,37,1032,1006,1032,165,1008,1040,9,1032,1006,1032,165,1101,2,0,1044,1105,1,224,2,1041,1043,1032,1006,1032,179,1101,1,0,1044,1106,0,224,1,1041,1043,1032,1006,1032,217,1,1042,1043,1032,1001,1032,-1,1032,1002,1032,39,1032,1,1032,1039,1032,101,-1,1032,1032,101,252,1032,211,1007,0,50,1044,1106,0,224,1102,0,1,1044,1105,1,224,1006,1044,247,1001,1039,0,1034,102,1,1040,1035,102,1,1041,1036,101,0,1043,1038,102,1,1042,1037,4,1044,1106,0,0,37,22,74,27,37,99,30,8,72,31,49,29,51,32,85,21,39,72,2,2,43,94,31,11,76,43,95,21,38,8,90,13,39,97,54,47,14,6,20,49,5,30,97,9,99,64,71,24,36,87,52,94,36,18,52,42,83,38,98,53,26,87,69,32,18,94,2,93,97,15,65,65,21,40,99,19,91,13,4,89,38,70,65,41,73,49,62,54,37,46,14,49,88,86,13,89,23,89,10,3,48,57,92,43,65,4,35,97,48,10,19,64,3,79,38,87,6,13,71,49,74,43,92,8,4,71,6,35,85,98,94,6,38,59,80,65,46,62,63,62,49,61,68,6,7,64,66,40,56,82,59,30,85,45,57,36,86,70,25,83,31,96,65,19,16,67,55,36,49,54,29,75,69,3,3,37,75,49,23,65,22,6,52,75,31,7,87,85,19,48,97,65,51,78,10,35,40,59,54,14,85,6,30,94,68,42,87,46,75,26,82,36,21,65,90,16,59,14,76,55,37,41,99,80,9,79,12,59,17,75,2,40,52,45,76,45,16,82,13,55,61,14,11,49,97,81,99,38,35,20,98,51,64,13,24,85,94,38,25,87,1,42,89,18,32,54,55,17,15,84,98,25,31,21,55,44,57,59,11,78,49,72,87,20,7,33,91,80,75,18,33,37,52,7,26,87,65,36,52,92,6,8,95,89,37,38,57,25,23,71,75,47,20,87,90,37,54,38,77,32,39,67,16,69,62,15,96,47,91,95,18,96,24,45,21,64,9,72,2,54,65,39,36,54,23,71,74,18,26,97,35,44,29,87,54,48,31,55,33,85,74,13,99,82,39,35,97,43,20,62,58,86,98,41,47,92,79,74,10,85,28,66,86,18,35,5,84,67,13,91,47,44,1,84,56,32,96,7,77,21,88,92,38,31,65,82,87,45,55,4,60,58,64,49,53,3,63,32,52,43,10,66,75,96,53,11,95,44,36,16,65,91,47,32,9,3,73,29,25,93,29,18,88,45,41,46,12,94,13,89,5,36,94,88,33,10,10,2,52,90,19,63,26,84,12,76,16,42,75,63,39,32,72,72,84,70,2,63,33,74,43,68,38,84,72,44,89,18,24,78,69,4,80,41,54,75,72,4,16,91,5,48,30,64,38,4,52,38,30,95,99,32,38,52,35,58,71,38,89,86,25,84,88,41,39,32,56,79,12,52,19,80,46,66,38,32,69,67,6,87,88,36,59,51,5,33,46,45,82,15,57,80,91,12,86,29,34,15,61,19,73,46,82,60,73,13,52,36,67,3,49,87,39,12,98,58,87,32,82,47,65,6,87,71,13,17,65,69,14,34,42,82,42,1,77,63,10,63,28,90,24,13,99,19,38,68,62,44,2,65,81,95,7,54,24,58,16,58,48,95,9,80,9,51,73,23,96,49,64,58,1,6,72,69,39,2,10,63,36,9,85,59,90,41,2,72,77,23,23,80,75,33,6,20,18,59,39,36,89,35,89,42,42,22,37,24,30,51,53,43,78,48,27,76,84,22,81,72,25,95,28,15,51,58,48,7,1,90,72,19,37,52,60,39,81,20,70,6,39,82,26,77,14,96,52,30,84,33,66,80,5,52,15,72,46,55,2,21,8,97,79,43,8,91,27,67,5,18,74,71,34,51,6,83,25,52,92,5,15,85,11,72,33,85,30,59,6,84,29,51,77,99,43,95,44,83,95,89,27,54,16,85,90,82,34,98,59,87,12,73,25,74,29,95,82,51,5,81,46,51,0,0,21,21,1,10,1,0,0,0,0,0,0"], "252");
    }

    #[test]
    fn test_part_two() {
        let solver = Day15Solver {};
        test_solver(&solver, true, &["3,1033,1008,1033,1,1032,1005,1032,31,1008,1033,2,1032,1005,1032,58,1008,1033,3,1032,1005,1032,81,1008,1033,4,1032,1005,1032,104,99,1002,1034,1,1039,1001,1036,0,1041,1001,1035,-1,1040,1008,1038,0,1043,102,-1,1043,1032,1,1037,1032,1042,1105,1,124,102,1,1034,1039,1001,1036,0,1041,1001,1035,1,1040,1008,1038,0,1043,1,1037,1038,1042,1106,0,124,1001,1034,-1,1039,1008,1036,0,1041,1002,1035,1,1040,1002,1038,1,1043,101,0,1037,1042,1106,0,124,1001,1034,1,1039,1008,1036,0,1041,101,0,1035,1040,1002,1038,1,1043,101,0,1037,1042,1006,1039,217,1006,1040,217,1008,1039,40,1032,1005,1032,217,1008,1040,40,1032,1005,1032,217,1008,1039,37,1032,1006,1032,165,1008,1040,9,1032,1006,1032,165,1101,2,0,1044,1105,1,224,2,1041,1043,1032,1006,1032,179,1101,1,0,1044,1106,0,224,1,1041,1043,1032,1006,1032,217,1,1042,1043,1032,1001,1032,-1,1032,1002,1032,39,1032,1,1032,1039,1032,101,-1,1032,1032,101,252,1032,211,1007,0,50,1044,1106,0,224,1102,0,1,1044,1105,1,224,1006,1044,247,1001,1039,0,1034,102,1,1040,1035,102,1,1041,1036,101,0,1043,1038,102,1,1042,1037,4,1044,1106,0,0,37,22,74,27,37,99,30,8,72,31,49,29,51,32,85,21,39,72,2,2,43,94,31,11,76,43,95,21,38,8,90,13,39,97,54,47,14,6,20,49,5,30,97,9,99,64,71,24,36,87,52,94,36,18,52,42,83,38,98,53,26,87,69,32,18,94,2,93,97,15,65,65,21,40,99,19,91,13,4,89,38,70,65,41,73,49,62,54,37,46,14,49,88,86,13,89,23,89,10,3,48,57,92,43,65,4,35,97,48,10,19,64,3,79,38,87,6,13,71,49,74,43,92,8,4,71,6,35,85,98,94,6,38,59,80,65,46,62,63,62,49,61,68,6,7,64,66,40,56,82,59,30,85,45,57,36,86,70,25,83,31,96,65,19,16,67,55,36,49,54,29,75,69,3,3,37,75,49,23,65,22,6,52,75,31,7,87,85,19,48,97,65,51,78,10,35,40,59,54,14,85,6,30,94,68,42,87,46,75,26,82,36,21,65,90,16,59,14,76,55,37,41,99,80,9,79,12,59,17,75,2,40,52,45,76,45,16,82,13,55,61,14,11,49,97,81,99,38,35,20,98,51,64,13,24,85,94,38,25,87,1,42,89,18,32,54,55,17,15,84,98,25,31,21,55,44,57,59,11,78,49,72,87,20,7,33,91,80,75,18,33,37,52,7,26,87,65,36,52,92,6,8,95,89,37,38,57,25,23,71,75,47,20,87,90,37,54,38,77,32,39,67,16,69,62,15,96,47,91,95,18,96,24,45,21,64,9,72,2,54,65,39,36,54,23,71,74,18,26,97,35,44,29,87,54,48,31,55,33,85,74,13,99,82,39,35,97,43,20,62,58,86,98,41,47,92,79,74,10,85,28,66,86,18,35,5,84,67,13,91,47,44,1,84,56,32,96,7,77,21,88,92,38,31,65,82,87,45,55,4,60,58,64,49,53,3,63,32,52,43,10,66,75,96,53,11,95,44,36,16,65,91,47,32,9,3,73,29,25,93,29,18,88,45,41,46,12,94,13,89,5,36,94,88,33,10,10,2,52,90,19,63,26,84,12,76,16,42,75,63,39,32,72,72,84,70,2,63,33,74,43,68,38,84,72,44,89,18,24,78,69,4,80,41,54,75,72,4,16,91,5,48,30,64,38,4,52,38,30,95,99,32,38,52,35,58,71,38,89,86,25,84,88,41,39,32,56,79,12,52,19,80,46,66,38,32,69,67,6,87,88,36,59,51,5,33,46,45,82,15,57,80,91,12,86,29,34,15,61,19,73,46,82,60,73,13,52,36,67,3,49,87,39,12,98,58,87,32,82,47,65,6,87,71,13,17,65,69,14,34,42,82,42,1,77,63,10,63,28,90,24,13,99,19,38,68,62,44,2,65,81,95,7,54,24,58,16,58,48,95,9,80,9,51,73,23,96,49,64,58,1,6,72,69,39,2,10,63,36,9,85,59,90,41,2,72,77,23,23,80,75,33,6,20,18,59,39,36,89,35,89,42,42,22,37,24,30,51,53,43,78,48,27,76,84,22,81,72,25,95,28,15,51,58,48,7,1,90,72,19,37,52,60,39,81,20,70,6,39,82,26,77,14,96,52,30,84,33,66,80,5,52,15,72,46,55,2,21,8,97,79,43,8,91,27,67,5,18,74,71,34,51,6,83,25,52,92,5,15,85,11,72,33,85,30,59,6,84,29,51,77,99,43,95,44,83,95,89,27,54,16,85,90,82,34,98,59,87,12,73,25,74,29,95,82,51,5,81,46,51,0,0,21,21,1,10,1,0,0,0,0,0,0"], "350");
    }



}