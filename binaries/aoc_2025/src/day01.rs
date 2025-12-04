use solver::SolverBase;

pub struct Solver {}

impl Solver {
    pub fn new(_input: &'static str) -> Self {
        Solver {}
    }
}

impl SolverBase for Solver {
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
        let result = Solver::new("abc").solve_part_one();
        assert_eq!(result, "");
    }
}

// #[cfg(test)]
// mod part2_tests {
//     use super::*;

//     #[test]
//     fn test_1() {
//         let result = Solver::new("abc").solve_part_two();
//         assert_eq!(result, "");
//     }
// }
