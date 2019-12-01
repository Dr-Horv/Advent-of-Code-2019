
use crate::lib::Solver;

pub(crate) struct Day1Solver {}

impl Day1Solver {
    fn calculate_fuel(&self, weight: i32) -> i32 {
        (weight as f64 / 3.0) as i32 - 2
    }
}

impl Solver for Day1Solver {
    fn solve(&self, lines: Vec<String>, part_two: bool) -> String {
        if !part_two {
            return lines
                .into_iter()
                .map(|s| s.parse::<i32>().unwrap())
                .map(|i| self.calculate_fuel(i))
                .sum::<i32>().to_string()
        }

        let mut total = 0;
        for weight in lines.iter().map(|s| s.parse::<i32>().unwrap()) {
            let mut fuel_to_add = self.calculate_fuel(weight);
            while fuel_to_add > 0 {
                total += fuel_to_add;
                fuel_to_add = self.calculate_fuel(fuel_to_add);
            }
        }

        return total.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::test_solver;


    #[test]
    fn test_part_one() {
        let solver = Day1Solver{};
        test_solver(&solver, false, &["12"], "2");
        test_solver(&solver, false, &["14"], "2");
        test_solver(&solver, false, &["1969"], "654");
        test_solver(&solver, false, &["100756"], "33583");
    }

    #[test]
    fn test_part_two() {
        let solver = Day1Solver{};

        test_solver(&solver, true, &["14"], "2");
        test_solver(&solver, true, &["1969"], "966");
        test_solver(&solver, true, &["100756"], "50346");
    }

}
