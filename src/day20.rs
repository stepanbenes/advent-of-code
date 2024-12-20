use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    East,
    West,
    North,
    South,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Location {
    Start,
    End,
    Empty,
    Wall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn move_in_direction(&self, direction: Direction) -> Position {
        match direction {
            Direction::East => Position {
                x: self.x + 1,
                y: self.y,
            },
            Direction::West => Position {
                x: self.x - 1,
                y: self.y,
            },
            Direction::North => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::South => Position {
                x: self.x,
                y: self.y + 1,
            },
        }
    }
}

pub fn count_of_cheats_that_save_at_least_100_picoseconds() -> usize {
    let map = get_input();
    print_map(&map);
    let start_position = find_start_position(&map);
    println!();
    let paths = find_all_shortest_paths(&map, start_position);
    for path in &paths {
        println!("Path time: {}", path.len() - 1);
    }
    print_map_with_paths(&map, &paths);
    let slowest_path_time = paths.iter().map(|path| path.len() - 1).max().unwrap();
    paths
        .iter()
        .filter(|path| slowest_path_time - (path.len() - 1) >= 100)
        .count()
}

fn print_map_with_paths(map: &[Vec<Location>], paths: &[HashSet<Position>]) {
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if paths.iter().any(|path| path.contains(&Position { x, y })) {
                match cell {
                    Location::Wall => print!("*"),
                    Location::Start => print!("S"),
                    Location::End => print!("E"),
                    Location::Empty => print!("0"),
                }
            } else {
                match cell {
                    Location::Wall => print!("#"),
                    Location::Start => print!("S"),
                    Location::End => print!("E"),
                    Location::Empty => print!("."),
                }
            }
        }
        println!();
    }
}

fn get_input() -> Vec<Vec<Location>> {
    let input = include_str!("../input/day20.txt");
    //     let input = r"###############
    // #...#...#.....#
    // #.#.#.#.#.###.#
    // #S#...#.#.#...#
    // #######.#.#.###
    // #######.#.#...#
    // #######.#.###.#
    // ###..E#...#...#
    // ###.#######.###
    // #...###...#...#
    // #.#####.#.###.#
    // #.#...#.#.#...#
    // #.#.#.#.#.#.###
    // #...#...#...###
    // ###############";
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Location::Wall,
                    'S' => Location::Start,
                    'E' => Location::End,
                    '.' => Location::Empty,
                    _ => panic!("Invalid character in input"),
                })
                .collect()
        })
        .collect()
}

fn print_map(map: &[Vec<Location>]) {
    for row in map {
        for cell in row {
            match cell {
                Location::Wall => print!("#"),
                Location::Start => print!("S"),
                Location::End => print!("E"),
                Location::Empty => print!("."),
            }
        }
        println!();
    }
}

fn find_start_position(map: &[Vec<Location>]) -> Position {
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if let Location::Start = cell {
                return Position { x, y };
            }
        }
    }
    panic!("Start position not found");
}

struct State {
    position: Position,
    visited: HashSet<Position>,
    cheat_used: bool,
}

fn find_all_shortest_paths(
    map: &[Vec<Location>],
    start_position: Position,
) -> Vec<HashSet<Position>> {
    let mut paths: Vec<HashSet<Position>> = Vec::new();
    let mut queue: VecDeque<State> = VecDeque::new();

    let mut start_path = HashSet::new();
    start_path.insert(start_position);
    queue.push_back(State {
        visited: start_path,
        position: start_position,
        cheat_used: false,
    });

    while let Some(State {
        mut visited,
        position,
        cheat_used,
    }) = queue.pop_front()
    {
        visited.insert(position);
        for direction in [
            Direction::East,
            Direction::West,
            Direction::North,
            Direction::South,
        ] {
            let new_position = position.move_in_direction(direction);
            if new_position.x == 0
                || new_position.y == 0
                || new_position.x >= map[0].len()
                || new_position.y >= map.len()
            {
                continue;
            }
            if visited.contains(&new_position) {
                continue;
            }
            match map[new_position.y][new_position.x] {
                Location::Wall => {
                    //continue;
                    if cheat_used {
                        continue;
                    }
                    let mut new_visited = visited.clone();
                    print!("{} ", new_visited.len());
                    new_visited.insert(new_position);
                    queue.push_back(State {
                        visited: new_visited,
                        position: new_position,
                        cheat_used: true,
                    });
                }
                Location::End => {
                    let mut new_visited = visited.clone();
                    new_visited.insert(new_position);
                    paths.push(new_visited);
                    println!("Path found: {}", paths.len() - 1);
                }
                Location::Empty => {
                    let mut new_visited = visited.clone();
                    new_visited.insert(new_position);
                    queue.push_back(State {
                        visited: new_visited,
                        position: new_position,
                        cheat_used,
                    });
                }
                Location::Start => panic!("Start position found in the middle of the map"),
            }
        }
    }
    paths
}
