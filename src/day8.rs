use std::collections::{HashMap, HashSet};

pub fn count_of_antinodes() -> usize {
    let input = get_input();
    let (map_width, map_height) = (input.len(), input[0].len());
    let antenna_positions = get_antenna_positions(&input);
    let mut all_antinode_positions = HashSet::new();
    for positions in antenna_positions.values() {
        let antinode_positions = get_all_antinodes_positions(positions, map_width, map_height);
        for antinode_position in antinode_positions {
            all_antinode_positions.insert(antinode_position);
        }
    }

    // print map with antinodes
    for (row, line) in input.iter().enumerate() {
        for (column, &character) in line.iter().enumerate() {
            let position = Position { row, column };
            if all_antinode_positions.contains(&position) {
                print!("#");
            } else {
                print!("{}", character);
            }
        }
        println!();
    }

    all_antinode_positions.len()
}

fn get_input() -> Vec<Vec<char>> {
    let input = include_str!("../input/day8.txt");
    //     let input = r"............
    // ........0...
    // .....0......
    // .......0....
    // ....0.......
    // ......A.....
    // ............
    // ............
    // ........A...
    // .........A..
    // ............
    // ............";
    input.lines().map(|line| line.chars().collect()).collect()
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Position {
    row: usize,
    column: usize,
}

fn get_antenna_positions(input: &[Vec<char>]) -> HashMap<char, Vec<Position>> {
    let mut antenna_positions = HashMap::new();
    for (row, line) in input.iter().enumerate() {
        for (column, &character) in line.iter().enumerate() {
            if character.is_alphanumeric() {
                let positions = antenna_positions.entry(character).or_insert(vec![]);
                positions.push(Position { row, column });
            }
        }
    }
    antenna_positions
}

fn find_all_antenna_pairs(antenna_positions: &[Position]) -> Vec<(&Position, &Position)> {
    let mut antenna_pairs = vec![];
    for (index, position) in antenna_positions.iter().enumerate() {
        for other_position in &antenna_positions[index + 1..] {
            antenna_pairs.push((position, other_position));
        }
    }
    antenna_pairs
}

fn get_all_antinodes_positions(
    antenna_positions: &[Position],
    map_width: usize,
    map_height: usize,
) -> Vec<Position> {
    let antenna_pairs = find_all_antenna_pairs(antenna_positions);
    let mut antinode_positions = vec![];
    for (position, other_position) in antenna_pairs {
        let row_diff: isize = position.row as isize - other_position.row as isize;
        let column_diff: isize = position.column as isize - other_position.column as isize;
        let first_antinode_position_row = position.row as isize + row_diff;
        let first_antinode_position_column = position.column as isize + column_diff;
        let second_antinode_position_row = other_position.row as isize - row_diff;
        let second_antinode_position_column = other_position.column as isize - column_diff;
        if first_antinode_position_row >= 0
            && first_antinode_position_row < map_height as isize
            && first_antinode_position_column >= 0
            && first_antinode_position_column < map_width as isize
        {
            antinode_positions.push(Position {
                row: first_antinode_position_row as usize,
                column: first_antinode_position_column as usize,
            });
        }
        if second_antinode_position_row >= 0
            && second_antinode_position_row < map_height as isize
            && second_antinode_position_column >= 0
            && second_antinode_position_column < map_width as isize
        {
            antinode_positions.push(Position {
                row: second_antinode_position_row as usize,
                column: second_antinode_position_column as usize,
            });
        }
    }
    antinode_positions
}
