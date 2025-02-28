use solver::SolverBase;

pub struct Solver {
    input: &'static str,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        Solver { input }
    }

    fn get_group_score(&self) -> (u32, u32) {
        let mut is_escape = false;
        let mut is_garbage = false;
        let mut depth = 0;
        let mut score = 0;
        let mut garbage_counter = 0;
        for c in self.input.chars() {
            if !is_escape {
                match c {
                    '{' if !is_garbage => {
                        depth += 1;
                        score += depth;
                    }
                    '{' => {
                        garbage_counter += 1;
                    }
                    '}' if !is_garbage => {
                        depth -= 1;
                    }
                    '}' => {
                        garbage_counter += 1;
                    }
                    '<' if !is_garbage => {
                        is_garbage = true;
                    }
                    '<' => {
                        garbage_counter += 1;
                    }
                    '>' => {
                        is_garbage = false;
                    }
                    '!' => {
                        is_escape = true;
                    }
                    _ if !is_garbage => {}
                    _ => {
                        garbage_counter += 1;
                    }
                }
            } else {
                is_escape = false;
            }
        }
        (score, garbage_counter)
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let (score, _) = self.get_group_score();
        score.to_string()
    }

    fn solve_part_two(&self) -> String {
        let (_, garbage_counter) = self.get_group_score();
        garbage_counter.to_string()
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

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new("<>").solve_part_two();
        assert_eq!(result, "0");
    }

    #[test]
    fn test_2() {
        let result = Solver::new("<random characters>").solve_part_two();
        assert_eq!(result, "17");
    }

    #[test]
    fn test_3() {
        let result = Solver::new("<<<<>").solve_part_two();
        assert_eq!(result, "3");
    }

    #[test]
    fn test_4() {
        let result = Solver::new("<{!>}>").solve_part_two();
        assert_eq!(result, "2");
    }

    #[test]
    fn test_5() {
        let result = Solver::new("<!!>").solve_part_two();
        assert_eq!(result, "0");
    }

    #[test]
    fn test_6() {
        let result = Solver::new("<!!!>>").solve_part_two();
        assert_eq!(result, "0");
    }

    #[test]
    fn test_7() {
        let result = Solver::new(r#"<{o"i!a,<{i<a>"#).solve_part_two();
        assert_eq!(result, "10");
    }
}
