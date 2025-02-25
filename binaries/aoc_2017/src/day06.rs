use solver::SolverBase;
use std::collections::HashMap;

pub struct Solver {
    memory_banks: Vec<u32>,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        Solver {
            memory_banks: input
                .split_whitespace()
                .map(|word| word.parse().unwrap())
                .collect(),
        }
    }

    fn get_cycle_start_and_end(&self) -> (u32, u32) {
        let mut memory = self.memory_banks.clone();
        let mut states = HashMap::new();
        let mut cycle_counter = 0;
        while !states.contains_key(&memory) {
            //println!("{:?}", memory);
            states.insert(memory.clone(), cycle_counter);
            let max_value = *memory.iter().max().unwrap();
            let max_index = memory.iter().position(|x| *x == max_value).unwrap();
            memory[max_index] = 0;
            for i in 0..max_value {
                let index = (max_index + 1 + i as usize) % memory.len();
                memory[index] += 1;
            }
            cycle_counter += 1;
        }
        (*states.get(&memory).unwrap(), cycle_counter)
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let (_, cycle_end) = self.get_cycle_start_and_end();
        cycle_end.to_string()
    }

    fn solve_part_two(&self) -> String {
        let (cycle_start, cycle_end) = self.get_cycle_start_and_end();
        let cycle_size = cycle_end - cycle_start;
        cycle_size.to_string()
    }

    fn day_number(&self) -> usize {
        6
    }

    fn description(&self) -> &'static str {
        "Memory reallocation"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new("0 2 7 0").solve_part_one();
        assert_eq!(result, "5");
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new("0 2 7 0").solve_part_two();
        assert_eq!(result, "4");
    }
}
