use solver::Solver;

pub struct Day01Solver<'a> {
    input: &'a str,
}

impl<'a> Solver<'a> for Day01Solver<'a> {
    type Part1Output = isize;
    type Part2Output = isize;

    fn new(input: &'a str) -> Self {
        Day01Solver { input }
    }

    fn solve_part_one(&self) -> Self::Part1Output {
        self.input.chars().fold(0, |acc, c| match c {
            '(' => acc + 1,
            ')' => acc - 1,
            _ => acc,
        })
    }

    fn solve_part_two(&self) -> Self::Part2Output {
        let mut floor = 0;
        for (i, c) in self.input.chars().enumerate() {
            floor += match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            };
            if floor == -1 {
                return i as isize + 1;
            }
        }
        panic!("Basement not reached");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod part1 {
        use super::*;
        #[test]
        fn floor_0_1() {
            let result = Day01Solver::new("(())").solve_part_one();
            assert_eq!(result, 0);
        }

        #[test]
        fn floor_0_2() {
            let result = Day01Solver::new("()()").solve_part_one();
            assert_eq!(result, 0);
        }

        #[test]
        fn floor_3_1() {
            let result = Day01Solver::new("(((").solve_part_one();
            assert_eq!(result, 3);
        }

        #[test]
        fn floor_3_2() {
            let result = Day01Solver::new("(()(()(").solve_part_one();
            assert_eq!(result, 3);
        }

        #[test]
        fn floor_3_3() {
            let result = Day01Solver::new("))(((((").solve_part_one();
            assert_eq!(result, 3);
        }

        #[test]
        fn floor_neg1_1() {
            let result = Day01Solver::new("())").solve_part_one();
            assert_eq!(result, -1);
        }

        #[test]
        fn floor_neg1_2() {
            let result = Day01Solver::new("))(").solve_part_one();
            assert_eq!(result, -1);
        }

        #[test]
        fn floor_neg3_1() {
            let result = Day01Solver::new(")))").solve_part_one();
            assert_eq!(result, -3);
        }

        #[test]
        fn floor_neg3_2() {
            let result = Day01Solver::new(")())())").solve_part_one();
            assert_eq!(result, -3);
        }
    }

    mod part2 {
        use super::*;
        #[test]
        fn basement_1() {
            let result = Day01Solver::new(")").solve_part_two();
            assert_eq!(result, 1);
        }

        #[test]
        fn basement_5() {
            let result = Day01Solver::new("()())").solve_part_two();
            assert_eq!(result, 5);
        }
    }
}
