use solver::Solver;

pub struct DayXXSolver {}

impl DayXXSolver {
    pub fn new(input: &'static str) -> Self {
        DayXXSolver {}
    }
}

impl Solver for DayXXSolver {
    fn solve_part_one(&self) -> String {
        "".to_string()
    }

    fn solve_part_two(&self) -> String {
        "".to_string()
    }

    fn day_number(&self) -> usize {
        0
    }

    fn description(&self) -> &'static str {
        ""
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = DayXXSolver::new("abc").solve_part_one();
        assert_eq!(result, "0");
    }
}

mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = DayXXSolver::new("abc").solve_part_two();
        assert_eq!(result, "0");
    }
}
