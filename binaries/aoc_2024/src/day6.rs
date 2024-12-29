pub fn count_of_distinct_positions_in_map() -> u32 {
    let (mut map, position) = get_map_with_position();
    _ = walk_through_map(&mut map, &position);
    get_count_of_visited_locations(&map)
}

pub fn count_of_different_positions_for_obstructions() -> u32 {
    let (mut map, position) = get_map_with_position();
    let mut count_of_obstacles = 0;
    for row_index in 0..map.len() {
        for column_index in 0..map[row_index].len() {
            if map[row_index][column_index] == Location::Empty {
                map[row_index][column_index] = Location::Obstacle;
                let result = walk_through_map(&mut map, &position);
                if let WalkResult::LoopDetected = result {
                    count_of_obstacles += 1;
                    //println!("Loop detected at row: {}, column: {}", row_index, column_index);
                }
                map[row_index][column_index] = Location::Empty;
                remove_all_visits(&mut map);
            }
        }
    }
    count_of_obstacles
}

#[derive(Debug, Clone)]
struct Position {
    row_index: usize,
    column_index: usize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Location {
    Empty,
    Obstacle,
    Visited(Vec<Direction>),
}

#[derive(Debug)]
enum WalkResult {
    Completed,
    LoopDetected,
}

fn get_map_with_position() -> (Vec<Vec<Location>>, Position) {
    let input = include_str!("../input/day6.txt");
    //     let input = r"....#.....
    // .........#
    // ..........
    // ..#.......
    // .......#..
    // ..........
    // .#..^.....
    // ........#.
    // #.........
    // ......#...";
    let mut position: Option<Position> = None;
    let map: Vec<Vec<Location>> = input
        .lines()
        .enumerate()
        .map(|(row_index, line)| {
            line.chars()
                .enumerate()
                .map(|(column_index, symbol)| {
                    if symbol == '^' {
                        position = Some(Position {
                            row_index,
                            column_index,
                        })
                    };
                    if symbol == '#' {
                        Location::Obstacle
                    } else {
                        Location::Empty
                    }
                })
                .collect()
        })
        .collect();
    (map, position.unwrap())
}

fn walk_through_map(map: &mut [Vec<Location>], position: &Position) -> WalkResult {
    let mut current_position = position.clone();
    let mut direction = Direction::Up;

    map[current_position.row_index][current_position.column_index] =
        Location::Visited(vec![direction]); // initial position is also visited

    //print_map_with_position(map, &current_position, &direction);

    while current_position.row_index > 0
        && current_position.column_index > 0
        && current_position.row_index < map.len() - 1
        && current_position.column_index < map[current_position.row_index].len() - 1
    {
        let next_position = match direction {
            Direction::Up => Position {
                row_index: current_position.row_index - 1,
                column_index: current_position.column_index,
            },
            Direction::Right => Position {
                row_index: current_position.row_index,
                column_index: current_position.column_index + 1,
            },
            Direction::Down => Position {
                row_index: current_position.row_index + 1,
                column_index: current_position.column_index,
            },
            Direction::Left => Position {
                row_index: current_position.row_index,
                column_index: current_position.column_index - 1,
            },
        };

        if map[next_position.row_index][next_position.column_index] == Location::Obstacle {
            // obstacle in front of me
            //print_map_with_position(map, &current_position, &direction);

            direction = direction.turn_right();
        } else {
            current_position = next_position; // move forward
            let loop_detected = visit_location(map, &current_position, &direction);
            if loop_detected {
                return WalkResult::LoopDetected;
            }
        }
    }
    //print_map_with_position(map, &current_position, &direction);

    WalkResult::Completed
}

fn visit_location(map: &mut [Vec<Location>], position: &Position, direction: &Direction) -> bool {
    match map[position.row_index][position.column_index] {
        Location::Empty => {
            map[position.row_index][position.column_index] = Location::Visited(vec![*direction]);
            false
        }
        Location::Obstacle => {
            panic!("Cannot visit obstacle");
        }
        Location::Visited(ref mut directions) => {
            if !directions.contains(direction) {
                directions.push(*direction);
                false
            } else {
                true // loop detected
            }
        }
    }
}

fn remove_all_visits(map: &mut [Vec<Location>]) {
    for row in map.iter_mut() {
        for location in row.iter_mut() {
            if let Location::Visited(_) = location {
                *location = Location::Empty;
            }
        }
    }
}

fn get_count_of_visited_locations(map: &[Vec<Location>]) -> u32 {
    map.iter()
        .map(|row| {
            row.iter()
                .filter(|location| matches!(location, Location::Visited(_)))
                .count() as u32
        })
        .sum()
}

fn _print_map_with_position(map: &[Vec<Location>], position: &Position, direction: &Direction) {
    let map_with_position: Vec<Vec<char>> = map
        .iter()
        .enumerate()
        .map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .map(|(column_index, location)| {
                    if row_index == position.row_index && column_index == position.column_index {
                        match direction {
                            Direction::Up => '^',
                            Direction::Right => '>',
                            Direction::Down => 'v',
                            Direction::Left => '<',
                        }
                    } else {
                        match location {
                            Location::Empty => '.',
                            Location::Obstacle => '#',
                            Location::Visited(_) => 'X',
                        }
                    }
                })
                .collect()
        })
        .collect();
    for row in map_with_position {
        println!("{}", row.iter().collect::<String>());
    }
    println!();
}
