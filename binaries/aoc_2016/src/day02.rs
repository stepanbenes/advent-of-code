use solver::SolverBase;

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Solver {
    instructions: Vec<Vec<Instruction>>,
    starting_position: (usize, usize),
}

impl Solver {
    pub fn new(input: &'static str, starting_position: (usize, usize)) -> Self {
        let instructions = input.lines().map(|line| {
            line.chars().map(|c| match c {
                'U' => Instruction::Up,
                'D' => Instruction::Down,
                'L' => Instruction::Left,
                'R' => Instruction::Right,
                _ => panic!("Invalid instruction"),
            }).collect()
        }).collect();
        Solver { instructions, starting_position }
    }

    fn get_next_position(position: (usize, usize), instruction: &Instruction) -> (usize, usize) {
        const KEYPAD_WIDTH: usize = 3;
        const KEYPAD_HEIGHT: usize = 3;
        match instruction {
            Instruction::Up => (position.0, position.1.saturating_sub(1)),
            Instruction::Down => (position.0, (position.1 + 1).min(KEYPAD_HEIGHT - 1)),
            Instruction::Left => (position.0.saturating_sub(1), position.1),
            Instruction::Right => ((position.0 + 1).min(KEYPAD_WIDTH - 1), position.1),
        }
    }

    fn get_key(position: (usize, usize)) -> char {
        match position {
            (0, 0) => '1',
            (1, 0) => '2',
            (2, 0) => '3',
            (0, 1) => '4',
            (1, 1) => '5',
            (2, 1) => '6',
            (0, 2) => '7',
            (1, 2) => '8',
            (2, 2) => '9',
            _ => panic!("Invalid position"),
        }
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let mut code = String::new();
        let mut position = self.starting_position;
        for key_instructions in self.instructions.iter() {
            for instruction in key_instructions {
                position = Solver::get_next_position(position, instruction);
            }
            code.push(Solver::get_key(position));
        }
        code
    }

    fn solve_part_two(&self) -> String {
        "".to_string()
    }

    fn day_number(&self) -> usize {
        2
    }

    fn description(&self) -> &'static str {
        "Keypad"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(r"ULL
RRDDD
LURDL
UUUUD", (1, 1)).solve_part_one();
        assert_eq!(result, "1985");
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
