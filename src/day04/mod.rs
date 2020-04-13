



use crate::lib::Solver;

pub(crate) struct Day4Solver {}

fn get_i(a: i32, i: i32) -> i32 {
    if i == 0 {
        return a % 10;
    }

    let divisor = 10_i32.pow(i as u32);
    let b = a / divisor;
    return b % 10;
}

fn check_number(a: i32) -> bool {
    let mut found_pair = false;
    for i in (1..=5).rev() {
        let first = get_i(a, i);
        let second = get_i(a, i - 1);
        if !found_pair && first == second {
            let before = if (i-2) >= 0 {
                get_i(a, i-2)
            } else {
                -1
            };

            let after = if (i+1) <= 5 {
                get_i(a, i+1)
            } else {
                -1
            };

            if before != first && after != second {
                found_pair = true;
            }
        }

        if first > second {
            return false;
        }
    }
    return found_pair
}

impl Solver for Day4Solver {
    fn solve(&self, _lines: Vec<String>, _part_two: bool) -> String {
        let mut count = 0;
        // 372304-847060
        for a in 372304..=847060 {
            if check_number(a) {
                count = count + 1;
            }
        }


        return count.to_string();
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::test_solver;


    #[test]
    fn test_check() {
        assert_eq!(check_number(111111), false);
        assert_eq!(check_number(223450), false);
        assert_eq!(check_number(123789), false);
        assert_eq!(check_number(122345), true);
        assert_eq!(check_number(123444), false);
        assert_eq!(check_number(112233), true);
        assert_eq!(check_number(111122), true);
        assert_eq!(check_number(223333), true);
        assert_eq!(check_number(788889), false);

    }
}




