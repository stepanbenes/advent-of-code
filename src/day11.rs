use std::collections::HashMap;

pub fn count_of_stones<const STEP_COUNT: usize>() -> u64 {
    let stones = get_input();
    let mut table: HashMap<u64, [u64; STEP_COUNT]> = HashMap::new();
    //println!("{:?}", stones);
    let mut total_count = 0;
    for stone in stones {
        let count = blink(stone, 0, &mut table);
        total_count += count;
    }
    total_count
}

fn get_input() -> Vec<u64> {
    //let input = "0 1 10 99 999";
    //let input = "125 17";
    let input = "112 1110 163902 0 7656027 83039 9 74";
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn blink<const STEP_COUNT: usize>(
    stone: u64,
    depth: usize,
    table: &mut HashMap<u64, [u64; STEP_COUNT]>,
) -> u64 {
    //println!("{:?} {:?}", stone, step_count);
    // stop the recursion
    if depth == STEP_COUNT {
        return 1;
    }
    // If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
    if stone == 0 {
        let new_stone_value = 1;
        return table_lookup(new_stone_value, depth, table);
    }
    // If the stone is engraved with a number that has an even number of digits, it is replaced by two stones. The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
    if stone.to_string().len() % 2 == 0 {
        let stone_str = stone.to_string();
        let half = stone_str.len() / 2;
        let left = stone_str[..half].parse().unwrap();
        let right = stone_str[half..].parse().unwrap();
        return table_lookup(left, depth, table) + table_lookup(right, depth, table);
    }
    // If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.
    {
        let new_stone_value = stone * 2024;
        table_lookup(new_stone_value, depth, table)
    }
}

fn table_lookup<const STEP_COUNT: usize>(
    new_stone_value: u64,
    depth: usize,
    table: &mut HashMap<u64, [u64; STEP_COUNT]>,
) -> u64 {
    let mut value;
    if let Some(table_row) = table.get_mut(&new_stone_value) {
        value = table_row[depth];
    } else {
        table.insert(new_stone_value, [0; STEP_COUNT]);
        value = 0;
    }
    if value == 0 {
        value = blink(new_stone_value, depth + 1, table);
        table.get_mut(&new_stone_value).unwrap()[depth] = value;
    }
    value
}
