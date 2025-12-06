use solver::SolverBase;

pub struct Solver {
    battery_joltage: Vec<Vec<u64>>,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        let battery_joltage = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u64)
                    .collect()
            })
            .collect();
        Solver { battery_joltage }
    }

    fn find_largest_joltage_two(bank: &[u64]) -> u64 {
        let (first_max_index, first_max_value) = bank[..bank.len() - 1]
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.cmp(b.1).then_with(|| b.0.cmp(&a.0)))
            .unwrap();
        let second_max_value = bank[first_max_index + 1..bank.len()].iter().max().unwrap();
        *first_max_value * 10 + *second_max_value
    }

    fn find_largest_joltage_twelve(bank: &[u64]) -> u64 {
        let mut joltage = 0;
        let mut pivot = 0;
        for i in (0..=11).rev() {
            let (max_index, max_value) = bank[pivot..bank.len() - i]
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.cmp(b.1).then_with(|| b.0.cmp(&a.0)))
                .unwrap();
            joltage = joltage * 10 + max_value;
            pivot += max_index + 1;
        }
        joltage
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        self.battery_joltage
            .iter()
            .map(|bank| Solver::find_largest_joltage_two(bank))
            .sum::<u64>()
            .to_string()
    }

    fn solve_part_two(&self) -> String {
        self.battery_joltage
            .iter()
            .map(|bank| Solver::find_largest_joltage_twelve(bank))
            .sum::<u64>()
            .to_string()
    }

    fn day_number(&self) -> usize {
        3
    }

    fn description(&self) -> &'static str {
        "Lobby"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r"987654321111111
811111111111119
234234234234278
818181911112111",
        )
        .solve_part_one();
        assert_eq!(result, "357");
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r"987654321111111
811111111111119
234234234234278
818181911112111",
        )
        .solve_part_two();
        assert_eq!(result, "3121910778619");
    }
}
