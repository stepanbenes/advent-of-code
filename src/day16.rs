use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    East,
    West,
    North,
    South,
}

impl Direction {
    fn rotate_clockwise(&self) -> Direction {
        match self {
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::North => Direction::East,
        }
    }

    fn rotate_counter_clockwise(&self) -> Direction {
        match self {
            Direction::East => Direction::North,
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct State {
    cost: u32,
    position: Position,
    direction: Direction,
}

// Custom comparison to make BinaryHeap a min-heap
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn lowest_score_a_reindeer_could_possibly_get() -> u32 {
    let map = get_input();
    print_map(&map);
    let start_position = find_start_position(&map);
    //let price = walk_through_map(&mut map, start_position, Direction::East);
    let (price, path) = cheapest_path_through_maze(&map, start_position, 1000, 1000, 1).unwrap();
    //print_map(&map);
    println!("Price: {}", price);
    for (position, direction) in path.iter() {
        println!("[{}, {}] {:?}", position.x, position.y, direction);
    }
    print_map_with_path(&map, &path);
    price
}

fn print_map_with_path(map: &[Vec<Location>], path: &[(Position, Direction)]) {
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let mut direction_to_print = None;
            for (position, direction) in path {
                if position.x == x && position.y == y {
                    direction_to_print = Some(direction);
                }
            }
            if let Some(direction) = direction_to_print {
                match direction {
                    Direction::East => print!(">"),
                    Direction::West => print!("<"),
                    Direction::North => print!("^"),
                    Direction::South => print!("v"),
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
    let input = include_str!("../input/day16.txt");
    //     let input = r"###############
    // #.......#....E#
    // #.#.###.#.###.#
    // #.....#.#...#.#
    // #.###.#####.#.#
    // #.#.#.......#.#
    // #.#.#####.###.#
    // #...........#.#
    // ###.#.#####.#.#
    // #...#.....#.#.#
    // #.#.#.###.#.#.#
    // #.....#...#.#.#
    // #.###.#.#.#.#.#
    // #S..#.....#...#
    // ###############";
    // let input = r"#################
    // #...#...#...#..E#
    // #.#.#.#.#.#.#.#.#
    // #.#.#.#...#...#.#
    // #.#.#.#.###.#.#.#
    // #...#.#.#.....#.#
    // #.#.#.#.#.#####.#
    // #.#...#.#.#.....#
    // #.#.#####.#.###.#
    // #.#.#.......#...#
    // #.#.###.#####.###
    // #.#.#...#.....#.#
    // #.#.#.#####.###.#
    // #.#.#.........#.#
    // #.#.#.#########.#
    // #S#.............#
    // #################";
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

fn cheapest_path_through_maze(
    map: &[Vec<Location>],
    start: Position,
    rotate_cw_cost: u32,
    rotate_ccw_cost: u32,
    forward_cost: u32,
) -> Option<(u32, Vec<(Position, Direction)>)> {
    let mut visited: HashSet<(Position, Direction)> = HashSet::new();
    let mut pq = BinaryHeap::new(); // Priority queue: stores State
    let mut parent: HashMap<(Position, Direction), (Position, Direction)> = HashMap::new(); // To track predecessors

    // Push all directions into the priority queue
    pq.push(Reverse(State {
        cost: 0,
        position: start,
        direction: Direction::East,
    }));

    while let Some(Reverse(current)) = pq.pop() {
        // If we reach the goal, reconstruct the path
        if map[current.position.y][current.position.x] == Location::End {
            let mut path = vec![];
            let mut state = (current.position, current.direction);

            // Backtrack to reconstruct the path
            while let Some(&prev) = parent.get(&state) {
                path.push(state); // Only store positions in the path
                state = prev;
            }
            path.push((start, Direction::East)); // Add the starting position
            path.reverse(); // Reverse to get the path from start to goal

            return Some((current.cost, path));
        }

        // If we've already visited this state, skip it
        if !visited.insert((current.position, current.direction)) {
            continue;
        }

        // Rotate clockwise
        let next_dir_cw = current.direction.rotate_clockwise();
        let cw_state = (current.position, next_dir_cw);
        if !visited.contains(&cw_state) {
            pq.push(Reverse(State {
                cost: current.cost + rotate_cw_cost,
                position: current.position,
                direction: next_dir_cw,
            }));
            parent.insert(cw_state, (current.position, current.direction));
        }

        // Rotate counter-clockwise
        let next_dir_ccw = current.direction.rotate_counter_clockwise();
        let ccw_state = (current.position, next_dir_ccw);
        if !visited.contains(&ccw_state) {
            pq.push(Reverse(State {
                cost: current.cost + rotate_ccw_cost,
                position: current.position,
                direction: next_dir_ccw,
            }));
            parent.insert(ccw_state, (current.position, current.direction));
        }

        // Move forward
        let next_position = current.position.move_in_direction(current.direction);
        if next_position.y < map.len() && next_position.x < map[0].len() {
            let next_location = map[next_position.y][next_position.x];

            if next_location != Location::Wall {
                let forward_state = (next_position, current.direction);
                if !visited.contains(&forward_state) {
                    pq.push(Reverse(State {
                        cost: current.cost + forward_cost,
                        position: next_position,
                        direction: current.direction,
                    }));
                    parent.insert(forward_state, (current.position, current.direction));
                }
            }
        }
    }

    // If no path is found
    None
}
