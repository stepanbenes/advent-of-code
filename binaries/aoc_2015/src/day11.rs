use std::collections::HashSet;

use itertools::Itertools;
use solver::SolverBase;

pub struct Solver {
    current_password: &'static str,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        Solver {
            current_password: input,
        }
    }

    fn is_valid_password(text: &str) -> bool {
        // Passwords may not contain the letters i, o, or l, as these letters can be mistaken for other characters and are therefore confusing.
        for c in text.chars() {
            if let 'i' | 'o' | 'l' = c {
                return false;
            }
        }
        // Passwords must include one increasing straight of at least three letters, like abc, bcd, cde, and so on, up to xyz. They cannot skip letters; abd doesn't count.
        let mut contains_increasing_straight = false;
        for (c1, c2, c3) in text.bytes().tuple_windows() {
            if c1 + 1 == c2 && c2 + 1 == c3 {
                contains_increasing_straight = true;
                break;
            }
        }
        // Passwords must contain at least two different, non-overlapping pairs of letters, like aa, bb, or zz.
        let mut pairs_indices = HashSet::new();
        for (i, (c1, c2)) in text.bytes().tuple_windows().enumerate() {
            if c1 == c2 && !pairs_indices.contains(&(i - 1)) {
                pairs_indices.insert(i);
            }
        }
        contains_increasing_straight && pairs_indices.len() >= 2
    }

    fn increment_password(text: &str) -> String {
        let mut new_password = String::new();
        let mut carry = true;
        for c in text.bytes().rev() {
            let next_byte = if carry { c + 1 } else { c };
            if next_byte <= b'z' {
                new_password.insert(0, next_byte as char);
                carry = false;
            } else {
                new_password.insert(0, 'a');
            }
        }
        if carry {
            new_password.insert(0, 'a');
        }
        new_password
    }

    fn generate_next_password(password: &str) -> String {
        let mut password = password.to_owned();
        loop {
            password = Solver::increment_password(&password);
            if Solver::is_valid_password(&password) {
                break;
            }
        }
        password
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        Solver::generate_next_password(self.current_password)
    }

    fn solve_part_two(&self) -> String {
        Solver::generate_next_password(&Solver::generate_next_password(self.current_password))
    }

    fn day_number(&self) -> usize {
        11
    }

    fn description(&self) -> &'static str {
        "Next password"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new("abcdefgh").solve_part_one();
        assert_eq!(result, "abcdffaa");
    }

    #[test]
    fn test_2() {
        let result = Solver::new("ghijklmn").solve_part_one();
        assert_eq!(result, "ghjaabcc");
    }

    #[test]
    fn test_3() {
        let result = Solver::is_valid_password("hijklmmn");
        assert_eq!(result, false);
    }

    #[test]
    fn test_4() {
        let result = Solver::is_valid_password("abbceffg");
        assert_eq!(result, false);
    }

    #[test]
    fn test_5() {
        let result = Solver::is_valid_password("abbcegjk");
        assert_eq!(result, false);
    }

    #[test]
    fn test_6() {
        let result = Solver::is_valid_password("abcdefgh");
        assert_eq!(result, false);
    }

    #[test]
    fn test_7() {
        let result = Solver::is_valid_password("abcdffaa");
        assert_eq!(result, true);
    }

    #[test]
    fn test_8() {
        let result = Solver::is_valid_password("ghijklmn");
        assert_eq!(result, false);
    }

    #[test]
    fn test_9() {
        let result = Solver::is_valid_password("ghjaabcc");
        assert_eq!(result, true);
    }

    #[test]
    fn test_10() {
        let result = Solver::increment_password("ghjaabcc");
        assert_eq!(result, "ghjaabcd");
    }
    #[test]
    fn test_11() {
        let result = Solver::increment_password("ghjaazzz");
        assert_eq!(result, "ghjabaaa");
    }

    #[test]
    fn test_12() {
        let result = Solver::increment_password("zzz");
        assert_eq!(result, "aaaa");
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
