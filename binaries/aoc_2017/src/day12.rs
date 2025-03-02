use std::collections::{HashMap, HashSet};

use regex::Regex;
use solver::SolverBase;

pub struct Solver {
    connections: HashMap<u32, Vec<u32>>,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        let re = Regex::new(r"(\d+) <-> ([\d, ]+)").unwrap();
        let mut connections: HashMap<u32, Vec<u32>> = HashMap::new();
        for capture in re.captures_iter(input) {
            let first = capture[1].parse::<u32>().unwrap();
            let second = capture[2]
                .split(", ")
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            connections.insert(first, second);
        }
        Solver { connections }
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let mut set = HashSet::new();
        let mut current_set = vec![0];
        while !current_set.is_empty() {
            while let Some(current) = current_set.pop() {
                if !set.contains(&current) {
                    set.insert(current);
                    for neighbor in self.connections.get(&current).unwrap().iter() {
                        current_set.push(*neighbor);
                    }
                }
            }
        }
        set.len().to_string()
    }

    fn solve_part_two(&self) -> String {
        "".to_string()
    }

    fn day_number(&self) -> usize {
        12
    }

    fn description(&self) -> &'static str {
        ""
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r"0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5",
        )
        .solve_part_one();
        assert_eq!(result, "6");
    }
}

// #[cfg(test)]
// mod part2_tests {
//     use super::*;

//     #[test]
//     fn test_1() {
//         let result = Solver::new(r"0 <-> 2
// 1 <-> 1
// 2 <-> 0, 3, 4
// 3 <-> 2, 4
// 4 <-> 2, 3, 6
// 5 <-> 6
// 6 <-> 4, 5").solve_part_two();
//         assert_eq!(result, "2");
//     }
// }
