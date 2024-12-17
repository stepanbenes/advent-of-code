use itertools::Itertools;

#[derive(Debug)]
struct Computer {
    instruction_pointer: usize,
    register_a: i64,
    register_b: i64,
    register_c: i64,
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
enum InstructionType {
    adv = 0,
    bxl = 1,
    bst = 2,
    jnz = 3,
    bxc = 4,
    out = 5,
    bdv = 6,
    cdv = 7,
}

impl std::str::FromStr for InstructionType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "adv" | "0" => Ok(InstructionType::adv),
            "bxl" | "1" => Ok(InstructionType::bxl),
            "bst" | "2" => Ok(InstructionType::bst),
            "jnz" | "3" => Ok(InstructionType::jnz),
            "bxc" | "4" => Ok(InstructionType::bxc),
            "out" | "5" => Ok(InstructionType::out),
            "bdv" | "6" => Ok(InstructionType::bdv),
            "cdv" | "7" => Ok(InstructionType::cdv),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    operation: InstructionType,
    operand: u8,
}

impl Instruction {
    fn get_combo_operand_value(&self, computer: &Computer) -> i64 {
        match self.operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => computer.register_a,
            5 => computer.register_b,
            6 => computer.register_c,
            7 => panic!("Reserved value"),
            _ => panic!("Invalid operand value"),
        }
    }

    fn get_literal_operand_value(&self) -> i64 {
        self.operand as i64
    }
}

type Program = Vec<Instruction>;

pub fn run_chronospacial_computer_program() -> String {
    let (mut computer, program) = get_input();
    println!("Computer: {:?}, Program: {:?}", computer, program);
    run_program(&mut computer, &program)
}

fn get_input() -> (Computer, Program) {
    let input = include_str!("../input/day17.txt");
    //     let input = r"Register A: 729
    // Register B: 0
    // Register C: 0

    // Program: 0,1,5,4,3,0";

    let mut register_a = 0;
    let mut register_b = 0;
    let mut register_c = 0;
    let mut program = Vec::new();

    for line in input.lines() {
        if line.starts_with("Register A: ") {
            register_a = line.split(": ").nth(1).unwrap().parse().unwrap();
        } else if line.starts_with("Register B: ") {
            register_b = line.split(": ").nth(1).unwrap().parse().unwrap();
        } else if line.starts_with("Register C: ") {
            register_c = line.split(": ").nth(1).unwrap().parse().unwrap();
        } else if line.starts_with("Program: ") {
            let program_str = line.split(": ").nth(1).unwrap();
            for chunk in program_str
                .split(",")
                .chunks(2)
                .into_iter()
                .map(|chunk| chunk.collect::<Vec<_>>())
            {
                if let [instruction, operand] = &chunk[..] {
                    let operation = instruction.parse().unwrap();
                    let operand = operand.parse().unwrap();
                    program.push(Instruction { operation, operand });
                } else {
                    panic!("Invalid program");
                }
            }
        }
    }
    (
        Computer {
            instruction_pointer: 0,
            register_a,
            register_b,
            register_c,
        },
        program,
    )
}

fn run_program(computer: &mut Computer, program: &Program) -> String {
    let mut output = Vec::<u8>::new();
    loop {
        let instruction = &program[computer.instruction_pointer];
        match instruction.operation {
            InstructionType::adv => {
                // The adv instruction (opcode 0) performs division. The numerator is the value in the A register.
                // The denominator is found by raising 2 to the power of the instruction's combo operand.
                // (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.)
                // The result of the division operation is truncated to an integer and then written to the A register.
                let denominator = 2_i64.pow(instruction.get_combo_operand_value(computer) as u32);
                computer.register_a /= denominator;
                computer.instruction_pointer += 1;
            }
            InstructionType::bxl => {
                // The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand, then stores the result in register B.
                computer.register_b ^= instruction.get_literal_operand_value();
                computer.instruction_pointer += 1;
            }
            InstructionType::bst => {
                // The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits), then writes that value to the B register.
                computer.register_b = instruction.get_combo_operand_value(computer) % 8;
                computer.instruction_pointer += 1;
            }
            InstructionType::jnz => {
                // The jnz instruction (opcode 3) does nothing if the A register is 0.
                // However, if the A register is not zero, it jumps by setting the instruction pointer to the value of its literal operand;
                // if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
                if computer.register_a != 0 {
                    computer.instruction_pointer = instruction.get_literal_operand_value() as usize;
                } else {
                    computer.instruction_pointer += 1;
                }
            }
            InstructionType::bxc => {
                // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then stores the result in register B.
                // (For legacy reasons, this instruction reads an operand but ignores it.)
                computer.register_b ^= computer.register_c;
                computer.instruction_pointer += 1;
            }
            InstructionType::out => {
                // The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value.
                // (If a program outputs multiple values, they are separated by commas.)
                let output_value = instruction.get_combo_operand_value(computer) % 8;
                output.push(output_value as u8);
                computer.instruction_pointer += 1;
            }
            InstructionType::bdv => {
                // The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register.
                // (The numerator is still read from the A register.)
                let denominator = 2_i64.pow(instruction.get_combo_operand_value(computer) as u32);
                computer.register_b = computer.register_a / denominator;
                computer.instruction_pointer += 1;
            }
            InstructionType::cdv => {
                // The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register.
                // (The numerator is still read from the A register.)
                let denominator = 2_i64.pow(instruction.get_combo_operand_value(computer) as u32);
                computer.register_c = computer.register_a / denominator;
                computer.instruction_pointer += 1;
            }
        }
        if computer.instruction_pointer >= program.len() {
            break;
        }
    }
    output.iter().join(",")
}
