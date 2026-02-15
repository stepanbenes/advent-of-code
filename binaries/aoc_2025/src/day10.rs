use std::str::FromStr;

use solver::SolverBase;

pub struct Solver {
    lights: Vec<Light>,
}

#[derive(Debug)]
pub struct Light {
    light_diagram: u32,
    toggle_indices: Vec<Vec<u32>>,
    joltages: Vec<i32>,
}

fn fewest_presses(light: &Light) -> usize {
    let binary_buttons = get_binary_buttons(&light.toggle_indices);
    for subset in subsets(&binary_buttons) {
        if subset.iter().fold(0, |a, &b| a ^ b) == light.light_diagram {
            return subset.len();
        }
    }
    unreachable!()
}

fn get_binary_buttons(buttons: &[Vec<u32>]) -> Vec<u32> {
    buttons
        .iter()
        .map(|b| b.iter().map(|n| 1u32 << n).sum())
        .collect()
}

// I gave up, went to Reddit and found this hint:
// https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/
// Find all possible sets of buttons you can push so that the remaining voltages are even, and divide by 2 and recurse.
fn fewest_joltage_presses(light: &Light) -> usize {
    let binary_buttons = get_binary_buttons(&light.toggle_indices);
    let subset_xors: Vec<_> = subsets(&binary_buttons)
        .iter()
        .map(|subset| (subset.to_owned(), subset.iter().fold(0, |a, &b| a ^ b)))
        .collect();
    fewest_joltage_presses_recur(&subset_xors, &light.joltages).unwrap()
}

fn fewest_joltage_presses_recur(
    subset_xors: &[(Vec<u32>, u32)],
    joltages: &[i32],
) -> Option<usize> {
    if joltages.iter().all(|&j| j == 0) {
        return Some(0);
    }
    let binary_joltages = get_binary_joltages(joltages);
    let mut best = None;
    for (subset, xor) in subset_xors {
        if *xor == binary_joltages {
            let new_joltages = get_new_joltages(joltages, subset);
            if new_joltages.iter().all(|&j| j >= 0) {
                let press_count = fewest_joltage_presses_recur(subset_xors, &new_joltages)
                    .map(|c| subset.len() + 2 * c);
                best = best.min(press_count).or(best).or(press_count);
            }
        }
    }
    best
}

fn get_new_joltages(joltages: &[i32], subset: &[u32]) -> Vec<i32> {
    let mut new_joltages = Vec::new();
    let mut mask = 1;
    for &joltage in joltages {
        new_joltages.push((joltage - subset.iter().filter(|&b| b & mask != 0).count() as i32) / 2);
        mask <<= 1;
    }
    new_joltages
}

fn get_binary_joltages(joltages: &[i32]) -> u32 {
    joltages
        .iter()
        .enumerate()
        .map(|(i, j)| ((1 << i) * (j % 2)) as u32)
        .sum()
}

fn subsets<T: Copy>(set: &[T]) -> Vec<Vec<T>> {
    let mut subsets: Vec<Vec<T>> = Vec::new();
    for count in 0..=set.len() {
        subsets.extend(get_combinations(set, count));
    }
    subsets
}

fn get_combinations<T: Copy>(set: &[T], count: usize) -> Vec<Vec<T>> {
    if count == 0 {
        vec![Vec::new()]
    } else {
        set[..set.len() - count + 1]
            .iter()
            .enumerate()
            .flat_map(|(i, &t)| {
                get_combinations(&set[i + 1..], count - 1)
                    .iter()
                    .map(|c| {
                        let mut c1 = c.clone();
                        c1.push(t);
                        c1
                    })
                    .collect::<Vec<Vec<T>>>()
            })
            .collect()
    }
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

        // All (...) groups
        let toggle_indices: Vec<Vec<u32>> = parts[1..parts.len() - 1]
            .iter()
            .filter(|s| s.starts_with('('))
            .map(|s| parse_paren(s))
            .collect();

        // Parse {...}
        let joltages: Vec<i32> = parts
            .last()
            .unwrap()
            .trim_start_matches('{')
            .trim_end_matches('}')
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        Ok(Light {
            light_diagram: diagram_pattern,
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
        let sum = self.lights.iter().map(fewest_presses).sum::<usize>();
        sum.to_string()
    }

    fn solve_part_two(&self) -> String {
        // see: https://old.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/
        let sum = self
            .lights
            .iter()
            .map(fewest_joltage_presses)
            .sum::<usize>();
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
