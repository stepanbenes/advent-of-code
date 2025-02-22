use solver::SolverBase;

pub struct Solver {
    square_number: i32,
}

enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        Solver {
            square_number: input.parse().unwrap(),
        }
    }

    fn get_coordinates(square_number: i32) -> (i32, i32) {
        let mut direction = Direction::Left;
        let mut square: i32 = 1;
        let mut increment: i32 = 1;
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        while square < square_number {
            match direction {
                Direction::Right => {
                    square += increment;
                    x += increment;
                    direction = Direction::Up;
                }
                Direction::Up => {
                    square += increment;
                    y += increment;
                    increment += 1;
                    direction = Direction::Left;
                }
                Direction::Left => {
                    square += increment;
                    x -= increment;
                    direction = Direction::Down;
                }
                Direction::Down => {
                    square += increment;
                    y -= increment;
                    increment += 1;
                    direction = Direction::Right;
                }
            }
        }
        let diff = square - square_number;
        if diff > 0 {
            match direction {
                Direction::Up => x -= diff,
                Direction::Left => y -= diff,
                Direction::Down => x += diff,
                Direction::Right => y += diff,
            }
        }
        (x, y)
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let (x, y) = Solver::get_coordinates(self.square_number);
        let manhattan_distance = x.abs() + y.abs();
        manhattan_distance.to_string()
    }

    fn solve_part_two(&self) -> String {
        "".to_string()
    }

    fn day_number(&self) -> usize {
        3
    }

    fn description(&self) -> &'static str {
        "Spiral memory"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new("1").solve_part_one();
        assert_eq!(result, "0");
    }

    #[test]
    fn test_2() {
        let result = Solver::new("12").solve_part_one();
        assert_eq!(result, "3");
    }

    #[test]
    fn test_3() {
        let result = Solver::new("23").solve_part_one();
        assert_eq!(result, "2");
    }

    #[test]
    fn test_4() {
        let result = Solver::new("1024").solve_part_one();
        assert_eq!(result, "31");
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
