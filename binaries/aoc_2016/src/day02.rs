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
}

impl Solver {
    pub fn get_standard_keypad() -> Vec<Vec<Option<char>>> {
        let keypad = vec![
            vec![Some('1'), Some('2'), Some('3')],
            vec![Some('4'), Some('5'), Some('6')],
            vec![Some('7'), Some('8'), Some('9')],
        ];
        keypad
    }

    pub fn get_star_keypad() -> Vec<Vec<Option<char>>> {
        let keypad = vec![
            vec![None, None, Some('1'), None, None],
            vec![None, Some('2'), Some('3'), Some('4'), None],
            vec![Some('5'), Some('6'), Some('7'), Some('8'), Some('9')],
            vec![None, Some('A'), Some('B'), Some('C'), None],
            vec![None, None, Some('D'), None, None],
        ];
        keypad
    }

    pub fn new(input: &'static str) -> Self {
        let instructions = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        'U' => Instruction::Up,
                        'D' => Instruction::Down,
                        'L' => Instruction::Left,
                        'R' => Instruction::Right,
                        _ => panic!("Invalid instruction"),
                    })
                    .collect()
            })
            .collect();
        Solver { instructions }
    }

    fn get_next_position_on_keypad(
        position: (usize, usize),
        instruction: &Instruction,
        keypad: &[Vec<Option<char>>],
    ) -> (usize, usize) {
        let keypad_width: usize = keypad[0].len();
        let keypad_height: usize = keypad.len();
        match instruction {
            Instruction::Up => {
                let new_position = (position.0, position.1.saturating_sub(1));
                if new_position.1 < keypad_height
                    && keypad[new_position.1][new_position.0].is_some()
                {
                    new_position
                } else {
                    position
                }
            }
            Instruction::Down => {
                let new_position = (position.0, (position.1 + 1).min(keypad_height - 1));
                if new_position.1 < keypad_height
                    && keypad[new_position.1][new_position.0].is_some()
                {
                    new_position
                } else {
                    position
                }
            }
            Instruction::Left => {
                let new_position = (position.0.saturating_sub(1), position.1);
                if new_position.0 < keypad_width && keypad[new_position.1][new_position.0].is_some()
                {
                    new_position
                } else {
                    position
                }
            }
            Instruction::Right => {
                let new_position = ((position.0 + 1).min(keypad_width - 1), position.1);
                if new_position.0 < keypad_width && keypad[new_position.1][new_position.0].is_some()
                {
                    new_position
                } else {
                    position
                }
            }
        }
    }

    fn get_key_on_standard_keypad(position: (usize, usize)) -> char {
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

    fn get_key_on_star_keypad(position: (usize, usize)) -> char {
        match position {
            (2, 0) => '1',
            (1, 1) => '2',
            (2, 1) => '3',
            (3, 1) => '4',
            (0, 2) => '5',
            (1, 2) => '6',
            (2, 2) => '7',
            (3, 2) => '8',
            (4, 2) => '9',
            (1, 3) => 'A',
            (2, 3) => 'B',
            (3, 3) => 'C',
            (2, 4) => 'D',
            _ => panic!("Invalid position"),
        }
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let mut code = String::new();
        let keypad = Solver::get_standard_keypad();
        let mut position = (1, 1);
        for key_instructions in self.instructions.iter() {
            for instruction in key_instructions {
                position = Solver::get_next_position_on_keypad(position, instruction, &keypad);
            }
            code.push(Solver::get_key_on_standard_keypad(position));
        }
        code
    }

    fn solve_part_two(&self) -> String {
        let mut code = String::new();
        let keypad = Solver::get_star_keypad();
        let mut position = (0, 2);
        for key_instructions in self.instructions.iter() {
            for instruction in key_instructions {
                position = Solver::get_next_position_on_keypad(position, instruction, &keypad);
            }
            code.push(Solver::get_key_on_star_keypad(position));
        }
        code
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
        let result = Solver::new(
            r"ULL
RRDDD
LURDL
UUUUD",
        )
        .solve_part_one();
        assert_eq!(result, "1985");
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r"ULL
RRDDD
LURDL
UUUUD",
        )
        .solve_part_two();
        assert_eq!(result, "5DB3");
    }
}
