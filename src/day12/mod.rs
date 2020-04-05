use crate::lib::Solver;
extern crate regex;

use regex::Regex;
use core::fmt;
use std::collections::{HashSet, HashMap};

pub(crate) struct Day12Solver {}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
    z: i32
}
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Velocity {
    x: i32,
    y: i32,
    z: i32
}
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Moon {
    id: i8,
    position: Position,
    velocity: Velocity
}


impl Moon {
    fn apply_velocity(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }
}

impl fmt::Display for Moon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pos=<x={}, y={}, z={}>, vel=<x={}, y={}, z={}>", self.position.x, self.position.y, self.position.z, self.velocity.x, self.velocity.y,self.velocity.z )
    }
}

fn simulate_step(moons: &mut Vec<Moon>) {
    for i in 0..moons.len() {
        for j in 0..moons.len() {
            if i == j {
                continue
            }
            let m2 = moons[j];
            let mut m1 = &mut moons[i];

            if m1.position.x > m2.position.x {
                m1.velocity.x = m1.velocity.x - 1;
            } else if m1.position.x < m2.position.x {
                m1.velocity.x = m1.velocity.x + 1;
            }
            if m1.position.y > m2.position.y {
                m1.velocity.y = m1.velocity.y - 1;
            } else if m1.position.y < m2.position.y {
                m1.velocity.y = m1.velocity.y + 1;
            }
            if m1.position.z > m2.position.z {
                m1.velocity.z = m1.velocity.z - 1;
            } else if m1.position.z < m2.position.z {
                m1.velocity.z = m1.velocity.z + 1;
            }
        }
    }
    for i in 0..moons.len() {
        moons[i].apply_velocity();
    }
}

fn calculate_energy(moons: &Vec<Moon>) -> i32 {
    return moons.iter()
        .map( |m| {
            let potential_energy = m.position.x.abs() + m.position.y.abs() + m.position.z.abs();
            let kinetic_energy = m.velocity.x.abs() + m.velocity.y.abs() + m.velocity.z.abs();
            return potential_energy * kinetic_energy;
        })
        .sum();
}

impl Solver for Day12Solver {
    fn solve(&self, lines: Vec<String>, part_two: bool) -> String {
        let regex = Regex::new(r"(?m)<x=(-*\d*),\sy=(-*\d*),\sz=(-*\d*)>").unwrap();

        let mut id = 0;
        let mut moons: Vec<Moon> = lines.iter()
            .map(|l| regex.captures(l.as_str()).unwrap())
            .map(|caps| {

                let parse = |i: usize| -> i32 {
                    return caps.get(i) .map_or("", |m| m.as_str())
                        .parse::<i32>().unwrap()
                };
                let x = parse(1);
                let y = parse(2);
                let z = parse(3);
                let p =  Position { x, y, z };
                let v = Velocity { x: 0, y: 0, z: 0 };
                let m = Moon{id, position: p, velocity:v };
                id += 1;
                return m;
            }).collect();

        if !part_two {
            for step in 0..1000 {
                simulate_step(&mut moons)
            }

            let energy = calculate_energy(&moons);

            return energy.to_string();
        }

        let mut moon_x_states: HashSet<Vec<(i32, i32)>> = HashSet::new();
        let mut moon_y_states: HashSet<Vec<(i32, i32)>>  = HashSet::new();
        let mut moon_z_states: HashSet<Vec<(i32, i32)>>  = HashSet::new();
        let mut moon_x_loop_time= 0;
        let mut moon_y_loop_time= 0;
        let mut moon_z_loop_time= 0;
        moon_x_states.insert(moons.iter().map(|m| (m.position.x, m.velocity.x)).collect());
        moon_y_states.insert(moons.iter().map(|m| (m.position.y, m.velocity.y)).collect());
        moon_z_states.insert(moons.iter().map(|m| (m.position.z, m.velocity.z)).collect());
        let mut step = 0;
        loop {
            if moon_x_loop_time != 0 && moon_y_loop_time != 0 && moon_z_loop_time != 0 {
                break;
            }

            simulate_step(&mut moons);
            step += 1;
            let xs: Vec<(i32, i32)> = moons.iter().map(|m| (m.position.x, m.velocity.x)).collect();
            let ys: Vec<(i32, i32)> = moons.iter().map(|m| (m.position.y, m.velocity.y)).collect();
            let zs: Vec<(i32, i32)> = moons.iter().map(|m| (m.position.z, m.velocity.z)).collect();
            if moon_x_loop_time == 0 && moon_x_states.contains(&xs) {
                moon_x_loop_time = step;
            }
            if moon_y_loop_time == 0 && moon_y_states.contains(&ys) {
                moon_y_loop_time = step;
            }
            if moon_z_loop_time == 0 && moon_z_states.contains(&zs) {
                moon_z_loop_time = step;
            }

            if step % 10_000 == 0 {
                println!("Step: {}", step)
            }
        }

        let vec = vec![moon_x_loop_time, moon_y_loop_time, moon_z_loop_time];
        let total_lcm = lcm(vec);

        return total_lcm.to_string();

    }
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    let mut t = b;
    loop {
        if b == 0 {
            return a;
        }
        t = b;
        b = a % b;
        a = t;
    }
}

