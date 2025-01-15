use solver::SolverBase;
use std::collections::{HashMap, HashSet};

pub struct Solver {
    rewrite_rules: HashMap<&'static str, Vec<&'static str>>,
    molecule: &'static str,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        let mut rewrite_rules: HashMap<&str, Vec<&str>> = HashMap::new();
        let mut molecule = "";
        for line in input.lines() {
            let parts: Vec<_> = line.split(" => ").collect();
            if let &[from, to] = &parts[..] {
                rewrite_rules.entry(from).or_default().push(to);
            } else if parts.len() == 1 {
                molecule = parts[0];
            }
        }
        Solver {
            rewrite_rules,
            molecule,
        }
    }

    fn find_all_positions(haystack: &str, needle: &str) -> Vec<usize> {
        let mut positions = Vec::new();
        let mut start = 0;

        while let Some(pos) = haystack[start..].find(needle) {
            positions.push(start + pos);
            start += pos + 1; // Move past the last match
        }

        positions
    }

    fn replace_at_index(
        original: &str,
        index: usize,
        to_replace: &str,
        replacement: &str,
    ) -> String {
        // Check if `to_replace` exists at the given index
        if original[index..].starts_with(to_replace) {
            // Split the string into parts: before, to_replace, and after
            let before = &original[..index];
            let after = &original[index + to_replace.len()..];
            // Combine the parts with the replacement
            format!("{}{}{}", before, replacement, after)
        } else {
            // If `to_replace` doesn't exist at the index, return the original string
            original.to_string()
        }
    }

    fn _invert_rules(
        rules: &HashMap<&'static str, Vec<&'static str>>,
    ) -> HashMap<&'static str, Vec<&'static str>> {
        let mut result = HashMap::new();
        for (rule_target, replacements) in rules.iter() {
            for replacement in replacements {
                result
                    .entry(*replacement)
                    .or_insert(Vec::new())
                    .push(*rule_target);
            }
        }
        result
    }

    fn find_all_descendants(
        molecule: String,
        rules: &HashMap<&'static str, Vec<&'static str>>,
    ) -> HashSet<String> {
        let mut distinct_molecules = HashSet::new();
        for (rule_target, replacements) in rules.iter() {
            let positions = Solver::find_all_positions(&molecule, rule_target);
            for position in positions {
                for replacement in replacements.iter() {
                    let new_molecule =
                        Solver::replace_at_index(&molecule, position, rule_target, replacement);
                    distinct_molecules.insert(new_molecule);
                }
            }
        }
        distinct_molecules
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let distinct_molecules =
            Solver::find_all_descendants(self.molecule.to_owned(), &self.rewrite_rules);
        distinct_molecules.len().to_string()
    }

    fn solve_part_two(&self) -> String {
        "0".to_string()
    }

    fn day_number(&self) -> usize {
        19
    }

    fn description(&self) -> &'static str {
        "Molecule replacement"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r"H => HO
H => OH
O => HH

HOH",
        )
        .solve_part_one();
        assert_eq!(result, "4");
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r"e => H
e => O
H => HO
H => OH
O => HH

HOH",
        )
        .solve_part_two();
        assert_eq!(result, "3");
    }

    #[test]
    fn test_2() {
        let result = Solver::new(
            r"e => H
e => O
H => HO
H => OH
O => HH

HOHOHO",
        )
        .solve_part_two();
        assert_eq!(result, "6");
    }
}
