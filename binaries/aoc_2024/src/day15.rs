use crossterm::event;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum BoxPart {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum Location {
    Empty,
    Wall,
    Box,
    DoubleBox(BoxPart),
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

pub fn sum_of_all_boxes_gps_coordinates(use_double_map: bool) -> u64 {
    let mut map = get_map();
    let _moves = get_moves();

    if use_double_map {
        map = double_map(&map);
    }

    print_map(&map);

    let mut robot_position = find_robot_position(&map);

    // for direction in moves {
    //     if is_allowed_to_move(&map, robot_position, direction) {
    //         move_object(&mut map, robot_position, direction);
    //         map[robot_position.y][robot_position.x] = Location::Empty;
    //         robot_position = robot_position.move_in_direction(direction);
    //     }
    //     //print_map(&map);
    // }

    // print_map(&map);

    loop {
        let direction = loop {
            if event::poll(std::time::Duration::from_millis(500)).unwrap() {
                if let event::Event::Key(_key_event) = event::read().unwrap() {
                    let key = event::read().unwrap();
                    if let event::Event::Key(key) = key {
                        match key.code {
                            event::KeyCode::Up => break Direction::Up,
                            event::KeyCode::Down => break Direction::Down,
                            event::KeyCode::Left => break Direction::Left,
                            event::KeyCode::Right => break Direction::Right,
                            _ => (),
                        }
                    }
                }
            }
        };

        println!("direction: {:?}", direction);
        if is_allowed_to_move(&map, robot_position, direction) {
            //println!("next move: {:?}", direction);
            move_object(&mut map, robot_position, direction);
            map[robot_position.y][robot_position.x] = Location::Empty;
            robot_position = robot_position.move_in_direction(direction);
        }
        print_map(&map);
    }

    //calculate_sum_of_all_boxes_gps_coordinates(&map)
}

fn find_robot_position(map: &[Vec<Location>]) -> Position {
    for (y, line) in map.iter().enumerate() {
        for (x, location) in line.iter().enumerate() {
            if let Location::Robot = location {
                return Position { x, y };
            }
        }
    }
    panic!("robot not found in map");
}

fn get_map() -> Vec<Vec<Location>> {
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
    //     let input = r"#######
    // #...#.#
    // #.....#
    // #..OO@#
    // #..O..#
    // #.....#
    // #######";
    let input = include_str!("../input/day15_map.txt");
    let lines = input.lines();
    let map: Vec<Vec<Location>> = lines
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Location::Empty,
                    '#' => Location::Wall,
                    'O' => Location::Box,
                    '@' => Location::Robot,
                    _ => panic!("unexpected character in map"),
                })
                .collect()
        })
        .collect();
    map
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
    //let input = "<vv<<^^<<^^";

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

fn move_object(map: &mut Vec<Vec<Location>>, position: Position, direction: Direction) {
    let location = map[position.y][position.x];
    let new_position = position.move_in_direction(direction);
    let new_location = map[new_position.y][new_position.x];
    match new_location {
        Location::Empty => {
            map[new_position.y][new_position.x] = location;
        }
        Location::Wall => panic!("wall in the way at {new_position:?}"),
        Location::Box => {
            move_object(map, new_position, direction);
            map[new_position.y][new_position.x] = location;
        }
        Location::DoubleBox(box_part) => match direction {
            Direction::Left | Direction::Right => {
                move_object(map, new_position, direction);
                map[new_position.y][new_position.x] = location;
            }
            Direction::Up | Direction::Down => {
                let other_part_new_position = if box_part == BoxPart::Left {
                    new_position.move_in_direction(Direction::Right)
                } else {
                    new_position.move_in_direction(Direction::Left)
                };
                move_object(map, new_position, direction);
                move_object(map, other_part_new_position, direction);
                map[new_position.y][new_position.x] = location;
                map[other_part_new_position.y][other_part_new_position.x] = Location::Empty;
            }
        },
        Location::Robot => panic!("robot is already on new location"),
    }
}

fn is_allowed_to_move(map: &[Vec<Location>], position: Position, direction: Direction) -> bool {
    let new_position = position.move_in_direction(direction);
    let new_location = map[new_position.y][new_position.x];

    match new_location {
        Location::Empty => true,
        Location::Box => is_allowed_to_move(map, new_position, direction),
        Location::Wall => false,
        Location::DoubleBox(box_part) => match direction {
            Direction::Left | Direction::Right => is_allowed_to_move(map, new_position, direction),
            Direction::Up | Direction::Down => {
                let other_part_new_position = if box_part == BoxPart::Left {
                    new_position.move_in_direction(Direction::Right)
                } else {
                    new_position.move_in_direction(Direction::Left)
                };

                let allowed_1 = is_allowed_to_move(map, new_position, direction);
                let allowed_2 = is_allowed_to_move(map, other_part_new_position, direction);
                allowed_1 && allowed_2
            }
        },
        Location::Robot => panic!("robot is already on new location"),
    }
}

fn print_map(map: &Vec<Vec<Location>>) {
    for line in map {
        for location in line {
            match location {
                Location::Empty => print!("."),
                Location::Wall => print!("#"),
                Location::Box => print!("O"),
                Location::DoubleBox(BoxPart::Left) => print!("["),
                Location::DoubleBox(BoxPart::Right) => print!("]"),
                Location::Robot => print!("@"),
            }
        }
        println!();
    }
}

fn _calculate_sum_of_all_boxes_gps_coordinates(map: &[Vec<Location>]) -> u64 {
    let mut sum = 0;
    for (y, line) in map.iter().enumerate() {
        for (x, location) in line.iter().enumerate() {
            if let Location::Box | Location::DoubleBox(BoxPart::Left) = location {
                sum += (y as u64) * 100 + (x as u64);
            }
        }
    }
    sum
}

fn double_map(map: &[Vec<Location>]) -> Vec<Vec<Location>> {
    map.iter()
        .map(|line| {
            line.iter()
                .flat_map(|&location| match location {
                    Location::Robot => [Location::Robot, Location::Empty],
                    Location::Box => [
                        Location::DoubleBox(BoxPart::Left),
                        Location::DoubleBox(BoxPart::Right),
                    ],
                    Location::DoubleBox(_) => panic!("double box in original map"),
                    _ => [location, location],
                })
                .collect()
        })
        .collect()
}
