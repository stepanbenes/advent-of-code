use solver::SolverBase;

pub struct Solver {
    input: &'static str,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        Solver { input }
    }

    fn get_first_char_of_hashed_word_with_index(
        word: &str,
        start_index: u32,
    ) -> (char, char, Option<usize>, u32) {
        let mut index = start_index;
        loop {
            let test = format!("{word}{index}");
            let hash = md5::compute(test);
            let hash_hex = format!("{:x}", hash);
            if hash_hex.starts_with("00000") {
                let c1 = hash_hex.chars().nth(5).unwrap();
                let c2 = hash_hex.chars().nth(6).unwrap();
                let mut position = None;
                if let Ok(x) = hash_hex[5..6].parse::<usize>() {
                    if x <= 7 {
                        position = Some(x);
                    }
                }
                return (c1, c2, position, index);
            }
            index += 1;
        }
    }

    fn set_char_at_index(s: &str, index: usize, new_char: char) -> String {
        // Convert the string into a vector of characters
        let mut chars: Vec<char> = s.chars().collect();

        // Replace the character at the specified index
        if index < chars.len() {
            chars[index] = new_char;
        } else {
            panic!("Index out of bounds");
        }

        // Reconstruct the string from the characters
        chars.into_iter().collect()
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let mut index = 0;
        let mut password = String::new();
        for _ in 0..8 {
            let (c, _c2, _position, current_index) =
                Solver::get_first_char_of_hashed_word_with_index(self.input, index);
            password.push(c);
            index = current_index + 1;
        }
        password
    }

    fn solve_part_two(&self) -> String {
        let mut index = 0;
        let mut password = "________".to_owned();
        loop {
            let (_c1, c2, position, current_index) =
                Solver::get_first_char_of_hashed_word_with_index(self.input, index);
            if let Some(position) = position {
                if password.chars().nth(position).unwrap() == '_' {
                    password = Solver::set_char_at_index(&password, position, c2);
                    println!("{password}");
                    if password.chars().filter(|x| *x != '_').count() == 8 {
                        break;
                    }
                }
            }
            index = current_index + 1;
        }
        password
    }

    fn day_number(&self) -> usize {
        5
    }

    fn description(&self) -> &'static str {
        "MD5 hashed password"
    }

    fn skip_run(&self) -> bool {
        true
    }
}

#[cfg(test)]
#[cfg(feature = "skip_tests")]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let (c, _, _, index) = Solver::get_first_char_of_hashed_word_with_index("abc", 0);
        assert_eq!(index, 3231929);
        assert_eq!(c, '1');
    }

    #[test]
    fn test_2() {
        let (c, _, _, index) = Solver::get_first_char_of_hashed_word_with_index("abc", 5017308);
        assert_eq!(index, 5017308);
        assert_eq!(c, '8');
    }

    #[test]
    fn test_3() {
        let (c, _, _, index) = Solver::get_first_char_of_hashed_word_with_index("abc", 5017308 + 1);
        assert_eq!(index, 5278568);
        assert_eq!(c, 'f');
    }

    #[test]
    fn test_4() {
        let result = Solver::new("abc").solve_part_one();
        assert_eq!(result, "18f47a30");
    }
}
#[cfg(test)]
#[cfg(feature = "skip_tests")]
mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new("abc").solve_part_two();
        assert_eq!(result, "05ace8e3");
    }
}