fn lcm(numbers: Vec<u128>) -> u128 {
    let mut queue = numbers.clone();
    let mut a = queue.pop().unwrap();
    loop {
        if queue.is_empty() {
            return a;
        }
        let b = queue.pop().unwrap();
        let nom = a;
        let denom = gcd(a, b);
        a = (nom/denom) * b;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::test_solver;

    #[test]
    fn test_part_one() {
        let solver = Day12Solver {};
        test_solver(&solver, false, &[
            "<x=0, y=6, z=1>",
            "<x=4, y=4, z=19>",
            "<x=-11, y=1, z=8>",
            "<x=2, y=19, z=15>"], "14809");
    }

    /*
    #[test]
    fn test_part_two() {
        let solver = Day11Solver {};
        test_solver(&solver, true, &["3,8,1005,8,306,1106,0,11,0,0,0,104,1,104,0,3,8,1002,8,-1,10,1001,10,1,10,4,10,108,1,8,10,4,10,1002,8,1,28,2,107,3,10,1,101,19,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,0,10,4,10,102,1,8,59,2,5,13,10,3,8,102,-1,8,10,1001,10,1,10,4,10,1008,8,0,10,4,10,1001,8,0,85,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,1,10,4,10,1001,8,0,107,1006,0,43,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,1,10,4,10,101,0,8,132,3,8,102,-1,8,10,1001,10,1,10,4,10,1008,8,0,10,4,10,1001,8,0,154,2,4,1,10,2,4,9,10,3,8,1002,8,-1,10,101,1,10,10,4,10,108,0,8,10,4,10,1001,8,0,183,1,1102,5,10,1,1102,1,10,1006,0,90,2,9,12,10,3,8,102,-1,8,10,1001,10,1,10,4,10,1008,8,0,10,4,10,1001,8,0,221,1006,0,76,1006,0,27,1,102,9,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,108,1,8,10,4,10,102,1,8,252,2,4,9,10,1006,0,66,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,1,10,4,10,101,0,8,282,1,102,19,10,101,1,9,9,1007,9,952,10,1005,10,15,99,109,628,104,0,104,1,21102,1,387240010644,1,21101,0,323,0,1105,1,427,21102,846541370112,1,1,21101,334,0,0,1106,0,427,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,21102,3425718295,1,1,21102,381,1,0,1105,1,427,21102,179410541715,1,1,21101,0,392,0,1106,0,427,3,10,104,0,104,0,3,10,104,0,104,0,21101,0,718078255872,1,21101,0,415,0,1105,1,427,21102,1,868494234468,1,21102,1,426,0,1105,1,427,99,109,2,21202,-1,1,1,21101,0,40,2,21101,458,0,3,21101,0,448,0,1106,0,491,109,-2,2106,0,0,0,1,0,0,1,109,2,3,10,204,-1,1001,453,454,469,4,0,1001,453,1,453,108,4,453,10,1006,10,485,1102,0,1,453,109,-2,2105,1,0,0,109,4,2102,1,-1,490,1207,-3,0,10,1006,10,508,21102,1,0,-3,22102,1,-3,1,22101,0,-2,2,21102,1,1,3,21102,1,527,0,1106,0,532,109,-4,2105,1,0,109,5,1207,-3,1,10,1006,10,555,2207,-4,-2,10,1006,10,555,22101,0,-4,-4,1105,1,623,22101,0,-4,1,21201,-3,-1,2,21202,-2,2,3,21101,574,0,0,1105,1,532,21202,1,1,-4,21102,1,1,-1,2207,-4,-2,10,1006,10,593,21102,0,1,-1,22202,-2,-1,-2,2107,0,-3,10,1006,10,615,21201,-1,0,1,21101,615,0,0,106,0,490,21202,-2,-1,-2,22201,-4,-2,-4,109,-5,2105,1,0"], "\n.  .  #  #  .  .  .  #  #  .  .  #  #  #  .  .  #  #  #  .  .  #  .  .  #  .  #  #  #  #  .  #  .  .  #  .  #  .  .  .  .  .  .  \n.  #  .  .  #  .  #  .  .  #  .  #  .  .  #  .  #  .  .  #  .  #  .  #  .  .  .  .  .  #  .  #  .  .  #  .  #  .  .  .  .  .  .  \n.  #  .  .  .  .  #  .  .  #  .  #  .  .  #  .  #  .  .  #  .  #  #  .  .  .  .  .  #  .  .  #  .  .  #  .  #  .  .  .  .  .  .  \n.  #  .  #  #  .  #  #  #  #  .  #  #  #  .  .  #  #  #  .  .  #  .  #  .  .  .  #  .  .  .  #  .  .  #  .  #  .  .  .  .  .  .  \n.  #  .  .  #  .  #  .  .  #  .  #  .  #  .  .  #  .  .  .  .  #  .  #  .  .  #  .  .  .  .  #  .  .  #  .  #  .  .  .  .  >  .  \n.  .  #  #  #  .  #  .  .  #  .  #  .  .  #  .  #  .  .  .  .  #  .  .  #  .  #  #  #  #  .  .  #  #  .  .  #  #  #  #  .  .  .  \n");
    }*/
}




