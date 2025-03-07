use solver::SolverBase;
use std::{collections::HashSet, fmt::Write};

use crate::day10;

type Disk = [[bool; 128]; 128];

pub struct Solver {
    input: &'static str,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        Solver { input }
    }

    fn get_disk(&self) -> Disk {
        fn hex_to_binary(hex: &str, disk_partition: &mut [bool]) {
            let ones_and_zeroes = hex.chars().fold(String::new(), |mut acc, c| {
                _ = write!(&mut acc, "{:04b}", c.to_digit(16).unwrap());
                acc
            });

            for (i, c) in ones_and_zeroes.chars().enumerate() {
                disk_partition[i] = c == '1';
            }
        }
        let mut disk: Disk = [[false; 128]; 128];
        for (i, row) in disk.iter_mut().enumerate() {
            let hasher_input = format!("{}-{}", self.input, i);
            let knot_hash = day10::Solver::new(hasher_input, 256).full_knot_hash();
            hex_to_binary(&knot_hash, row);
        }
        disk
    }

    fn dfs(start: (usize, usize), disk: &Disk, visited: &mut HashSet<(usize, usize)>) {
        if disk[start.0][start.1] && !visited.contains(&start) {
            visited.insert(start);
            if start.0 > 0 {
                Solver::dfs((start.0 - 1, start.1), disk, visited);
            }
            if start.0 < 127 {
                Solver::dfs((start.0 + 1, start.1), disk, visited);
            }
            if start.1 > 0 {
                Solver::dfs((start.0, start.1 - 1), disk, visited);
            }
            if start.1 < 127 {
                Solver::dfs((start.0, start.1 + 1), disk, visited);
            }
        }
    }

    fn get_number_of_regions(disk: &Disk) -> usize {
        let mut visited = HashSet::new();
        let mut counter = 0;
        for i in 0..128 {
            for j in 0..128 {
                if disk[i][j] && !visited.contains(&(i, j)) {
                    counter += 1;
                    Solver::dfs((i, j), disk, &mut visited);
                }
            }
        }
        counter
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        #[allow(dead_code)]
        fn hex_to_binary(hex: &str) -> String {
            // hex.chars()
            //     .map(|c| format!("{:04b}", c.to_digit(16).unwrap()))
            //     .collect()

            hex.chars().fold(String::new(), |mut acc, c| {
                _ = write!(&mut acc, "{:04b}", c.to_digit(16).unwrap());
                acc
            })
        }

        fn count_ones_in_hex(hex: &str) -> usize {
            hex.chars()
                .map(|c| c.to_digit(16).unwrap().count_ones() as usize)
                .sum()
        }

        let mut count_of_used_squares = 0;

        for i in 0..128 {
            let hasher_input = format!("{}-{}", self.input, i);
            //println!("{hasher_input}");
            let knot_hash = day10::Solver::new(hasher_input, 256).full_knot_hash();
            //println!("{knot_hash}");
            //let binary_string = hex_to_binary(&knot_hash);
            //println!("{binary_string}");
            let sum = count_ones_in_hex(&knot_hash);
            count_of_used_squares += sum;
        }

        count_of_used_squares.to_string()
    }

    fn solve_part_two(&self) -> String {
        let disk = self.get_disk();
        let number_of_regions = Solver::get_number_of_regions(&disk);
        number_of_regions.to_string()
    }

    fn day_number(&self) -> usize {
        14
    }

    fn description(&self) -> &'static str {
        "Disk defragmentation - knot hash reuse"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new("flqrgnkx").solve_part_one();
        assert_eq!(result, "8108");
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new("flqrgnkx").solve_part_two();
        assert_eq!(result, "1242");
    }
}
