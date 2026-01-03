use std::{cmp::Ordering, collections::VecDeque, str::FromStr};

use solver::SolverBase;

pub struct Solver {
    lights: Vec<Light>,
}

#[derive(Debug)]
pub struct Light {
    light_diagram: u32,
    toggles: Vec<u32>,
    toggle_indices: Vec<Vec<u32>>,
    joltages: Vec<u32>,
}

impl FromStr for Light {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // split by spaces
        let parts: Vec<&str> = s.split_whitespace().collect();

        // Helper to convert pattern to bitmask
        fn pattern_to_bitmask(pattern: &str) -> u32 {
            pattern.chars().enumerate().fold(
                0u32,
                |mask, (i, c)| {
                    if c == '#' { mask | (1 << i) } else { mask }
                },
            )
        }

        // 1. Extract pattern
        let diagram_pattern =
            pattern_to_bitmask(parts[0].trim_start_matches('[').trim_end_matches(']'));

        // Helper to parse "(...)"
        fn parse_paren(s: &str) -> Vec<u32> {
            s.trim_start_matches('(')
                .trim_end_matches(')')
                .split(',')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<u32>().unwrap())
                .collect()
        }

        // Helper to convert indices to bitmask
        fn to_bitmask(bits: &[u32]) -> u32 {
            bits.iter().fold(0u32, |mask, &bit| mask | (1 << bit))
        }

        // 2. All (...) groups
        let toggles: Vec<u32> = parts[1..parts.len() - 1]
            .iter()
            .filter(|s| s.starts_with('('))
            .map(|s| to_bitmask(&parse_paren(s)))
            .collect();

        // 3. All (...) groups
        let toggle_indices: Vec<Vec<u32>> = parts[1..parts.len() - 1]
            .iter()
            .filter(|s| s.starts_with('('))
            .map(|s| parse_paren(s))
            .collect();

        // 4. Parse {...}
        let joltages: Vec<u32> = parts
            .last()
            .unwrap()
            .trim_start_matches('{')
            .trim_end_matches('}')
            .split(',')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        Ok(Light {
            light_diagram: diagram_pattern,
            toggles,
            toggle_indices,
            joltages,
        })
    }
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        Solver {
            lights: input
                .lines()
                .map(|line| Light::from_str(line).unwrap())
                .collect(),
        }
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        fn bfs(light: &Light) -> usize {
            let mut queue: VecDeque<(u32, usize)> = VecDeque::new();
            queue.push_back((0, 0));
            while let Some((pattern, depth)) = queue.pop_front() {
                for toggle_indices in &light.toggles {
                    let new_pattern = pattern ^ toggle_indices;
                    if new_pattern == light.light_diagram {
                        return depth + 1;
                    }
                    queue.push_back((new_pattern, depth + 1));
                }
            }
            unreachable!()
        }

        let sum: usize = self.lights.iter().map(bfs).sum();
        sum.to_string()
    }

    fn solve_part_two(&self) -> String {
        fn bfs(light: &Light) -> usize {
            let mut queue: VecDeque<(Vec<u32>, usize)> = VecDeque::new();
            queue.push_back((vec![0; light.joltages.len()], 0));
            while let Some((pattern, depth)) = queue.pop_front() {
                for indices in &light.toggle_indices {
                    let mut new_pattern = pattern.clone();
                    for index in indices {
                        new_pattern[*index as usize] += 1;
                    }
                    match compare_patterns(&new_pattern, &light.joltages) {
                        Ordering::Equal => return depth + 1,
                        Ordering::Less => queue.push_back((new_pattern, depth + 1)),
                        Ordering::Greater => {}
                    }
                }
            }
            unreachable!()
        }

        fn compare_patterns(a: &[u32], b: &[u32]) -> Ordering {
            if a == b {
                return Ordering::Equal;
            }
            for i in 0..a.len() {
                if a[i].cmp(&b[i]) == Ordering::Greater {
                    return Ordering::Greater;
                }
            }
            Ordering::Less
        }

        let sum: usize = self.lights.iter().map(bfs).sum();
        sum.to_string()
    }

    fn day_number(&self) -> usize {
        10
    }

    fn description(&self) -> &'static str {
        "Factory"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        )
        .solve_part_one();
        assert_eq!(result, "7");
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        )
        .solve_part_two();
        assert_eq!(result, "33");
    }
}
