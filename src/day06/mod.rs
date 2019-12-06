use std::collections::HashMap;
use crate::lib::Solver;

pub(crate) struct Day6Solver {}

fn calculate_orbits(k: &String, goal: &String, galaxy: &HashMap<String, String>) -> Vec<String> {
    let mut curr = k;
    let mut v = Vec::new();
    while curr.to_string() != goal.to_string() {
        // println!("{}", k);
        v.push(curr.to_string());
        curr = galaxy.get(curr).unwrap();
    }
    v.push(curr.to_string());
    return v;
}

impl Solver for Day6Solver {
    fn solve(&self, lines: Vec<String>, part_two: bool) -> String {
        let mut galaxy_map: HashMap<String, String> = HashMap::new();

        lines.into_iter()
            .map(|s| {
                let parts = s.split(')').map(|p| p.to_string()).collect::<Vec<String>>();
                return parts;
            })
            .for_each(|parts| {
                galaxy_map.insert(parts[1].to_string(), parts[0].to_string());
            });
        //println!("{:?}", galaxy_map);

        if !part_two {
            let mut total_orbits = 0;
            for k in galaxy_map.keys() {
                total_orbits += calculate_orbits(k, &String::from("COM"), &galaxy_map, ).len() as i32;

            }
            return total_orbits.to_string();
        }

        let santa_way = calculate_orbits(&"SAN".to_string(), &String::from("COM"),&galaxy_map);
        let you_way = calculate_orbits(&"YOU".to_string(), &String::from("COM"), &galaxy_map);

        for k in santa_way.iter() {
            if you_way.contains(k) {

                let jumps_for_santa = calculate_orbits(&"SAN".to_string(), k,&galaxy_map).len() - 2;
                let jumps_for_you = calculate_orbits(&"YOU".to_string(), k,&galaxy_map).len() - 2;
                println!("Common {} santa={} you={}", k, jumps_for_santa, jumps_for_you);
                return (jumps_for_santa + jumps_for_you).to_string();
            }
        }

        return String::from("Fail");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::test_solver;

    #[test]
    fn test_part_one() {
        let solver = Day6Solver{};
        test_solver(&solver, false, &["COM)B","B)C","C)D","D)E", "E)F","B)G","G)H","D)I","E)J","J)K","K)L"], "42");
    }

    #[test]
    fn test_part_two() {
       let solver = Day6Solver{};
        test_solver(&solver, true, &["COM)B",
            "B)C",
            "C)D",
            "D)E",
            "E)F",
            "B)G",
            "G)H",
            "D)I",
            "E)J",
            "J)K",
            "K)L",
            "K)YOU",
            "I)SAN"], "4");
    }

}
