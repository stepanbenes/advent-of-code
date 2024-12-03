use regex::Regex;

#[derive(Debug)]
enum Instruction {
    Do,
    Dont,
    Mul(u32, u32),
}

pub fn add_results_of_multiplications() -> u32 {
    let input = get_input();
    let result: u32 = parse_instructions(input)
        .iter()
        .filter_map(|x| {
            if let Instruction::Mul(a, b) = x {
                Some(a * b)
            } else {
                None
            }
        })
        .sum();
    result
}

pub fn add_results_of_conditional_multiplications() -> u32 {
    let input = get_input();
    let mut enabled = true;
    let result: u32 = parse_instructions(input)
        .iter()
        .filter_map(|x| {
            if let Instruction::Mul(a, b) = x {
                if enabled {
                    Some(a * b)
                } else {
                    None
                }
            } else if let Instruction::Do = x {
                enabled = true;
                None
            } else if let Instruction::Dont = x {
                enabled = false;
                None
            } else {
                panic!("unexpected instruction: {:?}", x);
            }
        })
        .sum();
    result
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let regex = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();
    let instructions: Vec<Instruction> = regex
        .captures_iter(input)
        .map(|cap| {
            let x: &str = &cap[0];
            if x.starts_with("mul(") {
                let a: u32 = cap[1].parse().unwrap();
                let b: u32 = cap[2].parse().unwrap();
                Instruction::Mul(a, b)
            } else if x == "do()" {
                Instruction::Do
            } else if x == "don't()" {
                Instruction::Dont
            } else {
                panic!("unexpected input: {:?}", x);
            }
        })
        .collect();
    instructions
}

fn get_input() -> &'static str {
    let input = include_str!("../input/day3.txt");
    input
}
