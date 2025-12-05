use solver::SolverBase;

pub struct Solver {
    rotations: Vec<Rotation>,
}

#[derive(Debug)]
pub enum Rotation {
    Left(u32),
    Right(u32),
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        let rotations = input
            .lines()
            .map(|line| {
                let (dir, value) = line.split_at(1);
                let value: u32 = value.parse().unwrap();
                match dir {
                    "L" => Rotation::Left(value),
                    "R" => Rotation::Right(value),
                    _ => panic!("Invalid direction"),
                }
            })
            .collect();
        Solver {
            rotations,
        }
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let mut dial = 50;
        let full = 100;
        let mut zero_reached_counter = 0;
        for r in &self.rotations {
            match r {
                Rotation::Left(distance) => {
                    dial = (dial + (100 - (distance % full))) % full;
                }
                Rotation::Right(distance) => {
                    dial = (dial + distance) % full;
                }
            }
            if dial == 0 {
                zero_reached_counter += 1;
            }
        }
        zero_reached_counter.to_string()
    }

    fn solve_part_two(&self) -> String {
        let mut dial = 50;
        let full = 100;
        let mut zero_reached_counter = 0;
        for r in &self.rotations {
            match r {
                Rotation::Left(distance) => {

                }
                Rotation::Right(distance) => {

                }
            }
        }
        zero_reached_counter.to_string()
    }

    fn day_number(&self) -> usize {
        0
    }

    fn description(&self) -> &'static str {
        ""
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82").solve_part_one();
        assert_eq!(result, "3");
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82").solve_part_two();
        assert_eq!(result, "6");
    }
}
