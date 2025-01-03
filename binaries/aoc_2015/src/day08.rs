use solver::SolverBase;

pub struct Solver {
    input: &'static str,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        Solver { input }
    }

    pub fn encode(text: &'static str) -> String {
        let mut encoded_text = String::new();
        encoded_text.push('\"');
        for c in text.chars() {
            match c {
                '\\' => encoded_text.push_str("\\\\"),
                '\"' => encoded_text.push_str("\\\""),
                _ => encoded_text.push(c),
            }
        }
        encoded_text.push('\"');
        encoded_text
    }
}

impl SolverBase for Solver {
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
        let mut old_char_count = 0;
        let mut new_char_count = 0;
        for line in self.input.lines() {
            let bytes = line.as_bytes();
            let length = bytes.len();
            old_char_count += length;

            let encoded_line = Solver::encode(line);
            let bytes = encoded_line.as_bytes();
            let length = bytes.len();
            new_char_count += length;
        }
        (new_char_count - old_char_count).to_string()
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
        let result = Solver::new(
            r#"""
"#,
        )
        .solve_part_one();
        assert_eq!(result, "2");
    }

    #[test]
    fn test_2() {
        let result = Solver::new(
            r#""abc"
"#,
        )
        .solve_part_one();
        assert_eq!(result, "2");
    }

    #[test]
    fn test_3() {
        let result = Solver::new(
            r#""aaa\"aaa"
"#,
        )
        .solve_part_one();
        assert_eq!(result, "3");
    }

    #[test]
    fn test_4() {
        let result = Solver::new(
            r#""\x27"
"#,
        )
        .solve_part_one();
        assert_eq!(result, "5");
    }

    #[test]
    fn test_5() {
        let result = Solver::new(
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

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::encode(r#""""#);
        assert_eq!(result, r#""\"\"""#);
    }

    #[test]
    fn test_2() {
        let result = Solver::encode(r#""abc""#);
        assert_eq!(result, r#""\"abc\"""#);
    }

    #[test]
    fn test_3() {
        let result = Solver::encode(r#""aaa\"aaa""#);
        assert_eq!(result, r#""\"aaa\\\"aaa\"""#);
    }

    #[test]
    fn test_4() {
        let result = Solver::encode(r#""\x27""#);
        assert_eq!(result, r#""\"\\x27\"""#);
    }
}
