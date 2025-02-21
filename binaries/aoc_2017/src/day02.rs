use itertools::Itertools;
use solver::SolverBase;

pub struct Solver {
    spreadsheet: Vec<Vec<i32>>,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        let spreadsheet = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|token| token.parse().unwrap())
                    .collect()
            })
            .collect();
        Solver { spreadsheet }
    }

    fn get_even_division(numbers: &[i32]) -> Option<i32> {
        for (&a, &b) in numbers.iter().tuple_combinations() {
            if a % b == 0 {
                return Some(a / b);
            }
            if b % a == 0 {
                return Some(b / a);
            }
        }
        None
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let mut checksum = 0;
        for line in self.spreadsheet.iter() {
            let minmax = line.iter().minmax();
            let (min, max) = minmax.into_option().unwrap();
            let diff = max - min;
            checksum += diff;
        }
        checksum.to_string()
    }

    fn solve_part_two(&self) -> String {
        let mut checksum = 0;
        for line in self.spreadsheet.iter() {
            let div_result = Solver::get_even_division(line).unwrap();
            checksum += div_result;
        }
        checksum.to_string()
    }

    fn day_number(&self) -> usize {
        2
    }

    fn description(&self) -> &'static str {
        "Checksum"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r"5 1 9 5
7 5 3
2 4 6 8",
        )
        .solve_part_one();
        assert_eq!(result, "18");
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r"5 9 2 8
9 4 7 3
3 8 6 5",
        )
        .solve_part_two();
        assert_eq!(result, "9");
    }
}
