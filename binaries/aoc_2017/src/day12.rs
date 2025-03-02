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

    fn get_set(&self, root: u32) -> HashSet<u32> {
        let mut set = HashSet::new();
        let mut current_set = vec![root];
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
        set
    }

    fn get_all_sets(&self) -> HashMap<u32, HashSet<u32>> {
        let mut all_roots: HashSet<u32> = self.connections.keys().copied().collect();
        let mut result = HashMap::new();
        while !all_roots.is_empty() {
            let root = *all_roots.iter().next().unwrap();
            let set = self.get_set(root);
            all_roots.retain(|x| !set.contains(x));
            result.insert(root, set);
        }
        result
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let set = self.get_set(0);
        set.len().to_string()
    }

    fn solve_part_two(&self) -> String {
        let all_sets = self.get_all_sets();
        all_sets.len().to_string()
    }

    fn day_number(&self) -> usize {
        12
    }

    fn description(&self) -> &'static str {
        "Program connection sets"
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

#[cfg(test)]
mod part2_tests {
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
        .solve_part_two();
        assert_eq!(result, "2");
    }
}
