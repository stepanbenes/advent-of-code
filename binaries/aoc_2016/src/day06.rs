use std::{cmp::Ordering, collections::HashMap};

use solver::SolverBase;

pub struct Solver {
    lines: Vec<&'static str>,
}

enum ErrorCorrection {
    MostFrequentChar,
    LeastFrequentChar,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        Solver {
            lines: input.lines().collect(),
        }
    }

    fn error_correct(&self, error_correction: ErrorCorrection) -> String {
        let mut code = String::new();
        for i in 0..self.lines[0].len() {
            let mut chart_frequency: HashMap<char, usize> = HashMap::new();
            for line in self.lines.iter() {
                let c = line.chars().nth(i).unwrap();
                *chart_frequency.entry(c).or_default() += 1;
            }

            let compare_fn: fn(&(&char, &usize), &(&char, &usize)) -> Ordering =
                match error_correction {
                    ErrorCorrection::MostFrequentChar => |a, b| a.1.cmp(b.1),
                    ErrorCorrection::LeastFrequentChar => |a, b| b.1.cmp(a.1),
                };
            let (most_frequent_char, _) = chart_frequency.iter().max_by(compare_fn).unwrap();
            code.push(*most_frequent_char);
        }
        code
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        self.error_correct(ErrorCorrection::MostFrequentChar)
    }

    fn solve_part_two(&self) -> String {
        self.error_correct(ErrorCorrection::LeastFrequentChar)
    }

    fn day_number(&self) -> usize {
        6
    }

    fn description(&self) -> &'static str {
        "Error corrected message"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r"eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar",
        )
        .solve_part_one();
        assert_eq!(result, "easter");
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r"eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar",
        )
        .solve_part_two();
        assert_eq!(result, "advent");
    }
}
