use solver::SolverBase;

pub struct Solver {
    input: &'static str,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        Solver { input }
    }

    fn consume_chars(iter: &mut impl Iterator<Item = char>, length: usize) {
        for _ in 0..length {
            _ = iter.next().unwrap();
        }
    }

    fn parse_marker(iter: &mut impl Iterator<Item = char>) -> (usize, usize) {
        let mut length_str = String::new();
        let mut repeat_str = String::new();
        let mut reading_repeat = false;
        for c in iter.by_ref() {
            match c {
                ')' => break,
                'x' => reading_repeat = true,
                _ => {
                    if reading_repeat {
                        repeat_str.push(c);
                    } else {
                        length_str.push(c);
                    }
                }
            }
        }
        (length_str.parse().unwrap(), repeat_str.parse().unwrap())
    }

    fn decompress(iter: &mut impl Iterator<Item = char>) -> String {
        let mut decompressed_text = String::new();
        while let Some(c) = iter.next() {
            match c {
                '(' => {
                    let (length, repeat) = Solver::parse_marker(iter);
                    let mut substring = String::new();
                    for _ in 0..length {
                        substring.push(iter.next().unwrap());
                    }
                    let sub_decompressed_text = Solver::decompress(&mut substring.chars());
                    for _ in 0..repeat {
                        decompressed_text.push_str(&sub_decompressed_text);
                    }
                }
                _ => {
                    decompressed_text.push(c);
                }
            }
        }
        decompressed_text
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let mut char_iter = self.input.chars();
        let mut decompressed_length = 0;
        while let Some(c) = char_iter.next() {
            match c {
                '(' => {
                    let (length, repeat) = Solver::parse_marker(&mut char_iter);
                    Solver::consume_chars(&mut char_iter, length);
                    decompressed_length += length * repeat;
                }
                _ => decompressed_length += 1,
            }
        }
        decompressed_length.to_string()
    }

    fn solve_part_two(&self) -> String {
        let text = Solver::decompress(&mut self.input.chars());
        text.len().to_string()
    }

    fn day_number(&self) -> usize {
        9
    }

    fn description(&self) -> &'static str {
        "String decompression"
    }

    fn skip_run(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new("ADVENT").solve_part_one();
        assert_eq!(result, "6");
    }

    #[test]
    fn test_2() {
        let result = Solver::new("A(1x5)BC").solve_part_one();
        assert_eq!(result, "7");
    }

    #[test]
    fn test_3() {
        let result = Solver::new("(3x3)XYZ").solve_part_one();
        assert_eq!(result, "9");
    }

    #[test]
    fn test_4() {
        let result = Solver::new("A(2x2)BCD(2x2)EFG").solve_part_one();
        assert_eq!(result, "11");
    }

    #[test]
    fn test_5() {
        let result = Solver::new("(6x1)(1x3)A").solve_part_one();
        assert_eq!(result, "6");
    }

    #[test]
    fn test_6() {
        let result = Solver::new("X(8x2)(3x3)ABCY").solve_part_one();
        assert_eq!(result, "18");
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::decompress(&mut "(3x3)XYZ".chars());
        assert_eq!(result, "XYZXYZXYZ");
    }

    #[test]
    fn test_2() {
        let result = Solver::decompress(&mut "X(8x2)(3x3)ABCY".chars());
        assert_eq!(result, "XABCABCABCABCABCABCY");
    }

    #[test]
    fn test_3() {
        let result = Solver::decompress(&mut "(6x4)(1x3)A".chars());
        assert_eq!(result, "AAAAAAAAAAAA");
    }
}
