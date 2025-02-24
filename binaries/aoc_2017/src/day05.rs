use solver::SolverBase;

pub struct Solver {
    jump_offsets: Vec<i32>,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        Solver {
            jump_offsets: input.lines().map(|line| line.parse().unwrap()).collect(),
        }
    }

    fn escape_from_maze(&self, offset_updater: impl Fn(i32) -> i32) -> u32 {
        let mut jump_offsets = self.jump_offsets.clone();
        let mut index: usize = 0;
        let mut steps: u32 = 0;
        loop {
            let offset = jump_offsets[index];
            jump_offsets[index] = offset_updater(offset);
            steps += 1;
            let new_index: isize = index as isize + offset as isize;
            if new_index < 0 || new_index >= jump_offsets.len() as isize {
                break;
            }
            index = new_index as usize;
        }
        steps
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        self.escape_from_maze(|x| x + 1).to_string()
    }

    fn solve_part_two(&self) -> String {
        self.escape_from_maze(|x| if x >= 3 { x - 1 } else { x + 1 })
            .to_string()
    }

    fn day_number(&self) -> usize {
        5
    }

    fn description(&self) -> &'static str {
        "Escaping offset maze"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r"0
3
0
1
-3",
        )
        .solve_part_one();
        assert_eq!(result, "5");
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r"0
3
0
1
-3",
        )
        .solve_part_two();
        assert_eq!(result, "10");
    }
}
