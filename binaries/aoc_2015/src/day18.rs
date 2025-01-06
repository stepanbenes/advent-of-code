use solver::SolverBase;

pub struct Solver {
    lights: Vec<Vec<bool>>,
    steps: usize,
}

impl Solver {
    pub fn new(input: &'static str, steps: usize) -> Self {
        let lights = input
            .lines()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();
        Solver { lights, steps }
    }

    fn move_to_next_state(current_state: &[Vec<bool>]) -> Vec<Vec<bool>> {
        let mut new_state = vec![vec![false; current_state[0].len()]; current_state.len()];
        for r in 0..current_state.len() {
            for c in 0..current_state[0].len() {
                let count = Solver::get_count_of_turned_on_neighbors(current_state, r, c);
                // A light which is on stays on when 2 or 3 neighbors are on, and turns off otherwise.
                if current_state[r][c] {
                    new_state[r][c] = count == 2 || count == 3;
                }
                // A light which is off turns on if exactly 3 neighbors are on, and stays off otherwise.
                else {
                    new_state[r][c] = count == 3;
                }
            }
        }
        new_state
    }

    fn turn_corners_on(state: &mut [Vec<bool>]) {
        let rows = state.len();
        let columns = state[0].len();
        state[0][0] = true;
        state[0][columns - 1] = true;
        state[rows - 1][0] = true;
        state[rows - 1][columns - 1] = true;
    }

    fn get_count_of_turned_on_neighbors(lights: &[Vec<bool>], row: usize, column: usize) -> usize {
        let rows = lights.len();
        let columns = lights[0].len();
        let mut neighbors = 0;
        let mut add_neighbor = |r: usize, c: usize| {
            if lights[r][c] {
                neighbors += 1;
            }
        };
        if row > 0 && column > 0 {
            add_neighbor(row - 1, column - 1);
        }
        if row > 0 {
            add_neighbor(row - 1, column);
        }
        if row > 0 && column < columns - 1 {
            add_neighbor(row - 1, column + 1);
        }
        if column < columns - 1 {
            add_neighbor(row, column + 1);
        }
        if row < rows - 1 && column < columns - 1 {
            add_neighbor(row + 1, column + 1);
        }
        if row < rows - 1 {
            add_neighbor(row + 1, column);
        }
        if row < rows - 1 && column > 0 {
            add_neighbor(row + 1, column - 1);
        }
        if column > 0 {
            add_neighbor(row, column - 1);
        }
        neighbors
    }

    #[allow(dead_code)]
    fn print_lights(lights: &[Vec<bool>]) {
        for row in lights.iter() {
            for light in row {
                if *light {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }

    fn get_count_oflights(lights: &[Vec<bool>]) -> usize {
        lights
            .iter()
            .map(|line| line.iter().filter(|light| **light).count())
            .sum()
    }

    fn animate(&self, corners_are_stuck_on: bool) -> usize {
        let mut lights = self.lights.clone();
        if corners_are_stuck_on {
            Solver::turn_corners_on(&mut lights);
        }
        //Solver::print_lights(&lights);
        for _i in 0..self.steps {
            //println!("step {}:", i + 1);
            lights = Solver::move_to_next_state(&lights);
            if corners_are_stuck_on {
                Solver::turn_corners_on(&mut lights);
            }
            //Solver::print_lights(&lights);
        }
        Solver::get_count_oflights(&lights)
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let count = self.animate(false);
        count.to_string()
    }

    fn solve_part_two(&self) -> String {
        let count = self.animate(true);
        count.to_string()
    }

    fn day_number(&self) -> usize {
        18
    }

    fn description(&self) -> &'static str {
        "Conway's Game of Life"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r".#.#.#
...##.
#....#
..#...
#.#..#
####..",
            4,
        )
        .solve_part_one();
        assert_eq!(result, "4");
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r".#.#.#
...##.
#....#
..#...
#.#..#
####..",
            5,
        )
        .solve_part_two();
        assert_eq!(result, "17");
    }
}
