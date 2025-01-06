use std::collections::{HashMap, HashSet};

use solver::SolverBase;

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
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
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

        let mut distinct_molecules = HashSet::new();
        for (atom, replacements) in self.rewrite_rules.iter() {
            let positions = Solver::find_all_positions(self.molecule, atom);
            for position in positions {
                for replacement in replacements.iter() {
                    let new_molecule = replace_at_index(self.molecule, position, atom, replacement);
                    distinct_molecules.insert(new_molecule);
                }
            }
        }

        distinct_molecules.len().to_string()
    }

    fn solve_part_two(&self) -> String {
        "".to_string()
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

// #[cfg(test)]
// mod part2_tests {
//     use super::*;

//     #[test]
//     fn test_1() {
//         let result = Solver::new("abc").solve_part_two();
//         assert_eq!(result, "0");
//     }
// }
