use solver::SolverBase;

pub struct Solver {
    grid: Vec<Vec<Location>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Location {
    Empty,
    Start,
    Splitter,
    Visited(u64),
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        let grid = input
            .lines()
            .map(|line| {
                line.bytes()
                    .map(|b| match b {
                        b'.' => Location::Empty,
                        b'S' => Location::Start,
                        b'^' => Location::Splitter,
                        b'|' => panic!("beam not expected in input"),
                        _ => panic!("unknown location"),
                    })
                    .collect()
            })
            .collect();
        Solver { grid }
    }

    fn find_start(grid: &[Vec<Location>]) -> Option<(usize, usize)> {
        for (i, row) in grid.iter().enumerate() {
            for (j, location) in row.iter().enumerate() {
                if *location == Location::Start {
                    return Some((i, j));
                }
            }
        }
        None
    }

    pub fn tachion_beam_traverse(grid: &mut [Vec<Location>]) -> usize {
        fn dfs(grid: &mut [Vec<Location>], location: (usize, usize)) -> usize {
            if location.0 >= grid.len() {
                return 0; // left the grid
            }
            match grid[location.0][location.1] {
                Location::Start => dfs(grid, (location.0 + 1, location.1)), // continue down
                Location::Visited(_) => 0,                                  // already visited
                Location::Splitter => {
                    1 + dfs(grid, (location.0 + 1, location.1 - 1))
                        + dfs(grid, (location.0 + 1, location.1 + 1))
                } // split beam, add +1
                Location::Empty => {
                    grid[location.0][location.1] = Location::Visited(1); // mark as visited
                    dfs(grid, (location.0 + 1, location.1)) // continue down
                }
            }
        }

        dfs(grid, Solver::find_start(grid).unwrap())
    }

    pub fn tachion_beam_path_counter(grid: &mut [Vec<Location>]) -> u64 {
        fn dfs(grid: &mut [Vec<Location>], location: (usize, usize)) -> u64 {
            if location.0 >= grid.len() {
                return 1; // left the grid
            }
            match grid[location.0][location.1] {
                Location::Start => dfs(grid, (location.0 + 1, location.1)), // continue down
                Location::Visited(count) => count,                          // already occupied
                Location::Splitter => {
                    let left_count = dfs(grid, (location.0 + 1, location.1 - 1));
                    let right_count = dfs(grid, (location.0 + 1, location.1 + 1));
                    left_count + right_count
                } // split beam, add +1
                Location::Empty => {
                    let count = dfs(grid, (location.0 + 1, location.1)); // continue down
                    grid[location.0][location.1] = Location::Visited(count); // mark as visited
                    count
                }
            }
        }

        dfs(grid, Solver::find_start(grid).unwrap())
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let mut grid = self.grid.clone();
        let split_count = Solver::tachion_beam_traverse(&mut grid);
        split_count.to_string()
    }

    fn solve_part_two(&self) -> String {
        let mut grid = self.grid.clone();
        let path_count = Solver::tachion_beam_path_counter(&mut grid);
        path_count.to_string()
    }

    fn day_number(&self) -> usize {
        7
    }

    fn description(&self) -> &'static str {
        "Laboratories"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............",
        )
        .solve_part_one();
        assert_eq!(result, "21");
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............",
        )
        .solve_part_two();
        assert_eq!(result, "40");
    }
}
