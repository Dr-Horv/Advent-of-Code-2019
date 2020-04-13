use std::cmp::{max, min};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::mpsc;
use std::thread;

use regex::Regex;

use crate::lib::Solver;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Reaction {
    quantity: i32,
    chemical: String,
    dependencies: Vec<Box<Dependency>>,
}


#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Dependency {
    quantity: i32,
    reaction: Reaction,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Chemical {
    quantity: i32,
    name: String,
}

pub(crate) struct Day14Solver {}

fn parse_chemical(s: String) -> Chemical {
    let parts: Vec<String> = s.trim()
        .split(" ")
        .map(|s| s.to_string())
        .collect();
    let quantity = parts[0].parse().unwrap();
    let name = parts[1].trim().to_string();
    return Chemical { quantity, name }
}

fn parse_first_step(s: String) -> Box<(Vec<Chemical>, Chemical)> {
    let parts: Vec<String> = s.split("=>")
        .map(|s| s.to_string())
        .collect();
    let inputs = parts[0].split(",")
        .map(|cs| parse_chemical(cs.to_string()))
        .collect();

    let output = parse_chemical(parts[1].to_string());

    return Box::new((inputs, output))
}

fn has_all_dependencies(dependencies: &Vec<Chemical>, reaction_lookup: &HashMap<String, Reaction>) -> bool {
    return dependencies.iter()
        .all(|c| reaction_lookup.contains_key(c.name.as_str()));
}

fn get_dependencies(dependencies: &Vec<Chemical>, reaction_lookup: &HashMap<String, Reaction>) -> Vec<Box<Dependency>> {
    let reactions = dependencies.iter()
        .map(|c| {
            let r = reaction_lookup.get(c.name.as_str()).unwrap().clone();
            let d = Dependency { quantity: c.quantity, reaction: r };
            return Box::new(d);
        })
        .collect();

    // println!("{:?}", reactions);
    return reactions;
}

fn calculate_ore_needed(reaction: &Reaction, mut stash: HashMap<String, i32>, mut ores_left: i64) -> (i32, HashMap<String, i32>, i64) {
    if reaction.dependencies.len() == 1 && reaction.dependencies.get(0).unwrap().reaction.chemical == "ORE".to_string() {
        // stash.insert(reaction.chemical.clone(), reaction.quantity);
        // println!("Happens");
        let ores_consumed = reaction.dependencies.get(0).unwrap().quantity as i64;
        if ores_consumed > ores_left {
            return (-1, stash, 0);
        }

        return (ores_consumed as i32, stash, ores_left - ores_consumed);
    }

    //println!("{:?} {:?} {:?}", reaction.chemical, depth, stash);

    let mut sum = 0;
    for d in &reaction.dependencies {
        let mut needed = d.quantity;
        while needed > 0 {
            if stash.contains_key(d.reaction.chemical.as_str()) {
                let mut stashed = *stash.get(d.reaction.chemical.as_str()).unwrap();
                let subtract = if stashed >= needed {
                    needed
                } else {
                    stashed
                };
                let stashed_quantity = stashed - subtract;
                needed -= subtract;
                stash.insert(d.reaction.chemical.clone(), stashed_quantity);
            }
            if needed > 0 {
                let (ore, mut s, ol) = calculate_ore_needed(&d.reaction, stash, ores_left);
                if ore < 0 {
                    return (sum, s, 0)
                }
                s.insert(d.reaction.chemical.clone(), d.reaction.quantity);
                // println!("{:?} {:?} {:?}", reaction.chemical, sum, stash);
                sum += ore;
                stash = s;
                ores_left = ol
            }
        }
        // println!("Need {:?} {:?} {:?}", needed, d.reaction.chemical, stash);
    }

    if reaction.chemical == "FUEL".to_string() {
        let f = *stash.get("FUEL").unwrap_or(&0);
        stash.insert("FUEL".to_string(), f+1);
    }
    //println!("{:?} {:?} {:?}", reaction.chemical, sum, stash);
    return (sum, stash, ores_left);
}

impl Solver for Day14Solver {
    fn solve(&self, lines: Vec<String>, part_two: bool) -> String {
        let mut unparsed = VecDeque::new();
        for l in lines {
            let o = parse_first_step(l);
            unparsed.push_front(o);
        }

        let mut reaction_lookup = HashMap::new();
        let r = Reaction { quantity: 1, chemical: "ORE".to_string(), dependencies: vec![] };
        reaction_lookup.insert("ORE".to_string(), r);
        while !unparsed.is_empty() {
            let b = unparsed.pop_front().unwrap();
            let (inputs, output) = *b;

            if has_all_dependencies(&inputs, &reaction_lookup) {
                let dependencies = get_dependencies(&inputs, &reaction_lookup);
                let r = Reaction { quantity: output.quantity, chemical: output.name.to_string(), dependencies };
                // println!("{:?}", r);
                reaction_lookup.insert(output.name, r);
            } else {
                unparsed.push_back(Box::new((inputs, output)));
            }
        }

        let fuel = reaction_lookup.get("FUEL").unwrap();
        let mut prev_fuel = 0;
        let mut new_fuel = 1;
        let mut stash = HashMap::new();
        let mut ores_left: i64 = 1000000000000;
        let mut it = 0;
        loop {
            let (ore_needed, updated_stash, updated_ore_left) = calculate_ore_needed(fuel, stash, ores_left);
            println!("{:?}", updated_stash);
            if !part_two {
                return ore_needed.to_string()
            }

            prev_fuel = new_fuel;
            new_fuel = *updated_stash.get("FUEL").unwrap();
            ores_left = updated_ore_left;
            stash = updated_stash;
            if ores_left <= 0 {
                return new_fuel.to_string();
            }
            it += 1;
            if it % 10_000 == 0 {
                println!("{:?}", it);
            }
            if it > 82_892_753 {
                println!("Broken");
                return "-1".to_string();
            }
        }


        /*
        9 ORE => 2 A
        8 ORE => 3 B
        7 ORE => 5 C
        3 A, 4 B => 1 AB
        5 B, 7 C => 1 BC
        4 C, 1 A => 1 CA
        2 AB, 3 BC, 4 CA => 1 FUEL
        */
    }
}

#[cfg(test)]
mod tests {
    use crate::lib::test_solver;

    use super::*;

    #[test]
    fn test_part_one() {
        let solver = Day14Solver {};
        test_solver(&solver, false, &[
            "9 ORE => 2 A",
            "8 ORE => 3 B",
            "7 ORE => 5 C",
            "3 A, 4 B => 1 AB",
            "5 B, 7 C => 1 BC",
            "4 C, 1 A => 1 CA",
            "2 AB, 3 BC, 4 CA => 1 FUEL"
        ], "165");

        test_solver(&solver, false, &[
            "157 ORE => 5 NZVS",
            "165 ORE => 6 DCFZ",
            "44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL",
            "12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ",
            "179 ORE => 7 PSHF",
            "177 ORE => 5 HKGWZ",
            "7 DCFZ, 7 PSHF => 2 XJWVT",
            "165 ORE => 2 GPVTF",
            "3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"
        ], "13312");
    }


    #[test]
    fn test_part_two() {
        let solver = Day14Solver {};
        test_solver(&solver, true, &[
            "157 ORE => 5 NZVS",
            "165 ORE => 6 DCFZ",
            "44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL",
            "12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ",
            "179 ORE => 7 PSHF",
            "177 ORE => 5 HKGWZ",
            "7 DCFZ, 7 PSHF => 2 XJWVT",
            "165 ORE => 2 GPVTF",
            "3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"
        ], "82892753");
    }
}