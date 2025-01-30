use itertools::Itertools;
use solver::SolverBase;

struct Ip {
    segments: Vec<&'static str>,
    hypernet_sequences: Vec<&'static str>,
}

pub struct Solver {
    ips: Vec<Ip>,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        let mut ips = Vec::new();
        for line in input.lines() {
            let parts: Vec<_> = line.split(['[', ']']).collect();
            let mut segments = Vec::new();
            let mut hypernet_sequences = Vec::new();
            let mut current_vec;
            for (i, part) in parts.iter().enumerate() {
                current_vec = if i % 2 == 0 {
                    &mut segments
                } else {
                    &mut hypernet_sequences
                };
                current_vec.push(*part);
            }
            ips.push(Ip {
                segments,
                hypernet_sequences,
            });
        }
        Solver { ips }
    }

    fn is_valid(ip: &Ip) -> bool {
        ip.segments.iter().any(|x| Solver::has_abba(x))
            && ip.hypernet_sequences.iter().all(|x| !Solver::has_abba(x))
    }

    fn has_abba(segment: &'static str) -> bool {
        for (a, b, c, d) in segment.chars().tuple_windows() {
            if a == d && b == c && a != b {
                return true;
            }
        }
        false
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let count = self.ips.iter().filter(|ip| Solver::is_valid(ip)).count();
        count.to_string()
    }

    fn solve_part_two(&self) -> String {
        "".to_string()
    }

    fn day_number(&self) -> usize {
        7
    }

    fn description(&self) -> &'static str {
        "IPv7, TLS support"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new("abba[mnop]qrst").solve_part_one();
        assert_eq!(result, "1");
    }

    #[test]
    fn test_2() {
        let result = Solver::new("abcd[bddb]xyyx").solve_part_one();
        assert_eq!(result, "0");
    }

    #[test]
    fn test_3() {
        let result = Solver::new("aaaa[qwer]tyui").solve_part_one();
        assert_eq!(result, "0");
    }

    #[test]
    fn test_4() {
        let result = Solver::new("ioxxoj[asdfgh]zxcvbn").solve_part_one();
        assert_eq!(result, "1");
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new("aba[bab]xyz").solve_part_two();
        assert_eq!(result, "1");
    }

    #[test]
    fn test_2() {
        let result = Solver::new("xyx[xyx]xyx").solve_part_two();
        assert_eq!(result, "0");
    }

    #[test]
    fn test_3() {
        let result = Solver::new("aaa[kek]eke").solve_part_two();
        assert_eq!(result, "1");
    }

    #[test]
    fn test_4() {
        let result = Solver::new("zazbz[bzb]cdb").solve_part_two();
        assert_eq!(result, "1");
    }
}
