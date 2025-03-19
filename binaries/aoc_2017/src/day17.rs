use solver::SolverBase;

pub struct Solver {
    input: usize,
}

impl Solver {
    pub fn new(input: usize) -> Self {
        Solver { input }
    }

    fn get_wrapped(buffer: &[u32], current: usize, offset: usize) -> (usize, u32) {
        let next = (current + offset) % buffer.len();
        (next, buffer[next])
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let mut buffer = vec![0u32];
        let mut position = 0usize;
        let offset = self.input;
        for i in 1..=2017 {
            // println!("{}", buffer.iter().enumerate().map(|(i, x)| if i == position { format!("({x})") } else { x.to_string() }).collect::<Vec<_>>().join(","));
            // println!();
            let (next_position, _) = Solver::get_wrapped(&buffer, position, offset);
            buffer.insert(next_position + 1, i);
            position = next_position + 1;
        }
        let (_, value) = Solver::get_wrapped(&buffer, position, 1);
        value.to_string()
    }

    fn solve_part_two(&self) -> String {
        "".to_string()
    }

    fn day_number(&self) -> usize {
        17
    }

    fn description(&self) -> &'static str {
        "Spinlock"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(3).solve_part_one();
        assert_eq!(result, "638");
    }

    #[test]
    fn test_2() {
        let result = Solver::new(3).solve_part_one();
        assert_eq!(result, "638");
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
