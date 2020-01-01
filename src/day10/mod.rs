
use std::collections::HashMap;

use crate::lib::{Solver, Position, real_distance};
use std::f64::consts::PI;
use std::ops::{Div, Mul};
use std::cmp::Ordering;

struct Asteroid {
    pub relative_pos: Position,
    pub original_pos: Position,
    pub angle: f64,
}

pub(crate) struct Day10Solver {}

fn has_line_of_sight(p1: &Position, p2: &Position, asteroid_map: &HashMap<Position, bool>) -> bool {
    let relative_pos = Position{x: p1.x - p2.x, y: p1.y - p2.y};
    !asteroid_map.iter()
        .filter( |&(k,_)|  p1 != k && p2 != k)
        .any(|(p, _)| {
            let relative_pos_a = Position{x: p1.x - p.x, y: p1.y - p.y};
            let relative_pos_b = Position{x: p2.x - p.x, y: p2.y - p.y};
            let a = (relative_pos.y as f64).atan2(relative_pos.x as f64);
            let b = (relative_pos_a.y as f64).atan2(relative_pos_a.x as f64);

            ((a - b).abs() < 0.001) && (real_distance(p, p1) < real_distance(p1, p2) && real_distance(p, p2) < real_distance(p1, p2))
        })
}

impl Solver for Day10Solver {
    fn solve(&self, lines: Vec<String>, part_two: bool) -> String {
        let mut asteroid_map = HashMap::new();
        let mut asteroid_map_vision = HashMap::new();
        let height = lines.len();
        let width = lines[0].len();
        for y in 0..lines.len() {
            let line = &lines[y];
            let mut chars = line.chars();
            for x in 0..line.len() {
                let c = chars.next().unwrap();
                if c == '#' {
                    let p = Position{x: x as i32, y: y as i32};
                    asteroid_map.insert(p, true);
                }
            }
        }

        let mut max = 0;
        let mut best = &Position{x: 0,y: 0};
        for (p1, _) in &asteroid_map {
            let mut count = 0;
            for (p2, _) in &asteroid_map {
                if p1 == p2 {
                    continue
                }
                if has_line_of_sight(p1, p2, &asteroid_map) {
                    count += 1
                }
            }

            asteroid_map_vision.insert(p1, count);
            if count > max {
                max = count;
                best = p1;
            }
        }

        let mut map = String::new();
        for y in 0..height {
            map.push_str("\n");
            for x in 0..width {
                let s = match asteroid_map_vision.get(&Position{x: x as i32, y: y as i32}) {
                    Some(t) => t.to_string(),
                    None => String::from(".")
                };
                map.push_str(s.as_str())
            }
        }


        println!("Best={},{} can see {}", best.x, best.y, max);

        if !part_two {
            return max.to_string();
        }


        let mut asteroids: Vec<Asteroid> = asteroid_map.iter()
            .map(|(&a,_)| {
                let relative_pos = Position{x: best.x - a.x, y: best.y - a.y};
                let pos = Position{x: a.x, y: a.y};
                let a = (relative_pos.y as f64).atan2(relative_pos.x as f64);
                let b = if a < 0.0 { a.abs() + PI } else { a };
                let c = a - PI/2.0;
                let d = if c < 0.0 { PI*2.0+c } else { c };
                let angle = d;

                return Asteroid{relative_pos, original_pos: pos, angle };
            }).collect();

        asteroids.sort_by(|a,b| {
            return a.angle.partial_cmp(&b.angle).unwrap()
        });

        let mut a_map_copy = asteroid_map.clone();
        let mut count = 0;
        let mut asteroids_to_remove = Vec::new();
        loop {
            for a in asteroids.iter() {
                if a.original_pos != *best && has_line_of_sight(best, &a.original_pos, &a_map_copy) {
                    count += 1;
                    println!("{} Asteroid={},{} angle {}", count, a.original_pos.x, a.original_pos.y, a.angle);
                    asteroids_to_remove.push(a);
                    if count == 200 {
                        println!("Asteroid={},{} angle {}", a.original_pos.x, a.original_pos.y, a.angle);
                        return (a.original_pos.x * 100 + a.original_pos.y).to_string();
                    }
                }
            }

            for a in asteroids_to_remove.iter_mut() {
                a_map_copy.remove(&a.original_pos);
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
        let solver = Day10Solver {};
        test_solver(&solver, false, &[".#..#", ".....", "#####", "....#", "...##"], "8");
        test_solver(&solver, false, &["......#.#.",
            "#..#.#....",
            "..#######.",
            ".#.#.###..",
            ".#..#.....",
            "..#....#.#",
            "#..#....#.",
            ".##.#..###",
            "##...#..#.",
            ".#....####"
        ], "33");
        test_solver(&solver, false, &["#.#...#.#.",
            ".###....#.",
            ".#....#...",
            "##.#.#.#.#",
            "....#.#.#.",
            ".##..###.#",
            "..#...##..",
            "..##....##",
            "......#...",
            ".####.###."], "35");




        test_solver(&solver, false, &[".#..##.###...#######",
            "##.############..##.",
            ".#.######.########.#",
            ".###.#######.####.#.",
            "#####.##.#.##.###.##",
            "..#####..#.#########",
            "####################",
            "#.####....###.#.#.##",
            "##.#################",
            "#####.##.###..####..",
            "..######..##.#######",
            "####.##.####...##..#",
            ".#####..#.######.###",
            "##...#.##########...",
            "#.##########.#######",
            ".####.#.###.###.#.##",
            "....##.##.###..#####",
            ".#.#.###########.###",
            "#.#.#.#####.####.###",
            "###.##.####.##.#..##"], "210");
    }

    #[test]
    fn test_part_two() {

        let solver = Day10Solver {};
        test_solver(&solver, true, &[".#..##.###...#######",
            "##.############..##.",
            ".#.######.########.#",
            ".###.#######.####.#.",
            "#####.##.#.##.###.##",
            "..#####..#.#########",
            "####################",
            "#.####....###.#.#.##",
            "##.#################",
            "#####.##.###..####..",
            "..######..##.#######",
            "####.##.####...##..#",
            ".#####..#.######.###",
            "##...#.##########...",
            "#.##########.#######",
            ".####.#.###.###.#.##",
            "....##.##.###..#####",
            ".#.#.###########.###",
            "#.#.#.#####.####.###",
            "###.##.####.##.#..##"], "802");
    }
}