use std::collections::HashMap;

use solver::Solver;

pub struct Day05Solver {
    input: &'static str,
}

impl Day05Solver {
    pub fn new(input: &'static str) -> Self {
        Day05Solver { input }
    }
}

impl Solver for Day05Solver {
    fn solve_part_one(&self) -> String {
        let mut nice_string_counter = 0;
        for s in self.input.lines() {
            let mut vowel_counter = 0;
            let mut previous_char = None;
            let mut contains_double_char = false;
            let mut contains_forbidden_string = false;
            for c in s.chars() {
                if let 'a' | 'e' | 'i' | 'o' | 'u' = c {
                    vowel_counter += 1;
                }
                if let Some(previous_char) = previous_char {
                    if previous_char == c {
                        contains_double_char = true;
                    }
                    if let "ab" | "cd" | "pq" | "xy" = format!("{previous_char}{c}").as_str() {
                        contains_forbidden_string = true;
                        break;
                    }
                }
                previous_char = Some(c);
            }
            if vowel_counter >= 3 && contains_double_char && !contains_forbidden_string {
                nice_string_counter += 1;
            }
        }
        nice_string_counter.to_string()
    }

    fn solve_part_two(&self) -> String {
        let mut nice_string_counter = 0;
        for s in self.input.lines() {
            let mut contains_double_pair = false;
            let mut contains_repeated_letter = false;
            let mut map: HashMap<&str, usize> = HashMap::new();
            for i in 0..s.len() - 1 {
                let window = &s[i..i + 2];
                if let Some(&other_index) = map.get(window) {
                    if other_index < i - 1 {
                        contains_double_pair = true;
                    } else {
                        map.insert(window, i);
                    }
                } else {
                    map.insert(window, i);
                }

                if i > 0 && s[i - 1..i] == s[i + 1..i + 2] {
                    contains_repeated_letter = true;
                }
            }
            if contains_double_pair && contains_repeated_letter {
                nice_string_counter += 1;
            }
        }
        nice_string_counter.to_string()
    }

    fn day_number(&self) -> usize {
        5
    }

    fn description(&self) -> &'static str {
        "Nice strings, naughty strings"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Day05Solver::new("ugknbfddgicrmopn").solve_part_one();
        assert_eq!(result, "1");
    }

    #[test]
    fn test_2() {
        let result = Day05Solver::new("aaa").solve_part_one();
        assert_eq!(result, "1");
    }

    #[test]
    fn test_3() {
        let result = Day05Solver::new("jchzalrnumimnmhp").solve_part_one();
        assert_eq!(result, "0");
    }

    #[test]
    fn test_4() {
        let result = Day05Solver::new("haegwjzuvuyypxyu").solve_part_one();
        assert_eq!(result, "0");
    }

    #[test]
    fn test_5() {
        let result = Day05Solver::new("dvszwmarrgswjxmb").solve_part_one();
        assert_eq!(result, "0");
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Day05Solver::new("qjhvhtzxzqqjkmpb").solve_part_two();
        assert_eq!(result, "1");
    }

    #[test]
    fn test_2() {
        let result = Day05Solver::new("xxyxx").solve_part_two();
        assert_eq!(result, "1");
    }

    #[test]
    fn test_3() {
        let result = Day05Solver::new("uurcxstgmygtbstg").solve_part_two();
        assert_eq!(result, "0");
    }

    #[test]
    fn test_4() {
        let result = Day05Solver::new("ieodomkazucvgmuy").solve_part_two();
        assert_eq!(result, "0");
    }

    #[test]
    fn test_5() {
        let result = Day05Solver::new("aa").solve_part_two();
        assert_eq!(result, "0");
    }
}
