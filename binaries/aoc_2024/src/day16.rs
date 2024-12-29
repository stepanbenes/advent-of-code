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

type Path = Vec<(Position, Direction)>;

pub fn lowest_score_a_reindeer_could_possibly_get() -> (u32, usize) {
    let map = get_input();
    print_map(&map);
    let start_position = find_start_position(&map);
    let (price, visited_positions, best_paths) =
        find_all_shortest_paths(&map, start_position, Direction::East, 1000, 1000, 1).unwrap();

    print_map_with_visited_positions(&map, &visited_positions);

    print_map_with_paths(&map, &best_paths);

    (price, visited_positions.len())
}

fn print_map_with_visited_positions(map: &[Vec<Location>], visited_positions: &HashSet<Position>) {
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if visited_positions.contains(&Position { x, y }) {
                print!("O");
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

fn print_map_with_paths(map: &[Vec<Location>], paths: &[Path]) {
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let mut direction_to_print = None;
            for path in paths {
                for (position, direction) in path {
                    if position.x == x && position.y == y {
                        direction_to_print = Some(direction);
                    }
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

fn find_all_shortest_paths(
    map: &[Vec<Location>],
    start_position: Position,
    start_direction: Direction,
    rotate_cw_cost: u32,
    rotate_ccw_cost: u32,
    forward_cost: u32,
) -> Option<(u32, HashSet<Position>, Vec<Path>)> {
    let mut dist: HashMap<(Position, Direction), u32> = HashMap::new();
    let mut paths: HashMap<(Position, Direction), Vec<Path>> = HashMap::new();
    let mut heap = BinaryHeap::new();
    let mut visited_positions = HashSet::new();
    let mut visited_paths = HashSet::new();
    let mut min_cost = u32::MAX;

    // Initial state
    heap.push(Reverse(State {
        cost: 0,
        position: start_position,
        direction: start_direction,
    }));

    dist.insert((start_position, start_direction), 0);
    paths.insert(
        (start_position, start_direction),
        vec![vec![(start_position, start_direction)]],
    );

    while let Some(Reverse(current)) = heap.pop() {
        // Skip if we've found a cheaper path to this state
        if let Some(&existing_cost) = dist.get(&(current.position, current.direction)) {
            if current.cost > existing_cost {
                continue;
            }
        }

        // Track visited positions
        visited_positions.insert(current.position);

        // Check if we've reached an end position
        if map[current.position.y][current.position.x] == Location::End {
            if current.cost <= min_cost {
                // Update min cost
                if current.cost < min_cost {
                    min_cost = current.cost;
                    visited_paths.clear();
                }

                // Add paths for this endpoint
                if let Some(endpoint_paths) = paths.get(&(current.position, current.direction)) {
                    visited_paths.extend(endpoint_paths.iter().cloned());
                }
            } else {
                break;
            }
        }

        let mut context = PathUpdateContext {
            dist: &mut dist,
            paths: &mut paths,
            heap: &mut heap,
        };

        // Try moving forward
        if let Some(next_pos) = try_move_forward(map, &current) {
            let new_cost = current.cost + forward_cost;
            let key = (next_pos, current.direction);
            update_path_and_heap(
                &mut context,
                current,
                key,
                new_cost,
                next_pos,
                current.direction,
            );
        }

        // Try rotating clockwise
        let cw_direction = current.direction.rotate_clockwise();
        let new_cost = current.cost + rotate_cw_cost;
        let key = (current.position, cw_direction);

        update_path_and_heap(
            &mut context,
            current,
            key,
            new_cost,
            current.position,
            cw_direction,
        );

        // Try rotating counterclockwise
        let ccw_direction = current.direction.rotate_counter_clockwise();
        let new_cost = current.cost + rotate_ccw_cost;
        let key = (current.position, ccw_direction);

        update_path_and_heap(
            &mut context,
            current,
            key,
            new_cost,
            current.position,
            ccw_direction,
        );
    }

    // Collect all minimal paths
    if !visited_paths.is_empty() {
        // Collect all positions from the paths
        let all_path_positions: HashSet<Position> = visited_paths
            .iter()
            .flat_map(|path| {
                path.iter()
                    .map(|&(pos, _)| pos)
                    .collect::<HashSet<Position>>()
            })
            .collect();

        Some((
            min_cost,
            all_path_positions,
            visited_paths.into_iter().collect(),
        ))
    } else {
        None
    }
}

// Helper functions remain the same as in the previous implementation
fn try_move_forward(map: &[Vec<Location>], current: &State) -> Option<Position> {
    let next_pos = current.position.move_in_direction(current.direction);

    // Check if next position is valid
    if next_pos.x < map[0].len() && next_pos.y < map.len() {
        match map[next_pos.y][next_pos.x] {
            Location::Wall => None,
            _ => Some(next_pos),
        }
    } else {
        None
    }
}

struct PathUpdateContext<'a> {
    dist: &'a mut HashMap<(Position, Direction), u32>,
    paths: &'a mut HashMap<(Position, Direction), Vec<Path>>,
    heap: &'a mut BinaryHeap<Reverse<State>>,
}

fn update_path_and_heap(
    context: &mut PathUpdateContext,
    current: State,
    key: (Position, Direction),
    new_cost: u32,
    next_pos: Position,
    next_direction: Direction,
) {
    match context.dist.get(&key) {
        None => {
            // First time visiting this state
            context.dist.insert(key, new_cost);

            let new_paths: Vec<Path> = context
                .paths
                .get(&(current.position, current.direction))
                .unwrap_or(&vec![])
                .iter()
                .cloned()
                .map(|mut path| {
                    path.push((next_pos, next_direction));
                    path
                })
                .collect();

            context.paths.insert(key, new_paths);

            context.heap.push(Reverse(State {
                cost: new_cost,
                position: next_pos,
                direction: next_direction,
            }));
        }
        Some(&existing_cost) => {
            match new_cost.cmp(&existing_cost) {
                std::cmp::Ordering::Less => {
                    // Found a cheaper path
                    context.dist.insert(key, new_cost);

                    let new_paths: Vec<Path> = context
                        .paths
                        .get(&(current.position, current.direction))
                        .unwrap_or(&vec![])
                        .iter()
                        .cloned()
                        .map(|mut path| {
                            path.push((next_pos, next_direction));
                            path
                        })
                        .collect();

                    context.paths.insert(key, new_paths);

                    context.heap.push(Reverse(State {
                        cost: new_cost,
                        position: next_pos,
                        direction: next_direction,
                    }));
                }
                std::cmp::Ordering::Equal => {
                    // Another path with the same minimal cost
                    let additional_paths: Vec<Path> = context
                        .paths
                        .get(&(current.position, current.direction))
                        .unwrap_or(&vec![])
                        .iter()
                        .cloned()
                        .map(|mut path| {
                            path.push((next_pos, next_direction));
                            path
                        })
                        .collect();

                    context
                        .paths
                        .entry(key)
                        .and_modify(|existing_paths| existing_paths.extend(additional_paths));
                }
                std::cmp::Ordering::Greater => {
                    // Do nothing, as we found a more expensive path
                }
            }
        }
    }
}
