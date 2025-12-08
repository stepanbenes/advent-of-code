use std::ops::RangeInclusive;

use solver::SolverBase;

pub struct Solver {
    fresh_ranges: Vec<RangeInclusive<u64>>,
    available_ingredients: Vec<u64>,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        let mut fresh_ranges = Vec::<RangeInclusive<u64>>::new();
        let mut available_ingredients = Vec::<u64>::new();
        let mut parsing_ranges = true;
        for line in input.lines() {
            if line.is_empty() {
                parsing_ranges = false;
            } else if parsing_ranges {
                let (from, to) = line.split_once('-').unwrap();
                let range = from.parse().unwrap()..=to.parse().unwrap();
                fresh_ranges.push(range);
            } else {
                let id = line.parse().unwrap();
                available_ingredients.push(id);
            }
        }
        Solver {
            fresh_ranges,
            available_ingredients,
        }
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let mut count = 0;
        for id in &self.available_ingredients {
            for range in &self.fresh_ranges {
                if range.contains(id) {
                    count += 1;
                    break;
                }
            }
        }
        count.to_string()
    }

    fn solve_part_two(&self) -> String {
        let mut ranges = self.fresh_ranges.clone();
        ranges.sort_by_key(|range| *range.start());
        let mut current = 1;
        while current < ranges.len() {
            let previous = current - 1;
            if ranges[current].start() <= ranges[previous].end() {
                if ranges[current].end() > ranges[previous].end() {
                    ranges[previous] = *ranges[previous].start()..=*ranges[current].end();
                }
                ranges.remove(current);
            } else {
                current += 1;
            }
        }
        let mut total_count = 0;
        for range in ranges {
            total_count += range.end() - range.start() + 1;
        }
        total_count.to_string()
    }

    fn day_number(&self) -> usize {
        5
    }

    fn description(&self) -> &'static str {
        "Cafeteria"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r"3-5
10-14
16-20
12-18

1
5
8
11
17
32",
        )
        .solve_part_one();
        assert_eq!(result, "3");
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r"3-5
10-14
16-20
12-18

1
5
8
11
17
32",
        )
        .solve_part_two();
        assert_eq!(result, "14");
    }
}
