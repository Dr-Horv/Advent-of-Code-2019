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


    #[test]
    fn test_part_two() {
        let solver = Day12Solver {};
        test_solver(&solver, true, &[
            "<x=0, y=6, z=1>",
            "<x=4, y=4, z=19>",
            "<x=-11, y=1, z=8>",
            "<x=2, y=19, z=15>"], "282270365571288");}
}




