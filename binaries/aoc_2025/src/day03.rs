use solver::SolverBase;

pub struct Solver {
    battery_joltage: Vec<Vec<u32>>,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        let battery_joltage = input
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();
        Solver { battery_joltage }
    }

    fn find_largest_joltage(bank: &[u32]) -> u32 {
        let (first_max_index, first_max_value) = bank[..bank.len() - 1]
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.cmp(b.1).then_with(|| b.0.cmp(&a.0)))
            .unwrap();
        let second_max_value = bank[first_max_index + 1..bank.len()].iter().max().unwrap();
        *first_max_value * 10 + *second_max_value
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        self.battery_joltage
            .iter()
            .map(|bank| Solver::find_largest_joltage(bank))
            .sum::<u32>()
            .to_string()
    }

    fn solve_part_two(&self) -> String {
        "".to_string()
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

// #[cfg(test)]
// mod part2_tests {
//     use super::*;

//     #[test]
//     fn test_1() {
//         let result = Solver::new("abc").solve_part_two();
//         assert_eq!(result, "0");
//     }
// }
