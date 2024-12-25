use itertools::Itertools;

pub fn count_of_unique_lock_pin_pairs_that_fit_together_without_overlapping() -> usize {
    let (locks, keys) = get_input();
    println!("locks: {:?}", locks);
    println!("keys: {:?}", keys);
    let mut count = 0;
    for (lock, key) in locks.iter().cartesian_product(keys.iter()) {
        if lock
            .iter()
            .zip(key.iter())
            .all(|(&lock_height, &key_height)| lock_height + key_height <= 5)
        {
            count += 1;
        }
    }
    count
}

fn get_input() -> (Vec<[u8; 5]>, Vec<[u8; 5]>) {
    let input = include_str!("../input/day25.txt");
    //     let input = r"#####
    // .####
    // .####
    // .####
    // .#.#.
    // .#...
    // .....

    // #####
    // ##.##
    // .#.##
    // ...##
    // ...#.
    // ...#.
    // .....

    // .....
    // #....
    // #....
    // #...#
    // #.#.#
    // #.###
    // #####

    // .....
    // .....
    // #.#..
    // ###..
    // ###.#
    // ###.#
    // #####

    // .....
    // .....
    // .....
    // #....
    // #.#..
    // #.#.#
    // #####";

    let mut lock_or_key = [[false; 5]; 7];
    let mut lines = input.lines();

    let mut locks: Vec<[u8; 5]> = Vec::new();
    let mut keys: Vec<[u8; 5]> = Vec::new();

    while let Some(line) = lines.next() {
        let mut schema = Vec::new();
        schema.push(line);
        for _ in 0..6 {
            schema.push(lines.next().unwrap());
        }
        let kind = parse_lock_or_key(&schema, &mut lock_or_key);
        let pin_heights = convert_to_pin_heights(&lock_or_key);
        match kind {
            LockOrKey::Lock => locks.push(pin_heights),
            LockOrKey::Key => keys.push(pin_heights),
        }
        _ = lines.next(); // Skip empty line
    }

    (locks, keys)
}

enum LockOrKey {
    Lock,
    Key,
}

fn parse_lock_or_key(schema: &[&str], lock_or_key: &mut [[bool; 5]; 7]) -> LockOrKey {
    for (i, line) in schema.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '#' => lock_or_key[i][j] = true,
                '.' => lock_or_key[i][j] = false,
                _ => panic!("Invalid character: {}", c),
            }
        }
    }
    match (lock_or_key[0], lock_or_key[6]) {
        ([true, true, true, true, true], [false, false, false, false, false]) => LockOrKey::Lock,
        ([false, false, false, false, false], [true, true, true, true, true]) => LockOrKey::Key,
        _ => panic!("Invalid lock or key"),
    }
}

fn convert_to_pin_heights(lock_or_key: &[[bool; 5]; 7]) -> [u8; 5] {
    let mut pin_heights = [0; 5];
    for row in lock_or_key.iter() {
        for (col, pin_height) in pin_heights.iter_mut().enumerate() {
            if row[col] {
                *pin_height += 1;
            }
        }
    }
    pin_heights.iter_mut().for_each(|height| *height -= 1);
    pin_heights
}
