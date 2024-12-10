use std::collections::HashSet;

pub fn get_sum_of_all_trailhead_scores() -> (u32, u32) {
    let map = get_topographic_map();
    let trailheads = get_all_trailheads(&map);
    let mut total_score = 0;
    let mut total_rating = 0;
    for trailhead in &trailheads {
        println!("trailhead {:?}:", trailhead);
        let mut summits = HashSet::new();
        let rating = walk_trailhead(trailhead, &map, &mut summits);
        println!("summits:");
        for summit in &summits {
            println!("{:?}", summit);
        }
        total_score += summits.len() as u32;
        println!("rating: {}", rating);
        total_rating += rating;
    }
    (total_score, total_rating)
}

fn get_topographic_map() -> Vec<Vec<u8>> {
    let input = include_str!("../input/day10.txt");
    //     let input = r"0123
    // 1234
    // 8765
    // 9876";
    // let input = r"...0...
    // ...1...
    // ...2...
    // 6543456
    // 7.....7
    // 8.....8
    // 9.....9";
    //     let input = r"..90..9
    // ...1.98
    // ...2..7
    // 6543456
    // 765.987
    // 876....
    // 987....";
    // let input = r"10..9..
    // 2...8..
    // 3...7..
    // 4567654
    // ...8..3
    // ...9..2
    // .....01";
    // let input = r"89010123
    // 78121874
    // 87430965
    // 96549874
    // 45678903
    // 32019012
    // 01329801
    // 10456732";
    //     let input = r".....0.
    // ..4321.
    // ..5..2.
    // ..6543.
    // ..7..4.
    // ..8765.
    // ..9....";
    // let input = r"012345
    // 123456
    // 234567
    // 345678
    // 4.6789
    // 56789.";
    // let input = r"89010123
    // 78121874
    // 87430965
    // 96549874
    // 45678903
    // 32019012
    // 01329801
    // 10456732";
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap_or(u8::MAX as u32) as u8)
                .collect()
        })
        .collect()
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Position {
    row: usize,
    column: usize,
}

fn get_all_trailheads(map: &[Vec<u8>]) -> Vec<Position> {
    let mut trailheads = Vec::new();
    for (row_index, row) in map.iter().enumerate() {
        for (column_index, &value) in row.iter().enumerate() {
            if value == 0 {
                trailheads.push(Position {
                    row: row_index,
                    column: column_index,
                });
            }
        }
    }
    trailheads
}

fn walk_trailhead(
    current_position: &Position,
    map: &[Vec<u8>],
    summits: &mut HashSet<Position>,
) -> u32 {
    let current_height = map[current_position.row][current_position.column];
    if current_height == 9 {
        summits.insert(current_position.clone());
        return 1;
    }
    let mut rating_sum = 0;
    for neighbor in get_neighbors(current_position, map[0].len(), map.len()) {
        let neighbor_height = map[neighbor.row][neighbor.column];
        if neighbor_height == current_height + 1 {
            let rating = walk_trailhead(&neighbor, map, summits);
            rating_sum += rating;
        }
    }
    rating_sum
}

fn get_neighbors(position: &Position, map_width: usize, map_height: usize) -> Vec<Position> {
    let mut neighbors = Vec::new();
    if position.row > 0 {
        neighbors.push(Position {
            row: position.row - 1,
            column: position.column,
        });
    }
    if position.row < map_height - 1 {
        neighbors.push(Position {
            row: position.row + 1,
            column: position.column,
        });
    }
    if position.column > 0 {
        neighbors.push(Position {
            row: position.row,
            column: position.column - 1,
        });
    }
    if position.column < map_width - 1 {
        neighbors.push(Position {
            row: position.row,
            column: position.column + 1,
        });
    }
    neighbors
}
