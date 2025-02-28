use std::ops::Range;

use solver::SolverBase;

pub struct Solver {
    lengths: &'static [usize],
    element_count: usize,
}

impl Solver {
    pub fn new(input: &'static [usize], element_count: usize) -> Self {
        Solver {
            lengths: input,
            element_count,
        }
    }

    fn knot_hash(&self) -> Vec<u32> {
        let mut elements: Vec<u32> = (0..self.element_count as u32).collect();
        let mut position: usize = 0;
        for (skip_size, length) in self.lengths.iter().enumerate() {
            // Reverse the order of that length of elements in the list, starting with the element at the current position.
            Solver::reverse(&mut elements, position..position + length);
            // Move the current position forward by that length plus the skip size.
            position = (position + length + skip_size) % self.element_count;
            // Increase the skip size by one (done using enumerate())
        }
        elements
    }

    fn reverse(elements: &mut [u32], range: Range<usize>) {
        let mut slice = Vec::new();
        for i in range.clone() {
            let position = i % elements.len();
            slice.push(elements[position]);
        }
        for i in range {
            let position = i % elements.len();
            elements[position] = slice.pop().unwrap();
        }
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let hash = self.knot_hash();
        let result: u32 = hash.iter().take(2).product();
        result.to_string()
    }

    fn solve_part_two(&self) -> String {
        "".to_string()
    }

    fn day_number(&self) -> usize {
        10
    }

    fn description(&self) -> &'static str {
        "Knot hash"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(&[3, 4, 1, 5], 5).solve_part_one();
        assert_eq!(result, "12");
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
