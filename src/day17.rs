use itertools::Itertools;
use rayon::prelude::*;

#[derive(Clone)]
struct Computer {
    instruction_pointer: usize,
    register_a: u64,
    register_b: u64,
    register_c: u64,
}

impl std::fmt::Debug for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Computer")
            .field("instruction_pointer", &self.instruction_pointer)
            .field("register_a", &format_args!("{:o}", self.register_a))
            .field("register_b", &format_args!("{:o}", self.register_b))
            .field("register_c", &format_args!("{:o}", self.register_c))
            .finish()
    }
}

#[derive(Debug, Clone, Copy)]
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
    fn get_combo_operand_value(&self, computer: &Computer) -> u64 {
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

    fn get_literal_operand_value(&self) -> u64 {
        self.operand as u64
    }
}

type Program = Vec<Instruction>;

pub fn run_chronospacial_computer_program() -> String {
    let (mut computer, program) = get_input();
    println!("Computer: {:?}, Program: {:?}", computer, program);
    run_program(&mut computer, &program).iter().join(",")
}

pub fn lowest_positive_value_of_register_a_to_print_copy_of_itself() -> u64 {
    let (_, program) = get_input();

    search_a(
        &mut Computer {
            register_a: 0,
            register_b: 0,
            register_c: 0,
            instruction_pointer: 0,
        },
        (program.len() * 2) - 1,
        &program,
    )
    .unwrap()

    // let program_input: Vec<u64> = program.iter().flat_map(|inst| [inst.operation as u8 as u64, inst.operand as u64]).collect();
    // println!("Program input: {}", program_input.iter().join(","));
    // for register_a in (105843716614554..) {
    //     computer.register_a = register_a;
    //     computer.register_b = 0;
    //     computer.register_c = 0;
    //     computer.instruction_pointer = 0;
    //     let program_output = run_program(&mut computer, &program);
    //     //if register_a % 500000 == 0 {
    //         // println!("Testing register A: {:o}", register_a);
    //         // println!("Program output: {}", program_output);
    //         // println!();
    //     //}
    //     if program_output == program_input {
    //         return register_a;
    //     }
    // }
    // 0
}

fn search_a(memory: &mut Computer, iteration: usize, program: &Program) -> Option<u64> {
    for remainder in 0..8 {
        let multiplier = 8u64.pow(iteration as u32);

        if memory.register_a + multiplier * remainder < 8u64.pow(program.len() as u32 - 1) {
            continue;
        }

        let result = run_program(
            &mut Computer {
                register_a: memory.register_a + multiplier * remainder,
                register_b: memory.register_b,
                register_c: memory.register_c,
                instruction_pointer: 0,
            },
            program,
        );

        if result[iteration]
            == program
                .iter()
                .flat_map(|int| [int.operation as u8 as u64, int.operand as u64])
                .collect::<Vec<_>>()[iteration]
        {
            return if iteration == 0 {
                Some(memory.register_a + multiplier * remainder)
            } else if let Some(register_a) = search_a(
                &mut Computer {
                    register_a: memory.register_a + multiplier * remainder,
                    register_b: memory.register_b,
                    register_c: memory.register_c,
                    instruction_pointer: 0,
                },
                iteration - 1,
                program,
            ) {
                Some(register_a)
            } else {
                continue;
            };
        }
    }

    None
}

pub fn _lowest_positive_value_of_register_a_to_print_copy_of_itself_parallel() -> u64 {
    let (computer_template, program) = get_input();
    let program_input: Vec<u64> = program
        .iter()
        .flat_map(|inst| [inst.operation as u8 as u64, inst.operand as u64])
        .collect();
    println!("Test value: {}", program_input.iter().join(","));

    // Determine the number of threads to use
    let num_threads = rayon::current_num_threads();
    println!("Number of threads: {}", num_threads);
    // Parallel search across multiple threads
    let result = (0..num_threads).into_par_iter().find_map_any(|thread_id| {
        println!("Thread {} started", thread_id);
        // Each thread searches a different segment of numbers
        let mut current = thread_id as u64;
        loop {
            if current % num_threads as u64 == thread_id as u64 {
                if current % (13 * 17 * 23 * 29 * 31) == 0 {
                    println!("Testing register A: {} (thread {})", current, thread_id);
                }

                // Create a clone of the computer for each iteration
                let mut computer = computer_template.clone();
                computer.register_a = current;
                computer.register_b = 0;
                computer.register_c = 0;
                computer.instruction_pointer = 0;

                let program_output = run_program(&mut computer, &program);

                if program_output == program_input {
                    return Some(current);
                }
            }

            // Increment by number of threads to ensure even distribution
            current += num_threads as u64;
        }
    });

    result.unwrap_or(0)
}

fn get_input() -> (Computer, Program) {
    let input = include_str!("../input/day17.txt");
    //     let input = r"Register A: 729
    // Register B: 0
    // Register C: 0

    // Program: 0,1,5,4,3,0";
    //     let input = r"Register A: 2024
    // Register B: 0
    // Register C: 0

    // Program: 0,3,5,4,3,0";

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

fn run_program(computer: &mut Computer, program: &Program) -> Vec<u64> {
    let mut output = Vec::<u64>::new();
    loop {
        if computer.instruction_pointer >= program.len() {
            break;
        }
        let instruction = &program[computer.instruction_pointer];
        match instruction.operation {
            InstructionType::adv => {
                // The adv instruction (opcode 0) performs division. The numerator is the value in the A register.
                // The denominator is found by raising 2 to the power of the instruction's combo operand.
                // (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.)
                // The result of the division operation is truncated to an integer and then written to the A register.
                let denominator = 2_u64.pow(instruction.get_combo_operand_value(computer) as u32);
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
                output.push(output_value);
                computer.instruction_pointer += 1;
            }
            InstructionType::bdv => {
                // The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register.
                // (The numerator is still read from the A register.)
                let denominator = 2_u64.pow(instruction.get_combo_operand_value(computer) as u32);
                computer.register_b = computer.register_a / denominator;
                computer.instruction_pointer += 1;
            }
            InstructionType::cdv => {
                // The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register.
                // (The numerator is still read from the A register.)
                let denominator = 2_u64.pow(instruction.get_combo_operand_value(computer) as u32);
                computer.register_c = computer.register_a / denominator;
                computer.instruction_pointer += 1;
            }
        }
    }
    output
}
