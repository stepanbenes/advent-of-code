use std::fmt::Write;
use std::ops::Range;

use solver::SolverBase;

pub struct Solver {
    input: String,
    element_count: usize,
}

impl Solver {
    pub fn new(input: String, element_count: usize) -> Self {
        Solver {
            input,
            element_count,
        }
    }

    fn single_round_of_knot_hash(&self) -> Vec<u8> {
        let lengths: Vec<_> = self.input.split(',').map(|x| x.parse().unwrap()).collect();
        let mut elements: Vec<u8> = (0..=(self.element_count - 1) as u8).collect();
        let mut position = 0;
        let mut skip_size = 0;
        Solver::partial_knot_hash(&lengths, &mut elements, &mut position, &mut skip_size);
        elements
    }

    fn reverse(elements: &mut [u8], range: Range<usize>) {
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

    fn partial_knot_hash(
        lengths: &[usize],
        elements: &mut [u8],
        position: &mut usize,
        skip_size: &mut usize,
    ) {
        for length in lengths.iter() {
            // Reverse the order of that length of elements in the list, starting with the element at the current position.
            Solver::reverse(elements, *position..*position + length);
            // Move the current position forward by that length plus the skip size.
            *position = (*position + *length + *skip_size) % elements.len();
            // Increase the skip size by one
            *skip_size += 1;
        }
    }

    pub fn full_knot_hash(&self) -> String {
        // convert to ascii codes
        let mut lengths: Vec<usize> = self.input.bytes().map(|x| x as usize).collect();
        // extend with sequence
        {
            lengths.extend([17, 31, 73, 47, 23]);
        }
        // run hashing 64 times to create "sparse hash"
        let mut elements: Vec<u8> = (0..=255).collect();
        {
            let mut position = 0;
            let mut skip_size = 0;
            for _ in 0..64 {
                Solver::partial_knot_hash(&lengths, &mut elements, &mut position, &mut skip_size);
            }
        }
        // reduce the list to 16 numbers called "dense hash"
        let mut dense_hash: Vec<u8> = Vec::new();
        {
            for block in elements.chunks(16) {
                assert!(block.len() == 16);
                let mut result = block[0];
                for num in block.iter().skip(1) {
                    result ^= num;
                }
                dense_hash.push(result);
            }
        }
        // convert to hex string
        //dense_hash.into_iter().map(|n| format!("{n:02x}")).collect()
        dense_hash.into_iter().fold(String::new(), |mut acc, n| {
            _ = write!(&mut acc, "{n:02x}");
            acc
        })
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let hash = self.single_round_of_knot_hash();
        let result: u32 = hash.iter().take(2).map(|&x| x as u32).product();
        result.to_string()
    }

    fn solve_part_two(&self) -> String {
        self.full_knot_hash()
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
        let result = Solver::new("3,4,1,5".to_owned(), 5).solve_part_one();
        assert_eq!(result, "12");
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new("".to_owned(), 256).solve_part_two();
        assert_eq!(result, "a2582a3a0e66e6e86e3812dcb672a272");
    }

    #[test]
    fn test_2() {
        let result = Solver::new("AoC 2017".to_owned(), 256).solve_part_two();
        assert_eq!(result, "33efeb34ea91902bb2f59c9920caa6cd");
    }

    #[test]
    fn test_3() {
        let result = Solver::new("1,2,3".to_owned(), 256).solve_part_two();
        assert_eq!(result, "3efbe78a8d82f29979031a4aa0b16a9d");
    }

    #[test]
    fn test_4() {
        let result = Solver::new("1,2,4".to_owned(), 256).solve_part_two();
        assert_eq!(result, "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}
