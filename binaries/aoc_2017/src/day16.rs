use std::str::FromStr;

use solver::SolverBase;

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

    fn dance(&self) -> String {
        let mut programs = self.init_state.chars().collect::<Vec<_>>();
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
        programs.iter().collect()
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        self.dance()
    }

    fn solve_part_two(&self) -> String {
        "".to_string()
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

// #[cfg(test)]
// mod part2_tests {
//     use super::*;

//     #[test]
//     fn test_1() {
//         let result = Solver::new("abc").solve_part_two();
//         assert_eq!(result, "0");
//     }
// }
