use std::collections::HashMap;

use solver::SolverBase;

pub struct Solver {
    instructions: Vec<Instruction>,
}

#[derive(Debug)]
enum Instruction {
    Increment(&'static str, i32, Condition),
    Decrement(&'static str, i32, Condition),
}

#[derive(Debug)]
enum Condition {
    GreaterThan(&'static str, i32),
    GreaterThanEqualTo(&'static str, i32),
    LessThan(&'static str, i32),
    LessThanEqualTo(&'static str, i32),
    EqualTo(&'static str, i32),
    NotEqualTo(&'static str, i32),
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        let mut instructions = Vec::new();

        fn parse_condition(tokens: &[&'static str]) -> Condition {
            match tokens {
                [condition_register, ">", condition_value] => {
                    Condition::GreaterThan(condition_register, condition_value.parse().unwrap())
                }
                [condition_register, ">=", condition_value] => Condition::GreaterThanEqualTo(
                    condition_register,
                    condition_value.parse().unwrap(),
                ),
                [condition_register, "<", condition_value] => {
                    Condition::LessThan(condition_register, condition_value.parse().unwrap())
                }
                [condition_register, "<=", condition_value] => {
                    Condition::LessThanEqualTo(condition_register, condition_value.parse().unwrap())
                }
                [condition_register, "==", condition_value] => {
                    Condition::EqualTo(condition_register, condition_value.parse().unwrap())
                }
                [condition_register, "!=", condition_value] => {
                    Condition::NotEqualTo(condition_register, condition_value.parse().unwrap())
                }
                _ => panic!("wrong format"),
            }
        }

        for line in input.lines() {
            let tokens: Vec<_> = line.split_whitespace().collect();
            let instruction = match &tokens[0..3] {
                [instruction_register, "inc", instruction_value] => Instruction::Increment(
                    instruction_register,
                    instruction_value.parse().unwrap(),
                    parse_condition(&tokens[4..]),
                ),
                [instruction_register, "dec", instruction_value] => Instruction::Decrement(
                    instruction_register,
                    instruction_value.parse().unwrap(),
                    parse_condition(&tokens[4..]),
                ),
                _ => panic!("wrong format"),
            };
            instructions.push(instruction);
        }
        Solver { instructions }
    }

    fn check_condition(condition: &Condition, registers: &HashMap<&'static str, i32>) -> bool {
        match condition {
            Condition::GreaterThan(register, condition_value) => {
                let register_value = *registers.get(*register).unwrap_or(&0);
                register_value > *condition_value
            }
            Condition::GreaterThanEqualTo(register, condition_value) => {
                let register_value = *registers.get(*register).unwrap_or(&0);
                register_value >= *condition_value
            }
            Condition::LessThan(register, condition_value) => {
                let register_value = *registers.get(*register).unwrap_or(&0);
                register_value < *condition_value
            }
            Condition::LessThanEqualTo(register, condition_value) => {
                let register_value = *registers.get(*register).unwrap_or(&0);
                register_value <= *condition_value
            }
            Condition::EqualTo(register, condition_value) => {
                let register_value = *registers.get(*register).unwrap_or(&0);
                register_value == *condition_value
            }
            Condition::NotEqualTo(register, condition_value) => {
                let register_value = *registers.get(*register).unwrap_or(&0);
                register_value != *condition_value
            }
        }
    }

    fn process_instruction(
        instruction: &Instruction,
        registers: &mut HashMap<&'static str, i32>,
    ) -> Option<i32> {
        match instruction {
            Instruction::Increment(register, value, condition)
                if Solver::check_condition(condition, registers) =>
            {
                let register = registers.entry(register).or_default();
                *register += value;
                Some(*register)
            }
            Instruction::Decrement(register, value, condition)
                if Solver::check_condition(condition, registers) =>
            {
                let register = registers.entry(register).or_default();
                *register -= value;
                Some(*register)
            }
            _ => None,
        }
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let mut registers = HashMap::new();
        for instruction in self.instructions.iter() {
            _ = Solver::process_instruction(instruction, &mut registers);
        }
        let max_value = registers.values().max().unwrap();
        max_value.to_string()
    }

    fn solve_part_two(&self) -> String {
        let mut registers = HashMap::new();
        let mut running_max = 0;
        for instruction in self.instructions.iter() {
            if let Some(register_value) = Solver::process_instruction(instruction, &mut registers) {
                if register_value > running_max {
                    running_max = register_value;
                }
            }
        }
        running_max.to_string()
    }

    fn day_number(&self) -> usize {
        8
    }

    fn description(&self) -> &'static str {
        "Largest value in computer registers"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r"b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10",
        )
        .solve_part_one();
        assert_eq!(result, "1");
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r"b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10",
        )
        .solve_part_two();
        assert_eq!(result, "10");
    }
}
