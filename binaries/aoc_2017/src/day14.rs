use solver::SolverBase;
use std::fmt::Write;

use crate::day10;

pub struct Solver {
    input: &'static str,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        Solver { input }
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        #[allow(dead_code)]
        fn hex_to_binary(hex: &str) -> String {
            // hex.chars()
            //     .map(|c| format!("{:04b}", c.to_digit(16).unwrap()))
            //     .collect()

            hex.chars().fold(String::new(), |mut acc, c| {
                _ = write!(&mut acc, "{:04b}", c.to_digit(16).unwrap());
                acc
            })
        }

        fn count_ones_in_hex(hex: &str) -> usize {
            hex.chars()
                .map(|c| c.to_digit(16).unwrap().count_ones() as usize)
                .sum()
        }

        let mut count_of_used_squares = 0;

        for i in 0..128 {
            let hasher_input = format!("{}-{}", self.input, i);
            //println!("{hasher_input}");
            let knot_hash = day10::Solver::new(hasher_input, 256).full_knot_hash();
            //println!("{knot_hash}");
            //let binary_string = hex_to_binary(&knot_hash);
            //println!("{binary_string}");
            let sum = count_ones_in_hex(&knot_hash);
            count_of_used_squares += sum;
        }

        count_of_used_squares.to_string()
    }

    fn solve_part_two(&self) -> String {
        "".to_string()
    }

    fn day_number(&self) -> usize {
        14
    }

    fn description(&self) -> &'static str {
        "Disk defragmentation - knot hash reuse"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new("flqrgnkx").solve_part_one();
        assert_eq!(result, "8108");
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
