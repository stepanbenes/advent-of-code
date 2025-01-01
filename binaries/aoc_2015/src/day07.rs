use std::{cell::RefCell, collections::HashMap};

use solver::Solver;

pub struct Day07Solver {
    instructions: HashMap<&'static str, Instruction>,
    memo: RefCell<HashMap<&'static str, u16>>,
}

#[derive(Debug)]
enum WireOrValue {
    Wire(&'static str),
    Value(u16),
}

#[derive(Debug)]
enum Instruction {
    Forward(WireOrValue),
    And(WireOrValue, WireOrValue),
    Or(WireOrValue, WireOrValue),
    Not(WireOrValue),
    Lshift(WireOrValue, u16),
    Rshift(WireOrValue, u16),
}

impl Day07Solver {
    pub fn new(input: &'static str) -> Self {
        fn parse_wire_or_value(op: &'static str) -> WireOrValue {
            op.parse()
                .map_or(WireOrValue::Wire(op), WireOrValue::Value)
        }

        fn parse_line(line: &'static str) -> (&'static str, Instruction) {
            let tokens = line.split_whitespace().collect::<Vec<_>>();
            match &tokens[..] {
                [op, "->", wire] => (wire, Instruction::Forward(parse_wire_or_value(op))),
                [left, "AND", right, "->", wire] => (
                    wire,
                    Instruction::And(parse_wire_or_value(left), parse_wire_or_value(right)),
                ),
                [left, "OR", right, "->", wire] => (
                    wire,
                    Instruction::Or(parse_wire_or_value(left), parse_wire_or_value(right)),
                ),
                ["NOT", op, "->", wire] => (wire, Instruction::Not(parse_wire_or_value(op))),
                [op, "LSHIFT", value, "->", wire] => (
                    wire,
                    Instruction::Lshift(parse_wire_or_value(op), value.parse().unwrap()),
                ),
                [op, "RSHIFT", value, "->", wire] => (
                    wire,
                    Instruction::Rshift(parse_wire_or_value(op), value.parse().unwrap()),
                ),
                _ => panic!("unrecognized instruction"),
            }
        }
        let mut instructions: HashMap<&'static str, Instruction> = HashMap::new();
        for line in input.lines() {
            let (wire, instruction) = parse_line(line);
            instructions.insert(wire, instruction);
        }
        Day07Solver {
            instructions,
            memo: RefCell::new(HashMap::new()),
        }
    }

    fn get_value(&self, wire_or_value: &WireOrValue) -> u16 {
        match wire_or_value {
            WireOrValue::Value(value) => *value,
            WireOrValue::Wire(wire) => self.evaluate(self.instructions.get(wire).unwrap(), wire),
        }
    }

    fn evaluate(&self, instruction: &Instruction, wire_name: &'static str) -> u16 {
        if let Some(&value) = self.memo.borrow().get(wire_name) {
            return value;
        }
        let result = match instruction {
            Instruction::Forward(op) => self.get_value(op),
            Instruction::And(left, right) => self.get_value(left) & self.get_value(right),
            Instruction::Or(left, right) => self.get_value(left) | self.get_value(right),
            Instruction::Not(op) => !self.get_value(op),
            Instruction::Lshift(op, value) => self.get_value(op) << value,
            Instruction::Rshift(op, value) => self.get_value(op) >> value,
        };
        self.memo.borrow_mut().insert(wire_name, result);
        result
    }

    #[allow(dead_code)]
    fn evaluate_all(&self) -> String {
        let mut signals: Vec<(&'static str, u16)> = Vec::new();
        for (wire, instruction) in self.instructions.iter() {
            let signal = self.evaluate(instruction, wire);
            signals.push((wire, signal));
        }
        signals.sort_by(|(wire_a, _), (wire_b, _)| wire_a.cmp(wire_b));
        println!("{:?}", signals);
        let mut result = Vec::new();
        for (wire, signal) in signals {
            result.push(format!("{wire}: {signal}"));
        }
        result.join("\n")
    }
}

impl Solver for Day07Solver {
    fn solve_part_one(&self) -> String {
        let a_signal = self.evaluate(self.instructions.get("a").unwrap(), "a");
        a_signal.to_string()
    }

    fn solve_part_two(&self) -> String {
        {
            //_ = self.evaluate(self.instructions.get("a").unwrap(), "a");
            let mut memo = self.memo.borrow_mut();
            let a_signal = *memo.get("a").unwrap();
            memo.clear();
            memo.insert("b", a_signal);
        }
        let a_signal = self.evaluate(self.instructions.get("a").unwrap(), "a");
        a_signal.to_string()
    }

    fn day_number(&self) -> usize {
        7
    }

    fn description(&self) -> &'static str {
        "Wires and logic gates"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Day07Solver::new(
            r"123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i",
        )
        .evaluate_all();
        assert_eq!(
            result,
            r"d: 72
e: 507
f: 492
g: 114
h: 65412
i: 65079
x: 123
y: 456"
        );
    }
}

// #[cfg(test)]
// mod part2_tests {
//     use super::*;

//     #[test]
//     fn test_1() {
//         let result = Day07Solver::new("abc").solve_part_two();
//         assert_eq!(result, "0");
//     }
// }
