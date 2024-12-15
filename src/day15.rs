//use crossterm::event;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum Location {
    Empty,
    Wall,
    Box,
    Robot,
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn move_in_direction(&self, direction: Direction) -> Position {
        match direction {
            Direction::Up => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Position {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Position {
                x: self.x + 1,
                y: self.y,
            },
        }
    }

    // fn gps(&self) -> u64 {
    //     (self.y as u64) * 100 + (self.x as u64)
    // }
}

pub fn sum_of_all_boxes_gps_coordinates() -> u64 {
    let (mut map, mut robot_position) = get_map();
    let moves = get_moves();
    print_map(&map);
    for direction in moves {
        //println!("next move: {:?}", direction);
        // loop {
        //     if event::poll(std::time::Duration::from_millis(500)).unwrap() {
        //         if let event::Event::Key(_key_event) = event::read().unwrap() {
        //             _ = event::read().unwrap();
        //             break; // Exit the loop on key press
        //         }
        //     }
        // }

        if move_object(&mut map, robot_position, direction) {
            map[robot_position.y][robot_position.x] = Location::Empty;
            robot_position = robot_position.move_in_direction(direction);
        }
        //print_map(&map);
    }

    calculate_sum_of_all_boxes_gps_coordinates(&map)
}

fn get_map() -> (Vec<Vec<Location>>, Position) {
    //     let input = r"##########
    // #..O..O.O#
    // #......O.#
    // #.OO..O.O#
    // #..O@..O.#
    // #O#..O...#
    // #O..O..O.#
    // #.OO.O.OO#
    // #....O...#
    // ##########";
    //     let input = r"########
    // #..O.O.#
    // ##@.O..#
    // #...O..#
    // #.#.O..#
    // #...O..#
    // #......#
    // ########";
    let input = include_str!("../input/day15_map.txt");
    let lines = input.lines();
    let mut robot_position = None;
    let map: Vec<Vec<Location>> = lines
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => Location::Empty,
                    '#' => Location::Wall,
                    'O' => Location::Box,
                    '@' => {
                        if robot_position.is_some() {
                            panic!("multiple robots in map");
                        }
                        robot_position = Some(Position { x, y });
                        Location::Robot
                    }
                    _ => panic!("unexpected character in map"),
                })
                .collect()
        })
        .collect();
    (map, robot_position.expect("no robot in map"))
}

fn get_moves() -> Vec<Direction> {
    //     let input = r"<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
    // vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
    // ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
    // <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
    // ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
    // ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
    // >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
    // <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
    // ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
    // v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
    //let input = "<^^>>>vv<v>>v<<";
    let input = include_str!("../input/day15_moves.txt");

    let moves: Vec<Direction> = input
        .chars()
        .filter_map(|c| match c {
            '\r' | '\n' => None,
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            _ => panic!("unexpected character in moves"),
        })
        .collect();
    moves
}

fn move_object(map: &mut Vec<Vec<Location>>, position: Position, direction: Direction) -> bool {
    let location = map[position.y][position.x];
    let new_position = position.move_in_direction(direction);
    let new_location = map[new_position.y][new_position.x];
    match new_location {
        Location::Empty => {
            map[new_position.y][new_position.x] = location;
            true
        }
        Location::Wall => false,
        Location::Box => {
            let moved = move_object(map, new_position, direction);
            if moved {
                map[new_position.y][new_position.x] = location;
                true
            } else {
                false
            }
        }
        Location::Robot => {
            panic!("robot is already on new location");
        }
    }
}

fn print_map(map: &Vec<Vec<Location>>) {
    for line in map {
        for location in line {
            match location {
                Location::Empty => print!("."),
                Location::Wall => print!("#"),
                Location::Box => print!("O"),
                Location::Robot => print!("@"),
            }
        }
        println!();
    }
}

fn calculate_sum_of_all_boxes_gps_coordinates(map: &[Vec<Location>]) -> u64 {
    let mut sum = 0;
    for (y, line) in map.iter().enumerate() {
        for (x, location) in line.iter().enumerate() {
            if let Location::Box = location {
                sum += (y as u64) * 100 + (x as u64);
            }
        }
    }
    sum
}
