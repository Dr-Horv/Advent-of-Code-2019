use std::collections::HashMap;

use crate::lib::Solver;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Reaction {
    quantity: i64,
    chemical: String,
    dependencies: Vec<Dependency>,
}


#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Dependency {
    quantity: i64,
    chemical: String,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Chemical {
    quantity: i64,
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

fn calculate_ore(amount: i64, reaction: &Reaction, reaction_lookup: &HashMap<String, Reaction>) -> i64 {
    let mut stash: HashMap<String, i64> = HashMap::new();
    let stash_ref = &mut stash;
    let (_, ore_needed) = calculate_ore_helper(amount, reaction, stash_ref, reaction_lookup, 0, );
    return ore_needed;
}

fn calculate_ore_helper<'a>(amount: i64,
                            reaction: &Reaction,
                            mut stash: &'a mut HashMap<String, i64>,
                            reaction_lookup: &HashMap<String, Reaction>,
                            mut ore_used: i64
) -> (&'a mut HashMap<String, i64>, i64) {
    let reactions_needed = if amount % reaction.quantity != 0 {
        amount/reaction.quantity + 1
    } else {
        amount/reaction.quantity
    };

    for d in &reaction.dependencies {
        if d.chemical == "ORE".to_string() {
            let ou = ore_used + (reactions_needed * d.quantity);
            stash.insert(reaction.chemical.clone(), reactions_needed * reaction.quantity);
            return (stash, ou);
        }

        let mut needed = reactions_needed * d.quantity;
        if stash.contains_key(d.chemical.as_str()) {
            let stashed = stash[d.chemical.as_str()];
            let subtract = if stashed >= needed {
                needed
            } else {
                stashed
            };
            stash.insert(d.chemical.clone(), stashed-subtract);
            needed -= subtract;
        }
        if needed > 0 {
            let (s, ou) = calculate_ore_helper(
                needed,
                &reaction_lookup[d.chemical.as_str()],
                stash,
                reaction_lookup,
                ore_used
            );
            let stashed = s[d.chemical.as_str()];
            s.insert(d.chemical.clone(), stashed-needed);
            stash = s;
            ore_used = ou;
        }
    }

    stash.insert(reaction.chemical.clone(), reactions_needed * reaction.quantity);
    return (stash, ore_used)
}


impl Solver for Day14Solver {
    fn solve(&self, lines: Vec<String>, part_two: bool) -> String {
        let mut reaction_lookup = HashMap::new();
        let r = Reaction { quantity: 1, chemical: "ORE".to_string(), dependencies: vec![] };
        reaction_lookup.insert("ORE".to_string(), r);
        for l in lines {
            let (inputs, output) = *parse_first_step(l);
            let dependencies = inputs.iter()
                .map(|c| Dependency { quantity: c.quantity, chemical: c.name.clone() })
                .collect();
            let reaction = Reaction { quantity: output.quantity, chemical: output.name, dependencies };
            reaction_lookup.insert(reaction.chemical.clone(), reaction.clone());
        }

        let start = reaction_lookup.get("FUEL").unwrap();

        let ore_needed = calculate_ore(1, start, &reaction_lookup);
        if !part_two {
            return ore_needed.to_string();
        }

        let target = 1000000000000;
        let mut guess = target / ore_needed;
        let mut min = guess / 2;
        let mut max = guess * 2;
        let mut old_guess = -1;
        let mut it = 0;
        loop {
            let ore_needed = calculate_ore(guess, start, &reaction_lookup);
            if ore_needed < target {
                min = guess;
            } else if ore_needed > target {
                max = guess;
            }
            guess = (min + max) / 2;
            if guess == old_guess {
                break;
            }
            old_guess = guess;
            it += 1;
            if it > 100 {
                break;
            }
        }
        return guess.to_string();
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
            "157 ORE => 5 N",
            "165 ORE => 6 D",
            "44 X, 5 K, 1 Q, 29 N, 9 G, 48 H => 1 FUEL",
            "12 H, 1 G, 8 P => 9 Q",
            "179 ORE => 7 P",
            "177 ORE => 5 H",
            "7 D, 7 P => 2 X",
            "165 ORE => 2 G",
            "3 D, 7 N, 5 H, 10 P => 8 K"
        ], "13312");

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

    /**
    2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
    17 NVRVD, 3 JNWZP => 8 VPVL
    53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
    22 VJHF, 37 MNCFX => 5 FWMGM
    139 ORE => 4 NVRVD
    144 ORE => 7 JNWZP
    5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
    5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
    145 ORE => 6 MNCFX
    1 NVRVD => 8 CXFTF
    1 VJHF, 6 MNCFX => 4 RFSQX
    176 ORE => 6 VJHF

    */
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

        test_solver(&solver, true, &[
            "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG",
            "17 NVRVD, 3 JNWZP => 8 VPVL",
            "53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL",
            "22 VJHF, 37 MNCFX => 5 FWMGM",
            "139 ORE => 4 NVRVD",
            "144 ORE => 7 JNWZP",
            "5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC",
            "5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV",
            "145 ORE => 6 MNCFX",
            "1 NVRVD => 8 CXFTF",
            "1 VJHF, 6 MNCFX => 4 RFSQX",
            "176 ORE => 6 VJHF",
        ], "5586022");

        test_solver(&solver, true, &[
            "171 ORE => 8 CNZTR",
            "7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL",
            "114 ORE => 4 BHXH",
            "14 VRPVC => 6 BMBT",
            "6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL",
            "6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT",
            "15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW",
            "13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW",
            "5 BMBT => 4 WPTQ",
            "189 ORE => 9 KTJDG",
            "1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP",
            "12 VRPVC, 27 CNZTR => 2 XDBXC",
            "15 KTJDG, 12 BHXH => 5 XCVML",
            "3 BHXH, 2 VRPVC => 7 MZWV",
            "121 ORE => 7 VRPVC",
            "7 XCVML => 6 RJRHP",
            "5 BHXH, 4 VRPVC => 5 LTCX",
        ], "460664");
    }
}