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

    fn decompress(slice: &str) -> usize {
        let mut decompressed_length = 0;
        for (i, c) in slice.chars().enumerate() {
            match c {
                '(' => {
                    let (length, repeat) = Solver::parse_marker(&mut slice[i..].chars());
                    let sub_decompressed_length = Solver::decompress(&slice[i..i + length]);
                    decompressed_length += sub_decompressed_length * repeat;
                }
                _ => { fxffg
                    decompressed_length += 1;
                }
            }
        }
        decompressed_length
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
        Solver::decompress(&self.input).to_string()
    }

    fn day_number(&self) -> usize {
        9
    }

    fn description(&self) -> &'static str {
        "String decompression"
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
        let result = Solver::new("(3x3)XYZ").solve_part_two();
        assert_eq!(result, "XYZXYZXYZ".len().to_string());
    }

    #[test]
    fn test_2() {
        let result = Solver::new("X(8x2)(3x3)ABCY").solve_part_two();
        assert_eq!(result, "XABCABCABCABCABCABCY".len().to_string());
    }

    #[test]
    fn test_3() {
        let result = Solver::new("(27x12)(20x12)(13x14)(7x10)(1x12)A").solve_part_two();
        assert_eq!(result, "241920");
    }

    #[test]
    fn test_4() {
        let result = Solver::new("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN")
            .solve_part_two();
        assert_eq!(result, "445");
    }
}
