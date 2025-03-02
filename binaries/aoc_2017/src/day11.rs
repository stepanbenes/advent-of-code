use solver::SolverBase;

pub struct Solver {
    directions: Vec<Direction>,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North = 0,
    Northeast = 1,
    Southeast = 2,
    South = 3,
    Southwest = 4,
    Northwest = 5,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        let mut directions = Vec::new();
        for token in input.split(',') {
            let direction = match token {
                "n" => Direction::North,
                "ne" => Direction::Northeast,
                "se" => Direction::Southeast,
                "s" => Direction::South,
                "sw" => Direction::Southwest,
                "nw" => Direction::Northwest,
                _ => panic!("unsupported direction"),
            };
            directions.push(direction);
        }
        Solver { directions }
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let mut dir_table = [0u32; 6];

        let get_triplet_to_merge = |table: &[u32; 6]| {
            for center_index in 0..6 {
                let left_index = if center_index == 0 {
                    5
                } else {
                    center_index - 1
                };
                let right_index = if center_index == 5 {
                    0
                } else {
                    center_index + 1
                };
                if table[left_index] > 0 && table[right_index] > 0 {
                    return Some((left_index, center_index, right_index));
                }
            }
            None
        };

        let get_duplet_to_equalize = |table: &[u32; 6]| {
            for index in 0..6 {
                let mut other_index = index + 3;
                if other_index > 5 {
                    other_index -= 6;
                }
                if table[index] > 0 && table[other_index] > 0 {
                    return Some((index, other_index));
                }
            }
            None
        };

        for direction in self.directions.iter() {
            dir_table[*direction as usize] += 1;
        }

        while let Some((left_index, center_index, right_index)) = get_triplet_to_merge(&dir_table) {
            let left_value = dir_table[left_index];
            let right_value = dir_table[right_index];
            let min_value = left_value.min(right_value);
            dir_table[left_index] -= min_value;
            dir_table[right_index] -= min_value;
            dir_table[center_index] += min_value;
        }

        while let Some((index, other_index)) = get_duplet_to_equalize(&dir_table) {
            let value = dir_table[index];
            let other_value = dir_table[other_index];
            let min_value = value.min(other_value);
            dir_table[index] -= min_value;
            dir_table[other_index] -= min_value;
        }

        dir_table.iter().sum::<u32>().to_string()
    }

    fn solve_part_two(&self) -> String {
        "".to_string()
    }

    fn day_number(&self) -> usize {
        11
    }

    fn description(&self) -> &'static str {
        "Honeycomb"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new("ne,ne,ne").solve_part_one();
        assert_eq!(result, "3");
    }

    #[test]
    fn test_2() {
        let result = Solver::new("ne,ne,sw,sw").solve_part_one();
        assert_eq!(result, "0");
    }

    #[test]
    fn test_3() {
        let result = Solver::new("ne,ne,s,s").solve_part_one();
        assert_eq!(result, "2");
    }

    #[test]
    fn test_4() {
        let result = Solver::new("se,sw,se,sw,sw").solve_part_one();
        assert_eq!(result, "3");
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
