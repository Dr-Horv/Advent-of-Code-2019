use crate::lib::Solver;

pub(crate) struct Day16Solver {}

struct FftPattern {
    index: i32,
    position: i32,
}


impl Iterator for FftPattern {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        let n = (self.index / self.position) % 4;
        let next = match n {
            0 => 0,
            1 => 1,
            2 => 0,
            3 => -1,
            _ => panic!("Nope")
        };

        self.index = self.index+1;
        Some(next)
    }
}


fn fft_pattern(position: i32) -> FftPattern {
    FftPattern { index: 1, position }
}

fn do_phase(digits: Vec<i32>) -> Vec<i32> {
    let mut next = Vec::with_capacity(digits.len());
    for i1 in 0..digits.len() {
        let mut sequence = fft_pattern((i1+1) as i32);
        let mut sum: i32 = 0;
        for i2 in 0..digits.len() {
            let factor = sequence.next().unwrap();
            let digit = digits[i2];
            sum += digit * factor;
        }
        next.push(sum.abs() % 10);
    }

    return next;
}

fn do_partial_phase(digits: Vec<i32>) -> Vec<i32> {
    let mut next = Vec::with_capacity(digits.len());
    let mut sum = 0;
    for i1 in (0..digits.len()).rev() {
        sum += digits[i1];
        next.push(sum % 10);
    }
    next.reverse();
    return next;
}

impl Solver for Day16Solver {
    fn solve(&self, lines: Vec<String>, part_two: bool) -> String {
        const RADIX: u32 = 10;
        let mut digits: Vec<i32> = lines[0]
            .chars()
            .map(|c| c.to_digit(RADIX).unwrap() as i32)
            .collect();

        if !part_two {
            for _ in 0..100 {
                digits = do_phase(digits);
            }

            return digits.iter()
                .take(8)
                .map(|d| d.to_string())
                .collect::<Vec<String>>()
                .join("");
        }


        let mut repeated_digits = Vec::new();
        for _ in 0..10_000  {
            repeated_digits.extend(digits.clone());
        }

        println!("Repeating done: {}", repeated_digits.len());
        let offset: usize = repeated_digits.iter()
            .take(7)
            .map(|d| d.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse()
            .unwrap();

        repeated_digits = repeated_digits.iter().skip(offset).map(|d|*d).collect();
        for _ in 0..100 {
            repeated_digits = do_partial_phase(repeated_digits);
        }

        return repeated_digits.iter()
            .take(8)
            .map(|d| d.to_string())
            .collect::<Vec<String>>()
            .join("");



    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::test_solver;

    fn ftt_pattern_to_vec(position: i32, size: usize) -> Vec<i32> {
        let vec: Vec<i32> = fft_pattern(position).take(size).collect();
        return vec;
    }

    #[test]
    fn test_fft_pattern() {
        assert_eq!(ftt_pattern_to_vec(1, 10), vec![1, 0, -1, 0, 1, 0, -1, 0, 1, 0]);
        assert_eq!(ftt_pattern_to_vec(2, 15), vec![0, 1, 1, 0, 0, -1, -1, 0, 0, 1, 1, 0, 0, -1, -1]);
        assert_eq!(ftt_pattern_to_vec(3, 11), vec![0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1]);
    }

    #[test]
    fn test_part_one() {
        let solver = Day16Solver {};
        test_solver(&solver, false, &["80871224585914546619083218645595"], "24176176");
        test_solver(&solver, false, &["19617804207202209144916044189917"], "73745418");
        test_solver(&solver, false, &["69317163492948606335995924319873"], "52432133");
    }

    #[test]
    fn test_part_two() {
        let solver = Day16Solver {};
        test_solver(&solver, true, &["03036732577212944063491565474664"], "84462026");
        test_solver(&solver, true, &["02935109699940807407585447034323"], "78725270");
        test_solver(&solver, true, &["03081770884921959731165446850517"], "53553731");
    }



}