use solver::Solver;

pub struct Day08Solver {
    input: &'static str,
}

impl Day08Solver {
    pub fn new(input: &'static str) -> Self {
        Day08Solver { input }
    }
}

impl Solver for Day08Solver {
    fn solve_part_one(&self) -> String {
        let mut code_char_count = 0;
        let mut memory_char_count = 0;
        for line in self.input.lines() {
            let bytes = line.as_bytes();
            let length = bytes.len();
            code_char_count += length;
            let mut escape_mode = false;
            let mut i = 1;
            while i < length - 1 {
                if escape_mode {
                    match bytes[i] {
                        b'\\' | b'\"' => {
                            escape_mode = false;
                            memory_char_count += 1;
                        }
                        b'x' => {
                            i += 2;
                            escape_mode = false;
                            memory_char_count += 1;
                        }
                        _ => panic!("unsupported escape character"),
                    }
                } else if bytes[i] == b'\\' {
                    escape_mode = true;
                } else {
                    memory_char_count += 1;
                }
                i += 1;
            }
        }
        (code_char_count - memory_char_count).to_string()
    }

    fn solve_part_two(&self) -> String {
        "".to_string()
    }

    fn day_number(&self) -> usize {
        8
    }

    fn description(&self) -> &'static str {
        "String literals"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Day08Solver::new(
            r#"""
"#,
        )
        .solve_part_one();
        assert_eq!(result, "2");
    }

    #[test]
    fn test_2() {
        let result = Day08Solver::new(
            r#""abc"
"#,
        )
        .solve_part_one();
        assert_eq!(result, "2");
    }

    #[test]
    fn test_3() {
        let result = Day08Solver::new(
            r#""aaa\"aaa"
"#,
        )
        .solve_part_one();
        assert_eq!(result, "3");
    }

    #[test]
    fn test_4() {
        let result = Day08Solver::new(
            r#""\x27"
"#,
        )
        .solve_part_one();
        assert_eq!(result, "5");
    }

    #[test]
    fn test_5() {
        let result = Day08Solver::new(
            r#"""
"abc"
"aaa\"aaa"
"\x27"
"#,
        )
        .solve_part_one();
        assert_eq!(result, "12");
    }
}

// #[cfg(test)]
// mod part2_tests {
//     use super::*;

//     #[test]
//     fn test_1() {
//         let result = Day08Solver::new("abc").solve_part_two();
//         assert_eq!(result, "0");
//     }
// }
