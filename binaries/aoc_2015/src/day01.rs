use solver::Solver;

pub struct Day01Solver {
    input: &'static str,
}

impl Day01Solver {
    pub fn new(input: &'static str) -> Self {
        Day01Solver { input }
    }
}

impl Solver for Day01Solver {
    fn solve_part_one(&self) -> String {
        self.input
            .chars()
            .fold(0, |acc, c| match c {
                '(' => acc + 1,
                ')' => acc - 1,
                _ => acc,
            })
            .to_string()
    }

    fn solve_part_two(&self) -> String {
        let mut floor = 0;
        for (i, c) in self.input.chars().enumerate() {
            floor += match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            };
            if floor == -1 {
                return (i + 1).to_string();
            }
        }
        panic!("Basement not reached");
    }

    fn day_number(&self) -> usize {
        1
    }

    fn description(&self) -> &'static str {
        "Balancing parenthesis"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;
    #[test]
    fn floor_0_1() {
        let result = Day01Solver::new("(())").solve_part_one();
        assert_eq!(result, "0");
    }

    #[test]
    fn floor_0_2() {
        let result = Day01Solver::new("()()").solve_part_one();
        assert_eq!(result, "0");
    }

    #[test]
    fn floor_3_1() {
        let result = Day01Solver::new("(((").solve_part_one();
        assert_eq!(result, "3");
    }

    #[test]
    fn floor_3_2() {
        let result = Day01Solver::new("(()(()(").solve_part_one();
        assert_eq!(result, "3");
    }

    #[test]
    fn floor_3_3() {
        let result = Day01Solver::new("))(((((").solve_part_one();
        assert_eq!(result, "3");
    }

    #[test]
    fn floor_neg1_1() {
        let result = Day01Solver::new("())").solve_part_one();
        assert_eq!(result, "-1");
    }

    #[test]
    fn floor_neg1_2() {
        let result = Day01Solver::new("))(").solve_part_one();
        assert_eq!(result, "-1");
    }

    #[test]
    fn floor_neg3_1() {
        let result = Day01Solver::new(")))").solve_part_one();
        assert_eq!(result, "-3");
    }

    #[test]
    fn floor_neg3_2() {
        let result = Day01Solver::new(")())())").solve_part_one();
        assert_eq!(result, "-3");
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;
    #[test]
    fn basement_1() {
        let result = Day01Solver::new(")").solve_part_two();
        assert_eq!(result, "1");
    }

    #[test]
    fn basement_5() {
        let result = Day01Solver::new("()())").solve_part_two();
        assert_eq!(result, "5");
    }
}
