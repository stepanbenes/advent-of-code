use solver::SolverBase;
use std::ops::RangeInclusive;

pub struct Solver {
    ranges: Vec<RangeInclusive<u64>>,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        let ranges = input
            .split(',')
            .map(|line| {
                let (from, to) = line.split_once('-').unwrap();
                from.parse().unwrap()..=to.parse().unwrap()
            })
            .collect();
        Solver { ranges }
    }

    fn sum_of_invalid<F>(&self, is_invalid: &F) -> u64
    where
        F: Fn(u64) -> bool,
    {
        let mut sum = 0;
        for range in &self.ranges {
            for x in range.clone() {
                if is_invalid(x) {
                    sum += x;
                }
            }
        }
        sum
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        fn is_doubled(number: u64) -> bool {
            let s = number.to_string();
            let (left, right) = s.split_at(s.len() / 2);
            left == right
        }

        let sum = self.sum_of_invalid(&is_doubled);
        sum.to_string()
    }

    fn solve_part_two(&self) -> String {
        fn has_repetitions(number: u64) -> bool {
            let s = number.to_string();
            let len = s.len();

            for chunk_size in 1..=len / 2 {
                if !len.is_multiple_of(chunk_size) {
                    continue;
                }

                let bytes = s.as_bytes();
                let first_chunk = &bytes[..chunk_size];

                if bytes
                    .chunks_exact(chunk_size)
                    .all(|chunk| chunk == first_chunk)
                {
                    return true;
                }
            }
            false
        }

        let sum = self.sum_of_invalid(&has_repetitions);
        sum.to_string()
    }

    fn day_number(&self) -> usize {
        2
    }

    fn description(&self) -> &'static str {
        "Gift Shop"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124").solve_part_one();
        assert_eq!(result, "1227775554");
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124").solve_part_two();
        assert_eq!(result, "4174379265");
    }
}
