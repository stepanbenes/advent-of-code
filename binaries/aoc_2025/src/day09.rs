use solver::SolverBase;

pub struct Solver {
    tile_positions: Vec<(u64, u64)>,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        let tile_positions = input
            .lines()
            .map(|line| line.split_once(',').unwrap())
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .collect();
        Solver { tile_positions }
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let mut max_area = None;
        for i in 0..self.tile_positions.len() {
            for j in i + 1..self.tile_positions.len() {
                let (a_x, a_y) = self.tile_positions[i];
                let (b_x, b_y) = self.tile_positions[j];
                let area = (a_x.abs_diff(b_x) + 1) * (a_y.abs_diff(b_y) + 1);
                if let Some(max) = max_area
                    && area > max
                {
                    max_area = Some(area);
                } else if max_area.is_none() {
                    max_area = Some(area);
                }
            }
        }
        max_area.unwrap().to_string()
    }

    fn solve_part_two(&self) -> String {
        "".to_string()
    }

    fn day_number(&self) -> usize {
        9
    }

    fn description(&self) -> &'static str {
        "Movie Theater"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3",
        )
        .solve_part_one();
        assert_eq!(result, "50");
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
