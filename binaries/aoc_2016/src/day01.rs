use std::collections::HashSet;

use solver::SolverBase;

pub struct Solver {
    input: &'static str,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        Solver { input }
    }

    fn walk(&self, stop_at_intersection: bool) -> i32 {
        let mut position: (i32, i32) = (0, 0);
        let mut direction = 0;
        let mut visited = HashSet::new();
        visited.insert(position);
        'outer: for m in self.input.split(", ") {
            let (dir, dist) = m.split_at(1);
            let dist = dist.parse::<i32>().unwrap();
            direction = (direction + if dir == "R" { 1 } else { 3 }) % 4;
            for _ in 0..dist {
                match direction {
                    0 => position.1 += 1,
                    1 => position.0 += 1,
                    2 => position.1 -= 1,
                    3 => position.0 -= 1,
                    _ => unreachable!(),
                }
                if stop_at_intersection && visited.contains(&position) {
                    break 'outer;
                }
                visited.insert(position);
            }
        }
        position.0.abs() + position.1.abs()
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        self.walk(false).to_string()
    }

    fn solve_part_two(&self) -> String {
        self.walk(true).to_string()
    }

    fn day_number(&self) -> usize {
        1
    }

    fn description(&self) -> &'static str {
        "Street grid"
    }
}
