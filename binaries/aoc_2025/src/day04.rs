use solver::SolverBase;

pub struct Solver {
    grid: Vec<Vec<bool>>,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        let grid = input
            .lines()
            .map(|line| {
                line.bytes()
                    .map(|x| match x {
                        b'@' => true,
                        b'.' => false,
                        _ => panic!("unrecognized grid location"),
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        Solver { grid }
    }

    fn get_number_of_neighbors(grid: &[Vec<bool>], i: usize, j: usize) -> usize {
        let mut count = 0;

        if j > 0 {
            if grid[i][j - 1] {
                count += 1;
            }
            if i > 0 && grid[i - 1][j - 1] {
                count += 1;
            }
        }

        if i > 0 {
            if grid[i - 1][j] {
                count += 1;
            }
            if j < grid[0].len() - 1 && grid[i - 1][j + 1] {
                count += 1;
            }
        }

        if j < grid[0].len() - 1 {
            if grid[i][j + 1] {
                count += 1;
            }
            if i < grid.len() - 1 && grid[i + 1][j + 1] {
                count += 1;
            }
        }

        if i < grid.len() - 1 {
            if grid[i + 1][j] {
                count += 1;
            }
            if j > 0 && grid[i + 1][j - 1] {
                count += 1;
            }
        }

        count
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let mut sum = 0;
        for (i, row) in self.grid.iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                if *value && Solver::get_number_of_neighbors(&self.grid, i, j) < 4 {
                    sum += 1;
                }
            }
        }
        sum.to_string()
    }

    fn solve_part_two(&self) -> String {
        let mut grid = self.grid.clone();
        let mut total_sum = 0;
        loop {
            let mut sum = 0;
            for i in 0..grid.len() {
                let row_len = grid[i].len();
                for j in 0..row_len {
                    if grid[i][j] && Solver::get_number_of_neighbors(&grid, i, j) < 4 {
                        sum += 1;
                        grid[i][j] = false;
                    }
                }
            }
            if sum == 0 {
                break;
            }
            total_sum += sum;
        }
        total_sum.to_string()
    }

    fn day_number(&self) -> usize {
        4
    }

    fn description(&self) -> &'static str {
        "Printing Department"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
        )
        .solve_part_one();
        assert_eq!(result, "13");
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
        )
        .solve_part_two();
        assert_eq!(result, "43");
    }
}
