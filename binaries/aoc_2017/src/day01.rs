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

    fn get_next_index(&self, index: usize, offset: usize) -> usize {
        (index + offset) % self.input.len()
    }

    fn solve(&self, offset: usize) -> u32 {
        let mut sum = 0;
        let mut updater = |a: u8, b: u8| {
            if a == b {
                sum += a as u32;
            }
        };
        for index in 0..self.input.len() {
            let next_index = self.get_next_index(index, offset);
            let a = self.input[index];
            let b = self.input[next_index];
            updater(a, b);
        }
        sum
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let sum = self.solve(1);
        sum.to_string()
    }

    fn solve_part_two(&self) -> String {
        let sum = self.solve(self.input.len() / 2);
        sum.to_string()
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

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new("1212").solve_part_two();
        assert_eq!(result, "6");
    }

    #[test]
    fn test_2() {
        let result = Solver::new("1221").solve_part_two();
        assert_eq!(result, "0");
    }

    #[test]
    fn test_3() {
        let result = Solver::new("123425").solve_part_two();
        assert_eq!(result, "4");
    }

    #[test]
    fn test_4() {
        let result = Solver::new("123123").solve_part_two();
        assert_eq!(result, "12");
    }

    #[test]
    fn test_5() {
        let result = Solver::new("12131415").solve_part_two();
        assert_eq!(result, "4");
    }
}
