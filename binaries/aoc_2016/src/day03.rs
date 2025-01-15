use itertools::Itertools;
use solver::SolverBase;

pub struct Solver {
    triangles: Vec<(u32, u32, u32)>,
    triangles_vertically: Vec<(u32, u32, u32)>,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        fn get_sides(line: &str) -> (u32, u32, u32) {
            let mut sides = line.split_whitespace().map(|s| s.parse().unwrap());
            (
                sides.next().unwrap(),
                sides.next().unwrap(),
                sides.next().unwrap(),
            )
        }

        let triangles = input.lines().map(get_sides).collect();
        let mut triangles_vertically = Vec::new();
        for triplet in input.lines().chunks(3).into_iter() {
            let sides: Vec<_> = triplet.map(get_sides).collect();
            if sides.len() == 3 {
                triangles_vertically.push((sides[0].0, sides[1].0, sides[2].0));
                triangles_vertically.push((sides[0].1, sides[1].1, sides[2].1));
                triangles_vertically.push((sides[0].2, sides[1].2, sides[2].2));
            }
        }
        Solver {
            triangles,
            triangles_vertically,
        }
    }

    fn get_count_of_valid_triangles(triangles: &[(u32, u32, u32)]) -> usize {
        triangles
            .iter()
            .filter(|(a, b, c)| (*a + *b) > *c && (*a + *c) > *b && (*b + *c) > *a)
            .count()
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        Solver::get_count_of_valid_triangles(&self.triangles).to_string()
    }

    fn solve_part_two(&self) -> String {
        Solver::get_count_of_valid_triangles(&self.triangles_vertically).to_string()
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
