use itertools::Itertools;
use solver::SolverBase;

pub struct Solver {
    input: Vec<u8>,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        Solver {
            input: input
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect(),
        }
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let mut sum = 0;
        let mut updater = |a: u8, b: u8| {
            if a == b {
                sum += a as u32;
            }
        };
        for (&a, &b) in self.input.iter().tuple_windows() {
            updater(a, b);
        }
        updater(*self.input.last().unwrap(), *self.input.first().unwrap());
        sum.to_string()
    }

    fn solve_part_two(&self) -> String {
        "".to_string()
    }

    fn day_number(&self) -> usize {
        1
    }

    fn description(&self) -> &'static str {
        "Captcha"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new("1122").solve_part_one();
        assert_eq!(result, "3");
    }

    #[test]
    fn test_2() {
        let result = Solver::new("1111").solve_part_one();
        assert_eq!(result, "4");
    }

    #[test]
    fn test_3() {
        let result = Solver::new("1234").solve_part_one();
        assert_eq!(result, "0");
    }

    #[test]
    fn test_4() {
        let result = Solver::new("91212129").solve_part_one();
        assert_eq!(result, "9");
    }
}
