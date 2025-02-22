use std::collections::HashMap;

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
        let mut direction = Direction::Right;
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

    fn get_first_value_larger_than(number: i32) -> i32 {
        let mut direction = Direction::Right;
        let mut increment: i32 = 1;
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut map = HashMap::new();
        map.insert((0, 0), 1);

        fn get_sum_of_adjacent_squares(coords: &(i32, i32), map: &HashMap<(i32, i32), i32>) -> i32 {
            let adjacent_squares = [
                *map.get(&(coords.0 + 1, coords.1)).unwrap_or(&0),
                *map.get(&(coords.0 + 1, coords.1 + 1)).unwrap_or(&0),
                *map.get(&(coords.0, coords.1 + 1)).unwrap_or(&0),
                *map.get(&(coords.0 - 1, coords.1 + 1)).unwrap_or(&0),
                *map.get(&(coords.0 - 1, coords.1)).unwrap_or(&0),
                *map.get(&(coords.0 - 1, coords.1 - 1)).unwrap_or(&0),
                *map.get(&(coords.0, coords.1 - 1)).unwrap_or(&0),
                *map.get(&(coords.0 + 1, coords.1 - 1)).unwrap_or(&0),
            ];
            adjacent_squares.iter().sum()
        }

        loop {
            match direction {
                Direction::Right => {
                    for _ in 0..increment {
                        x += 1;
                        let coords = (x, y);
                        let square_value = get_sum_of_adjacent_squares(&coords, &map);
                        if square_value > number {
                            return square_value;
                        }
                        map.insert(coords, square_value);
                    }
                    direction = Direction::Up;
                }
                Direction::Up => {
                    for _ in 0..increment {
                        y += 1;
                        let coords = (x, y);
                        let square_value = get_sum_of_adjacent_squares(&coords, &map);
                        if square_value > number {
                            return square_value;
                        }
                        map.insert(coords, square_value);
                    }
                    increment += 1;
                    direction = Direction::Left;
                }
                Direction::Left => {
                    for _ in 0..increment {
                        x -= 1;
                        let coords = (x, y);
                        let square_value = get_sum_of_adjacent_squares(&coords, &map);
                        if square_value > number {
                            return square_value;
                        }
                        map.insert(coords, square_value);
                    }
                    direction = Direction::Down;
                }
                Direction::Down => {
                    for _ in 0..increment {
                        y -= 1;
                        let coords = (x, y);
                        let square_value = get_sum_of_adjacent_squares(&coords, &map);
                        if square_value > number {
                            return square_value;
                        }
                        map.insert(coords, square_value);
                    }
                    increment += 1;
                    direction = Direction::Right;
                }
            }
        }
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let (x, y) = Solver::get_coordinates(self.square_number);
        let manhattan_distance = x.abs() + y.abs();
        manhattan_distance.to_string()
    }

    fn solve_part_two(&self) -> String {
        let value = Solver::get_first_value_larger_than(self.square_number);
        value.to_string()
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

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::get_first_value_larger_than(1);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2() {
        let result = Solver::get_first_value_larger_than(70);
        assert_eq!(result, 122);
    }

    #[test]
    fn test_3() {
        let result = Solver::get_first_value_larger_than(750);
        assert_eq!(result, 806);
    }
}
