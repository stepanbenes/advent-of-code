#![allow(clippy::needless_range_loop)]
use solver::SolverBase;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Command {
    action: Action,
    from: Position,
    to: Position,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

const GRID_WIDTH: usize = 1000;
const GRID_HEIGHT: usize = 1000;

pub struct Solver {
    commands: Vec<Command>,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        fn parse_command(text: &str) -> Command {
            let tokens: Vec<_> = text.split([' ', ',']).collect();
            match &tokens[..] {
                ["turn", "on", from_x, from_y, "through", to_x, to_y] => Command {
                    action: Action::TurnOn,
                    from: Position {
                        x: from_x.parse().unwrap(),
                        y: from_y.parse().unwrap(),
                    },
                    to: Position {
                        x: to_x.parse().unwrap(),
                        y: to_y.parse().unwrap(),
                    },
                },
                ["turn", "off", from_x, from_y, "through", to_x, to_y] => Command {
                    action: Action::TurnOff,
                    from: Position {
                        x: from_x.parse().unwrap(),
                        y: from_y.parse().unwrap(),
                    },
                    to: Position {
                        x: to_x.parse().unwrap(),
                        y: to_y.parse().unwrap(),
                    },
                },
                ["toggle", from_x, from_y, "through", to_x, to_y] => Command {
                    action: Action::Toggle,
                    from: Position {
                        x: from_x.parse().unwrap(),
                        y: from_y.parse().unwrap(),
                    },
                    to: Position {
                        x: to_x.parse().unwrap(),
                        y: to_y.parse().unwrap(),
                    },
                },
                _ => panic!("unexpected input"),
            }
        }

        let commands = input.lines().map(parse_command).collect();
        Solver { commands }
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let mut grid = vec![vec![false; GRID_WIDTH]; GRID_HEIGHT];

        for command in self.commands.iter() {
            match command.action {
                Action::TurnOn => {
                    for i in command.from.y..=command.to.y {
                        for j in command.from.x..=command.to.x {
                            grid[i][j] = true;
                        }
                    }
                }
                Action::TurnOff => {
                    for i in command.from.y..=command.to.y {
                        for j in command.from.x..=command.to.x {
                            grid[i][j] = false;
                        }
                    }
                }
                Action::Toggle => {
                    for i in command.from.y..=command.to.y {
                        for j in command.from.x..=command.to.x {
                            grid[i][j] = !grid[i][j];
                        }
                    }
                }
            }
        }

        fn count_lights(grid: &[Vec<bool>]) -> usize {
            let mut counter = 0;
            for i in 0..GRID_HEIGHT {
                for j in 0..GRID_WIDTH {
                    if grid[i][j] {
                        counter += 1;
                    }
                }
            }
            counter
        }
        count_lights(&grid).to_string()
    }

    fn solve_part_two(&self) -> String {
        let mut grid = vec![vec![0u32; GRID_WIDTH]; GRID_HEIGHT];

        for command in self.commands.iter() {
            match command.action {
                Action::TurnOn => {
                    for i in command.from.y..=command.to.y {
                        for j in command.from.x..=command.to.x {
                            grid[i][j] += 1;
                        }
                    }
                }
                Action::TurnOff => {
                    for i in command.from.y..=command.to.y {
                        for j in command.from.x..=command.to.x {
                            grid[i][j] = grid[i][j].saturating_sub(1);
                        }
                    }
                }
                Action::Toggle => {
                    for i in command.from.y..=command.to.y {
                        for j in command.from.x..=command.to.x {
                            grid[i][j] += 2;
                        }
                    }
                }
            }
        }

        fn count_brightness(grid: &[Vec<u32>]) -> usize {
            let mut counter = 0;
            for i in 0..GRID_HEIGHT {
                for j in 0..GRID_WIDTH {
                    counter += grid[i][j];
                }
            }
            counter as usize
        }
        count_brightness(&grid).to_string()
    }

    fn day_number(&self) -> usize {
        6
    }

    fn description(&self) -> &'static str {
        "Million lights grid"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new("turn on 0,0 through 999,999").solve_part_one();
        assert_eq!(result, "1000000");
    }

    #[test]
    fn test_2() {
        let result = Solver::new("toggle 0,0 through 999,0").solve_part_one();
        assert_eq!(result, "1000");
    }

    #[test]
    fn test_3() {
        let result = Solver::new("turn off 499,499 through 500,500").solve_part_one();
        assert_eq!(result, "0");
    }

    #[test]
    fn test_4() {
        let result = Solver::new(
            r"turn on 0,0 through 999,999
toggle 0,0 through 999,0
turn off 499,499 through 500,500",
        )
        .solve_part_one();
        assert_eq!(result, (1000_000 - 1000 - 4).to_string());
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new("toggle 0,0 through 999,999").solve_part_two();
        assert_eq!(result, "2000000");
    }

    #[test]
    fn test_2() {
        let result = Solver::new(
            r"turn on 0,0 through 0,0
turn on 0,0 through 0,0
turn on 0,0 through 0,0
turn on 0,0 through 0,0
turn on 0,0 through 0,0",
        )
        .solve_part_two();
        assert_eq!(result, "5");
    }

    #[test]
    fn test_3() {
        let result = Solver::new(
            r"turn on 0,0 through 0,0
turn off 0,0 through 0,0
turn on 0,0 through 0,0
turn off 0,0 through 0,0
toggle 0,0 through 0,0",
        )
        .solve_part_two();
        assert_eq!(result, "2");
    }
}
