use std::collections::{HashMap, HashSet};

use solver::SolverBase;

pub struct Solver {
    devices: HashMap<&'static str, Vec<&'static str>>,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        let mut devices = HashMap::new();
        for line in input.lines() {
            let (device_name, outputs_str) = line.split_once(':').unwrap();
            let output_names = outputs_str.split_whitespace().collect();
            devices.insert(device_name, output_names);
        }
        Solver { devices }
    }

    pub fn count_paths(&self, from: &'static str, to: &'static str) -> usize {
        if from == to {
            return 1;
        }
        let mut sum = 0;
        for output in self.devices.get(from).unwrap() {
            sum += self.count_paths(output, to);
        }
        sum
    }

    pub fn count_paths_that_visit(
        &self,
        from: &'static str,
        to: &'static str,
        must_visit: &[&'static str],
    ) -> usize {
        self.count_paths_that_contains_devices(from, to, &must_visit.to_vec())
    }

    fn count_paths_that_contains_devices(&self, from: &'static str, to: &'static str, must_visit: &Vec<&'static str>) -> usize {
        if from == to {
            return if must_visit.is_empty() { 1 } else { 0 };
        }
        let must_visit = if must_visit.contains(&from) { &must_visit.iter().filter(|x| **x != from).map(|x| *x).collect::<Vec<&'static str>>() } else { must_visit };
        let mut sum = 0;
        for output in self.devices.get(from).unwrap() {
            sum += self.count_paths_that_contains_devices(output, to, must_visit);
        }
        sum
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let count = self.count_paths("you", "out");
        count.to_string()
    }

    fn solve_part_two(&self) -> String {
        let count = self.count_paths_that_visit("svr", "out", &["dac", "fft"]);
        count.to_string()
    }

    fn day_number(&self) -> usize {
        11
    }

    fn description(&self) -> &'static str {
        "Reactor"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out",
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
            r"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out",
        )
        .solve_part_two();
        assert_eq!(result, "2");
    }
}
