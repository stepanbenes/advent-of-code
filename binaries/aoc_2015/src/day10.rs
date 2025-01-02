use solver::Solver;

pub struct Day10Solver {
    input: &'static str,
    repeat_count_part1: usize,
    repeat_count_part2: usize,
}

impl Day10Solver {
    pub fn new(input: &'static str, repeat_count_part1: usize, repeat_count_part2: usize) -> Self {
        Day10Solver {
            input,
            repeat_count_part1,
            repeat_count_part2,
        }
    }

    fn look_and_say(text: &str) -> String {
        let mut chars = text.chars().peekable();
        let mut result = String::new();
        while let Some(c) = chars.next() {
            let mut count = 1;
            while chars.peek() == Some(&c) {
                _ = chars.next();
                count += 1;
            }
            result.push_str(&format!("{}{}", count, c));
        }
        result
    }

    fn look_and_say_repeat(text: &str, repeat_count: usize) -> String {
        let mut text = text.to_owned();
        for _ in 0..repeat_count {
            text = Day10Solver::look_and_say(&text);
        }
        text
    }
}

impl Solver for Day10Solver {
    fn solve_part_one(&self) -> String {
        let result = Day10Solver::look_and_say_repeat(self.input, self.repeat_count_part1);
        result.len().to_string()
    }

    fn solve_part_two(&self) -> String {
        let result = Day10Solver::look_and_say_repeat(self.input, self.repeat_count_part2);
        result.len().to_string()
    }

    fn day_number(&self) -> usize {
        10
    }

    fn description(&self) -> &'static str {
        "Look and say"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Day10Solver::look_and_say_repeat("1", 1);
        assert_eq!(result, "11");
    }

    #[test]
    fn test_2() {
        let result = Day10Solver::look_and_say_repeat("11", 1);
        assert_eq!(result, "21");
    }

    #[test]
    fn test_3() {
        let result = Day10Solver::look_and_say_repeat("21", 1);
        assert_eq!(result, "1211");
    }

    #[test]
    fn test_4() {
        let result = Day10Solver::look_and_say_repeat("1211", 1);
        assert_eq!(result, "111221");
    }

    #[test]
    fn test_5() {
        let result = Day10Solver::look_and_say_repeat("111221", 1);
        assert_eq!(result, "312211");
    }

    #[test]
    fn test_6() {
        let result = Day10Solver::look_and_say_repeat("1", 5);
        assert_eq!(result, "312211");
    }

    #[test]
    fn test_7() {
        let result = Day10Solver::new("1", 5, 0).solve_part_one();
        assert_eq!(result, "6");
    }
}

// #[cfg(test)]
// mod part2_tests {
//     use super::*;

//     #[test]
//     fn test_1() {
//         let result = Day10Solver::new("abc").solve_part_two();
//         assert_eq!(result, "0");
//     }
// }
