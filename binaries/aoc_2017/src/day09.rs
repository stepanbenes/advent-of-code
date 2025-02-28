use solver::SolverBase;

pub struct Solver {
    input: &'static str,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        Solver { input }
    }

    fn get_group_score(&self) -> u32 {
        let mut is_escape = false;
        let mut is_garbage = false;
        let mut depth = 0;
        let mut score = 0;
        for c in self.input.chars() {
            if !is_escape {
                match c {
                    '{' if !is_garbage => {
                        depth += 1;
                        score += depth;
                    }
                    '}' if !is_garbage => {
                        depth -= 1;
                    }
                    '<' => {
                        is_garbage = true;
                    }
                    '>' => {
                        is_garbage = false;
                    }
                    '!' => {
                        is_escape = true;
                    }
                    _ => {}
                }
            } else {
                is_escape = false;
            }
        }
        score
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        self.get_group_score().to_string()
    }

    fn solve_part_two(&self) -> String {
        "".to_string()
    }

    fn day_number(&self) -> usize {
        9
    }

    fn description(&self) -> &'static str {
        "Groups and garbage"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new("{}").solve_part_one();
        assert_eq!(result, "1");
    }

    #[test]
    fn test_2() {
        let result = Solver::new("{{{}}}").solve_part_one();
        assert_eq!(result, "6");
    }

    #[test]
    fn test_3() {
        let result = Solver::new("{{},{}}").solve_part_one();
        assert_eq!(result, "5");
    }

    #[test]
    fn test_4() {
        let result = Solver::new("{{{},{},{{}}}}").solve_part_one();
        assert_eq!(result, "16");
    }

    #[test]
    fn test_5() {
        let result = Solver::new("{<a>,<a>,<a>,<a>}").solve_part_one();
        assert_eq!(result, "1");
    }

    #[test]
    fn test_6() {
        let result = Solver::new("{{<ab>},{<ab>},{<ab>},{<ab>}}").solve_part_one();
        assert_eq!(result, "9");
    }

    #[test]
    fn test_7() {
        let result = Solver::new("{{<!!>},{<!!>},{<!!>},{<!!>}}").solve_part_one();
        assert_eq!(result, "9");
    }

    #[test]
    fn test_8() {
        let result = Solver::new("{{<a!>},{<a!>},{<a!>},{<ab>}}").solve_part_one();
        assert_eq!(result, "3");
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
