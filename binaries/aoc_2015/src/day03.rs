use solver::Solver;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position(isize, isize);

impl Position {
    fn move_in_direction(&self, direction: Direction) -> Position {
        match direction {
            Direction::Up => Position(self.0, self.1 - 1),
            Direction::Down => Position(self.0, self.1 + 1),
            Direction::Left => Position(self.0 - 1, self.1),
            Direction::Right => Position(self.0 + 1, self.1),
        }
    }
}

pub struct Day03Solver {
    directions: Vec<Direction>,
}

impl Day03Solver {
    pub fn new(input: &'static str) -> Self {
        let directions = input
            .chars()
            .map(|c| match c {
                '^' => Direction::Up,
                'v' => Direction::Down,
                '<' => Direction::Left,
                '>' => Direction::Right,
                _ => panic!("unrecognized symbol"),
            })
            .collect();
        Day03Solver { directions }
    }
}

impl Solver for Day03Solver {
    fn solve_part_one(&self) -> String {
        let mut position = Position(0, 0);
        let mut visited_positions = HashSet::new();
        visited_positions.insert(position);
        for direction in self.directions.iter() {
            position = position.move_in_direction(*direction);
            visited_positions.insert(position);
        }
        visited_positions.len().to_string()
    }

    fn solve_part_two(&self) -> String {
        let mut santa_position = Position(0, 0);
        let mut robot_position = Position(0, 0);
        let mut visited_positions = HashSet::new();
        visited_positions.insert(santa_position);
        visited_positions.insert(robot_position);
        for chunk in self.directions.chunks(2) {
            santa_position = santa_position.move_in_direction(chunk[0]);
            robot_position = robot_position.move_in_direction(chunk[1]);

            visited_positions.insert(santa_position);
            visited_positions.insert(robot_position);
        }
        visited_positions.len().to_string()
    }

    fn day_number(&self) -> usize {
        3
    }

    fn description(&self) -> &'static str {
        "Presents delivery"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Day03Solver::new(">").solve_part_one();
        assert_eq!(result, "2");
    }

    #[test]
    fn test_2() {
        let result = Day03Solver::new("^>v<").solve_part_one();
        assert_eq!(result, "4");
    }

    #[test]
    fn test_3() {
        let result = Day03Solver::new("^v^v^v^v^v").solve_part_one();
        assert_eq!(result, "2");
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Day03Solver::new("^v").solve_part_two();
        assert_eq!(result, "3");
    }

    #[test]
    fn test_2() {
        let result = Day03Solver::new("^>v<").solve_part_two();
        assert_eq!(result, "3");
    }

    #[test]
    fn test_3() {
        let result = Day03Solver::new("^v^v^v^v^v").solve_part_two();
        assert_eq!(result, "11");
    }
}
