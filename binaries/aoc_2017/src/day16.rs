use solver::SolverBase;
use std::str::FromStr;

#[derive(Debug)]
enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl FromStr for DanceMove {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[0..1] {
            "s" => Ok(DanceMove::Spin(s[1..].parse()?)),
            "x" => {
                let mut tokens = s[1..].split('/');
                Ok(DanceMove::Exchange(
                    tokens.next().unwrap().parse()?,
                    tokens.next().unwrap().parse()?,
                ))
            }
            "p" => {
                let mut tokens = s[1..].split('/');
                Ok(DanceMove::Partner(
                    tokens.next().unwrap().chars().next().unwrap(),
                    tokens.next().unwrap().chars().next().unwrap(),
                ))
            }
            _ => Err("unrecognized dance move".into()),
        }
    }
}

pub struct Solver {
    init_state: &'static str,
    dance_moves: Vec<DanceMove>,
}

impl Solver {
    pub fn new(init_state: &'static str, dance_moves: &'static str) -> Self {
        let dance_moves = dance_moves
            .split(',')
            .map(|s| DanceMove::from_str(s).unwrap())
            .collect();
        Solver {
            init_state,
            dance_moves,
        }
    }

    #[allow(dead_code)]
    fn dance(&self, programs: &mut [char]) {
        for dance_move in self.dance_moves.iter() {
            match *dance_move {
                DanceMove::Spin(n) => {
                    programs.rotate_right(n);
                }
                DanceMove::Exchange(i, j) => {
                    programs.swap(i, j);
                }
                DanceMove::Partner(a, b) => {
                    for c in programs.iter_mut() {
                        if *c == a {
                            *c = b;
                        } else if *c == b {
                            *c = a;
                        }
                    }
                }
            }
        }
    }

    fn dance_optimized(
        program_count: usize,
        dance_moves: &Vec<DanceMove>,
        repeat: usize,
    ) -> String {
        let mut table: Vec<_> = (0..program_count).collect();
        let start_point = table.clone();
        let mut r = 0;
        while r < repeat {
            for dance_move in dance_moves {
                match *dance_move {
                    DanceMove::Spin(n) => {
                        table.rotate_right(n);
                    }
                    DanceMove::Exchange(i, j) => {
                        table.swap(i, j);
                    }
                    DanceMove::Partner(a, b) => {
                        let idx1 = table
                            .iter()
                            .position(|&x| x == (a as u8 - b'a') as usize)
                            .unwrap();
                        let idx2 = table
                            .iter()
                            .position(|&x| x == (b as u8 - b'a') as usize)
                            .unwrap();
                        table.swap(idx1, idx2);
                    }
                }
            }
            if table == start_point {
                let skip = (repeat / (r + 1) - 1) * (r + 1);
                r += skip;
            }
            r += 1;
        }
        table
            .iter()
            .map(|x| (b'a' as usize + x) as u8 as char)
            .collect()
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        Solver::dance_optimized(self.init_state.len(), &self.dance_moves, 1)
    }

    fn solve_part_two(&self) -> String {
        Solver::dance_optimized(self.init_state.len(), &self.dance_moves, 1_000_000_000)
    }

    fn day_number(&self) -> usize {
        16
    }

    fn description(&self) -> &'static str {
        "Dancing programs"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new("abcde", "s1").solve_part_one();
        assert_eq!(result, "eabcd");
    }

    #[test]
    fn test_2() {
        let result = Solver::new("abcde", "x3/4").solve_part_one();
        assert_eq!(result, "abced");
    }

    #[test]
    fn test_3() {
        let result = Solver::new("abcde", "pe/b").solve_part_one();
        assert_eq!(result, "aecdb");
    }

    #[test]
    fn test_4() {
        let result = Solver::new("abcde", "s1,x3/4,pe/b").solve_part_one();
        assert_eq!(result, "baedc");
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::dance_optimized(
            5,
            &"s1,x3/4,pe/b"
                .split(',')
                .map(|x| DanceMove::from_str(x).unwrap())
                .collect::<Vec<_>>(),
            2,
        );
        assert_eq!(result, "ceadb");
    }
}
