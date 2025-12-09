use solver::SolverBase;

pub struct Solver {}

impl Solver {
    pub fn new(input: &'static str) -> Self {
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
        6
    }

    fn description(&self) -> &'static str {
        "Trash Compactor"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ",
        )
        .solve_part_one();
        assert_eq!(result, "4277556");
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
