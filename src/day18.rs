use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Location {
    Empty,
    Byte,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn move_in_direction(
        &self,
        direction: Direction,
        map_width: usize,
        map_height: usize,
    ) -> Option<Position> {
        match direction {
            Direction::Up => {
                if self.y > 0 {
                    Some(Position {
                        x: self.x,
                        y: self.y - 1,
                    })
                } else {
                    None
                }
            }
            Direction::Down => {
                if self.y < map_height - 1 {
                    Some(Position {
                        x: self.x,
                        y: self.y + 1,
                    })
                } else {
                    None
                }
            }
            Direction::Left => {
                if self.x > 0 {
                    Some(Position {
                        x: self.x - 1,
                        y: self.y,
                    })
                } else {
                    None
                }
            }
            Direction::Right => {
                if self.x < map_width - 1 {
                    Some(Position {
                        x: self.x + 1,
                        y: self.y,
                    })
                } else {
                    None
                }
            }
        }
    }

    // fn gps(&self) -> u64 {
    //     (self.y as u64) * 100 + (self.x as u64)
    // }
}

pub fn minimum_steps_to_reach_exit() -> u32 {
    let (map, _) = get_input();
    print_map_with_path(&map, &[]);
    let path = find_shortest_path(&map);
    println!();
    print_map_with_path(&map, &path);
    (path.len() - 1) as u32
}

pub fn get_first_byte_that_blocks_path() -> (usize, usize) {
    let (mut map, remaining_bytes) = get_input();
    print_map_with_path(&map, &[]);
    for byte in remaining_bytes {
        println!("Checking byte at {},{}", byte.x, byte.y);
        map[byte.y][byte.x] = Location::Byte;
        let path = find_shortest_path(&map);
        println!();
        print_map_with_path(&map, &path);
        if path.is_empty() {
            return (byte.x, byte.y);
        }
    }
    (0, 0)
}

fn print_map_with_path(map: &[Vec<Location>], path: &[Position]) {
    for (y, row) in map.iter().enumerate() {
        for (x, location) in row.iter().enumerate() {
            if path.contains(&Position { x, y }) {
                print!("O");
            } else {
                match location {
                    Location::Empty => print!("."),
                    Location::Byte => print!("#"),
                }
            }
        }
        println!();
    }
}

fn get_input() -> (Vec<Vec<Location>>, Vec<Position>) {
    //     let byte_locations = r"5,4
    // 4,2
    // 4,5
    // 3,0
    // 2,1
    // 6,3
    // 2,4
    // 1,5
    // 0,6
    // 3,3
    // 2,6
    // 5,1
    // 1,2
    // 5,5
    // 2,5
    // 6,5
    // 1,4
    // 0,4
    // 6,4
    // 1,1
    // 6,1
    // 1,0
    // 0,5
    // 1,6
    // 2,0";
    //     let number_of_bytes = 12;
    //     let map_width = 7;
    //     let map_height = 7;
    let byte_locations = include_str!("../input/day18.txt");
    let number_of_bytes = 1024;
    let map_width = 71;
    let map_height = 71;
    let mut locations = vec![vec![Location::Empty; map_width]; map_height];
    let mut remaining_bytes = Vec::new();
    for (index, line) in byte_locations.lines().enumerate() {
        let mut parts = line.split(",");
        let x = parts.next().unwrap().parse::<usize>().unwrap();
        let y = parts.next().unwrap().parse::<usize>().unwrap();
        if index < number_of_bytes {
            locations[y][x] = Location::Byte;
        } else {
            remaining_bytes.push(Position { x, y });
        }
    }

    (locations, remaining_bytes)
}

fn find_shortest_path(map: &[Vec<Location>]) -> Vec<Position> {
    let map_width = map[0].len();
    let map_height = map.len();
    let start: Position = Position { x: 0, y: 0 };
    let end: Position = Position {
        x: map_width - 1,
        y: map_height - 1,
    };

    // Use a queue for BFS
    let mut queue: VecDeque<(Position, Vec<Position>)> = VecDeque::new();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; map_width]; map_height];

    // Start with the initial position and an empty path
    queue.push_back((start, vec![start]));
    visited[start.y][start.x] = true;

    while let Some((current_position, path)) = queue.pop_front() {
        // Check if we've reached the end
        if current_position == end {
            return path;
        }

        // Explore all four directions
        for direction in &[
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if let Some(next_position) =
                current_position.move_in_direction(*direction, map_width, map_height)
            {
                // check if the next position is valid

                if !visited[next_position.y][next_position.x] {
                    // check if the next position has been visited
                    if map[next_position.y][next_position.x] != Location::Byte {
                        // check if the next position is not a byte
                        // Mark as visited
                        visited[next_position.y][next_position.x] = true;

                        // Create a new path by extending the current path
                        let mut new_path = path.clone();
                        new_path.push(next_position);

                        // Add to the queue
                        queue.push_back((next_position, new_path));
                    }
                }
            }
        }
    }

    // No path found
    vec![]
}
