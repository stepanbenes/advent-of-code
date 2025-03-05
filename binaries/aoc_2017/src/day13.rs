use std::collections::HashMap;

use solver::SolverBase;

pub struct Solver {
    layers: Vec<Layer>,
}

#[derive(Debug)]
struct Layer {
    depth: u32,
    range: u32,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        let mut layers = Vec::new();
        for line in input.lines() {
            let mut tokens = line.split(": ");
            let depth = tokens.next().unwrap().parse().unwrap();
            let range = tokens.next().unwrap().parse().unwrap();
            layers.push(Layer { depth, range });
        }
        Solver { layers }
    }

    #[allow(dead_code)]
    fn print_layers(&self, scanner_positions: &HashMap<u32, u32>, packet_position: u32) {
        let max_depth = self.layers.iter().map(|x| x.depth).max().unwrap();
        let max_range = self.layers.iter().map(|x| x.range).max().unwrap();
        for depth in 0..=max_depth {
            print!("{:>2}  ", depth);
        }
        println!();
        for line_index in 0..max_range {
            for depth in 0..=max_depth {
                let contains_scanner = scanner_positions.get(&depth) == Some(&line_index);
                let contains_packet = line_index == 0 && depth == packet_position;
                if let Some(Layer { range, .. }) = self.layers.iter().find(|x| x.depth == depth) {
                    if line_index < *range {
                        match (contains_scanner, contains_packet) {
                            (false, false) => print!("[ ] "),
                            (false, true) => print!("( ) "),
                            (true, false) => print!("[S] "),
                            (true, true) => print!("(S) "),
                        }
                    } else {
                        print!("    ");
                    }
                } else if line_index == 0 && contains_packet {
                    print!("(.) ");
                } else if line_index == 0 {
                    print!("... ");
                } else {
                    print!("    ");
                }
            }
            println!();
        }
        println!();
    }

    fn sawtooth_wave(x: u32, range: u32) -> u32 {
        let period = (range - 1) * 2;
        let mod_x = x % period;

        if mod_x < (range - 1) {
            mod_x
        } else {
            period - mod_x
        }
    }

    fn send_packet(&self, delay: u32, bailout_if_caught: bool) -> (bool, u32) {
        let mut scanner_positions = HashMap::new();

        for layer in &self.layers {
            scanner_positions.insert(layer.depth, 0);
        }
        let max_depth = self.layers.iter().map(|x| x.depth).max().unwrap();
        let mut severity = 0;
        let mut was_caught = false;

        for layer in &self.layers {
            scanner_positions
                .entry(layer.depth)
                .and_modify(|x| *x = Solver::sawtooth_wave(delay, layer.range));
        }

        for depth in 0..=max_depth {
            if let Some(layer) = self.layers.iter().find(|x| x.depth == depth) {
                let is_caught = if let Some(&s) = scanner_positions.get(&layer.depth) {
                    s == 0
                } else {
                    false
                };

                if is_caught {
                    was_caught = true;
                    severity += layer.depth * layer.range;
                    if bailout_if_caught {
                        break;
                    }
                }
            }

            //println!("Picosecond {depth}:");

            //self.print_layers(&scanner_positions, depth);

            for layer in &self.layers {
                scanner_positions
                    .entry(layer.depth)
                    .and_modify(|x| *x = Solver::sawtooth_wave(delay + depth + 1, layer.range));
            }

            //self.print_layers(&scanner_positions, depth);
        }

        (was_caught, severity)
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let (_was_caught, severity) = self.send_packet(0, false);
        severity.to_string()
    }

    fn solve_part_two(&self) -> String {
        for delay in 0..u32::MAX {
            let (was_caught, _) = self.send_packet(delay, true);
            if !was_caught {
                return delay.to_string();
            }
        }
        panic!("no solution")
    }

    fn day_number(&self) -> usize {
        13
    }

    fn description(&self) -> &'static str {
        "Packet scaners"
    }

    fn skip_run(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r"0: 3
1: 2
4: 4
6: 4",
        )
        .solve_part_one();
        assert_eq!(result, "24");
    }

    #[test]
    fn test_sawtooth() {
        let range = 5;
        let data = [
            (0, 0),
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 4),
            (5, 3),
            (6, 2),
            (7, 1),
            (8, 0),
            (9, 1),
            (10, 2),
            (11, 3),
            (12, 4),
            (13, 3),
            (14, 2),
            (15, 1),
            (16, 0),
            (17, 1),
            (18, 2),
            (19, 3),
            (20, 4),
            (21, 3),
            (22, 2),
            (23, 1),
        ];
        for (x, expected_result) in data {
            let result = Solver::sawtooth_wave(x, range);
            assert_eq!(result, expected_result);
        }
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r"0: 3
1: 2
4: 4
6: 4",
        )
        .solve_part_two();
        assert_eq!(result, "10");
    }
}
