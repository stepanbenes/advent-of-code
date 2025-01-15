use solver::SolverBase;

pub struct Solver {
    triangles: Vec<(u32, u32, u32)>,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        let triangles = input
            .lines()
            .map(|line| {
                let mut sides = line.split_whitespace().map(|s| s.parse().unwrap());
                (
                    sides.next().unwrap(),
                    sides.next().unwrap(),
                    sides.next().unwrap(),
                )
            })
            .collect();
        Solver { triangles }
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let valid_triangles = self
            .triangles
            .iter()
            .filter(|(a, b, c)| (*a + *b) > *c && (*a + *c) > *b && (*b + *c) > *a)
            .count();
        valid_triangles.to_string()
    }

    fn solve_part_two(&self) -> String {
        "".to_string()
    }

    fn day_number(&self) -> usize {
        3
    }

    fn description(&self) -> &'static str {
        "Triangles"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new("5 10 25").solve_part_one();
        assert_eq!(result, "0");
    }

    #[test]
    fn test_2() {
        let result = Solver::new("18 10 25").solve_part_one();
        assert_eq!(result, "1");
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
