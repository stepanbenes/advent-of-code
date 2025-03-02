use solver::SolverBase;

pub struct Solver {
    directions: Vec<Direction>,
}

#[derive(Debug)]
enum Direction {
    North,
    Northeast,
    Southeast,
    South,
    Southwest,
    Northwest,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        let mut directions = Vec::new();
        for token in input.split(',') {
            let direction = match token {
                "n" => Direction::North,
                "ne" => Direction::Northeast,
                "se" => Direction::Southeast,
                "s" => Direction::South,
                "sw" => Direction::Southwest,
                "nw" => Direction::Northwest,
                _ => panic!("unsupported direction"),
            };
            directions.push(direction);
        }
        Solver { directions }
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        for direction in self.directions.iter() {
            match direction {
                Direction::North => y += 2,
                Direction::Northeast => {
                    x += 2;
                    y += 1;
                }
                Direction::Southeast => {
                    x += 2;
                    y -= 1;
                }
                Direction::South => {
                    y -= 2;
                }
                Direction::Southwest => {
                    x -= 2;
                    y -= 1;
                }
                Direction::Northwest => {
                    x -= 2;
                    y += 1;
                }
            }
        }
        "".to_string()
    }

    fn solve_part_two(&self) -> String {
        "".to_string()
    }

    fn day_number(&self) -> usize {
        11
    }

    fn description(&self) -> &'static str {
        "Honeycomb"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new("ne,ne,ne").solve_part_one();
        assert_eq!(result, "3");
    }

    #[test]
    fn test_2() {
        let result = Solver::new("ne,ne,sw,sw").solve_part_one();
        assert_eq!(result, "0");
    }

    #[test]
    fn test_3() {
        let result = Solver::new("ne,ne,s,s").solve_part_one();
        assert_eq!(result, "2");
    }

    #[test]
    fn test_4() {
        let result = Solver::new("se,sw,se,sw,sw").solve_part_one();
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
