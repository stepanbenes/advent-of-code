use solver::SolverBase;

pub struct Solver {
    input: &'static str,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        Solver { input }
    }

    fn get_first_char_of_hashed_word_with_index(word: &str, start_index: u32) -> (char, u32) {
        let mut index = start_index;
        loop {
            let test = format!("{word}{index}");
            let hash = md5::compute(test);
            let hash_hex = format!("{:x}", hash);
            if hash_hex.starts_with("00000") {
                return (hash_hex.chars().nth(5).unwrap(), index);
            }
            index += 1;
        }
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let mut index = 0;
        let mut password = String::new();
        for _ in 0..8 {
            let (c, current_index) =
                Solver::get_first_char_of_hashed_word_with_index(self.input, index);
            password.push(c);
            index = current_index + 1;
        }
        password
    }

    fn solve_part_two(&self) -> String {
        "".to_string()
    }

    fn day_number(&self) -> usize {
        5
    }

    fn description(&self) -> &'static str {
        "MD5 hashed password"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let (c, index) = Solver::get_first_char_of_hashed_word_with_index("abc", 0);
        assert_eq!(index, 3231929);
        assert_eq!(c, '1');
    }

    #[test]
    fn test_2() {
        let (c, index) = Solver::get_first_char_of_hashed_word_with_index("abc", 5017308);
        assert_eq!(index, 5017308);
        assert_eq!(c, '8');
    }

    #[test]
    fn test_3() {
        let (c, index) = Solver::get_first_char_of_hashed_word_with_index("abc", 5017308 + 1);
        assert_eq!(index, 5278568);
        assert_eq!(c, 'f');
    }

    #[test]
    fn test_4() {
        let result = Solver::new("abc").solve_part_one();
        assert_eq!(result, "18f47a30");
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
