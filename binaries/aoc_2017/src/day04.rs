use std::collections::HashSet;

use solver::SolverBase;

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
        let mut counter = 0;
        for line in self.input.lines() {
            let mut dictionary = HashSet::new();
            let mut is_valid = true;
            for word in line.split_whitespace() {
                if !dictionary.insert(word) {
                    is_valid = false;
                    break;
                }
            }
            if is_valid {
                counter += 1;
            }
        }
        counter.to_string()
    }

    fn solve_part_two(&self) -> String {
        let mut counter = 0;
        for line in self.input.lines() {
            let mut dictionary = HashSet::new();
            let mut is_valid = true;
            for word in line.split_whitespace() {
                let mut word_chars = word.chars().collect::<Vec<_>>();
                word_chars.sort();
                let sorted_word = word_chars.iter().collect::<String>();
                if !dictionary.insert(sorted_word) {
                    is_valid = false;
                    break;
                }
            }
            if is_valid {
                counter += 1;
            }
        }
        counter.to_string()
    }

    fn day_number(&self) -> usize {
        4
    }

    fn description(&self) -> &'static str {
        "Passphrases"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new("aa bb cc dd ee").solve_part_one();
        assert_eq!(result, "1");
    }

    #[test]
    fn test_2() {
        let result = Solver::new("aa bb cc dd aa").solve_part_one();
        assert_eq!(result, "0");
    }

    #[test]
    fn test_3() {
        let result = Solver::new("aa bb cc dd ee").solve_part_one();
        assert_eq!(result, "1");
    }

    #[test]
    fn test_4() {
        let result = Solver::new("aa bb cc dd aaa").solve_part_one();
        assert_eq!(result, "1");
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new("abcde fghij").solve_part_two();
        assert_eq!(result, "1");
    }

    #[test]
    fn test_2() {
        let result = Solver::new("abcde xyz ecdab").solve_part_two();
        assert_eq!(result, "0");
    }

    #[test]
    fn test_3() {
        let result = Solver::new("a ab abc abd abf abj").solve_part_two();
        assert_eq!(result, "1");
    }

    #[test]
    fn test_4() {
        let result = Solver::new("iiii oiii ooii oooi oooo").solve_part_two();
        assert_eq!(result, "1");
    }

    #[test]
    fn test_5() {
        let result = Solver::new("oiii ioii iioi iiio").solve_part_two();
        assert_eq!(result, "0");
    }
}
