use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

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

    fn manhattan_distance_to(&self, other: Position) -> usize {
        (self.x as isize - other.x as isize).unsigned_abs()
            + (self.y as isize - other.y as isize).unsigned_abs()
    }
}

pub fn count_of_cheats_that_save_at_least_100_picoseconds(cheat_length: usize) -> usize {
    let map = get_input();
    print_map(&map);
    let start_position = find_start_position(&map);
    println!();
    let path = find_shortest_path(&map, start_position).unwrap();

    println!("Path time: {}", path.len() - 1);
    //println!("Path: {:?}", path);
    let all_path_lengths = get_all_path_lenghts_when_cheating(&path, cheat_length);
    all_path_lengths
        .iter()
        .sorted_by_key(|(length, _)| *length)
        .rev()
        .for_each(|(length, count)| {
            println!(
                "{} paths saves {} picoseconds",
                count,
                path.len() as isize - *length as isize
            );
        });
    all_path_lengths
        .iter()
        .sorted_by_key(|(length, _)| *length)
        .map(|(length, count)| (count, path.len() as isize - *length as isize))
        .filter(|(_, length)| *length >= 100)
        .map(|(count, _)| count)
        .sum()
}

fn get_all_path_lenghts_when_cheating(
    path: &[Position],
    cheat_length: usize,
) -> HashMap<usize, usize> {
    let mut path_lengths = HashMap::new();
    for cheat_start in 0..path.len() {
        for cheat_end in cheat_start + 4..path.len() {
            let shortcut_length = path[cheat_start].manhattan_distance_to(path[cheat_end]);
            if shortcut_length <= cheat_length {
                let original_length = cheat_end - cheat_start;
                if shortcut_length < original_length {
                    let new_path_length = path.len() - (original_length - shortcut_length);
                    *path_lengths.entry(new_path_length).or_insert(0) += 1;
                }
            }
        }
    }
    path_lengths
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
}

fn find_shortest_path(map: &[Vec<Location>], start_position: Position) -> Option<Vec<Position>> {
    let mut path: Vec<Position> = vec![start_position];
    let mut queue: VecDeque<State> = VecDeque::new();
    let mut visited: HashSet<Position> = HashSet::new();

    queue.push_back(State {
        position: start_position,
    });

    while let Some(State { position }) = queue.pop_front() {
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
                    continue;
                }
                Location::End => {
                    visited.insert(new_position);
                    path.push(new_position);
                    return Some(path);
                }
                Location::Empty => {
                    if visited.insert(new_position) {
                        path.push(new_position);
                    }
                    queue.push_back(State {
                        position: new_position,
                    });
                }
                Location::Start => panic!("Start position found in the middle of the map"),
            }
        }
    }
    None
}
