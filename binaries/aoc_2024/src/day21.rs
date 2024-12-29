use std::collections::HashMap;

const MAX_DEPTH: usize = 26;

pub fn sum_of_code_complexities() -> u64 {
    let input = get_input();
    let mut keyboard_state = ['A'; MAX_DEPTH + 1];
    let mut complexity_sum = 0;
    let mut memo: HashMap<(String, usize, char), usize> = HashMap::new();
    for code in input.iter() {
        println!("Code: {code}");
        let shortest_path_length =
            get_all_sequences(code, MAX_DEPTH, &mut keyboard_state, &mut memo);
        //println!("{shortest_path}");
        let numeric_part_of_code = code.chars().fold(0, |acc, c| {
            if let Some(value) = c.to_digit(10) {
                acc * 10 + value
            } else {
                acc
            }
        });
        let complexity = shortest_path_length as u64 * numeric_part_of_code as u64;
        complexity_sum += complexity;
    }
    complexity_sum
}

fn get_input() -> Vec<String> {
    let real_input = r"129A
540A
789A
596A
582A";
    //     let test_input = r"029A
    // 980A
    // 179A
    // 456A
    // 379A";
    real_input.lines().map(|line| line.to_owned()).collect()
}

fn get_all_sequences(
    start_path: &String,
    depth: usize,
    keyboard_state: &mut [char; MAX_DEPTH + 1],
    memo: &mut HashMap<(String, usize, char), usize>,
) -> usize {
    if depth == 0 {
        return start_path.len();
    }

    if let Some(result) = memo.get(&(start_path.to_owned(), depth, keyboard_state[depth])) {
        return *result;
    }

    let mut result_length: usize = 0;
    for c in start_path.chars() {
        if depth == MAX_DEPTH {
            println!("{c}");
        }
        let all_paths = find_all_paths(
            keyboard_state[depth],
            c,
            if depth == MAX_DEPTH {
                get_neighbors_for_numerical_keyboard
            } else {
                get_neighbors_for_directional_keyboard
            },
        );
        keyboard_state[depth] = c;

        let mut all_sub_paths: Vec<usize> = Vec::new();
        for path in all_paths {
            let next_level_path = get_next_level_path_from_path(&path);
            let sub_path = get_all_sequences(&next_level_path, depth - 1, keyboard_state, memo);
            all_sub_paths.push(sub_path);
        }
        let shortest_sub_path_length = all_sub_paths.iter().min().unwrap();
        result_length += shortest_sub_path_length;
    }

    memo.insert(
        (start_path.to_owned(), depth, keyboard_state[depth]),
        result_length,
    );

    result_length
}

fn get_next_level_path_from_path(path: &[(char, char)]) -> String {
    let mut result: String = path.iter().map(|(_, direction)| *direction).collect();
    result.push('A');
    result
}

fn find_all_paths(
    start: char,
    end: char,
    neighbors_fn: fn(char) -> Vec<(char, char)>,
) -> Vec<Vec<(char, char)>> {
    let mut paths: Vec<Vec<(char, char)>> = Vec::new();
    let mut current_path: Vec<(char, char)> = vec![];
    let mut visited: std::collections::HashSet<char> = std::collections::HashSet::new();
    visited.insert(start);

    dfs_paths(
        start,
        end,
        &mut current_path,
        &mut visited,
        &mut paths,
        neighbors_fn,
    );

    paths
}

fn dfs_paths(
    current: char,
    end: char,
    current_path: &mut Vec<(char, char)>,
    visited: &mut std::collections::HashSet<char>,
    paths: &mut Vec<Vec<(char, char)>>,
    neighbors_fn: fn(char) -> Vec<(char, char)>,
) {
    // If we've reached the end character, add the current path to our solutions
    if current == end {
        paths.push(current_path.clone());
        return;
    }

    // Get all possible neighbors for the current key
    let neighbors = neighbors_fn(current);

    // Explore each neighbor
    for (next_key, direction) in neighbors {
        // Only explore unvisited keys
        if !visited.contains(&next_key) {
            // Add the next key to our path and mark it as visited
            visited.insert(next_key);
            current_path.push((next_key, direction));

            // Recursively explore from this new position
            dfs_paths(next_key, end, current_path, visited, paths, neighbors_fn);

            // Backtrack: remove the key from our path and visited set
            visited.remove(&next_key);
            current_path.pop();
        }
    }
}

fn get_neighbors_for_numerical_keyboard(key: char) -> Vec<(char, char)> {
    match key {
        '1' => vec![('2', '>'), ('4', '^')],
        '2' => vec![('1', '<'), ('3', '>'), ('5', '^'), ('0', 'v')],
        '3' => vec![('2', '<'), ('6', '^'), ('A', 'v')],
        '4' => vec![('1', 'v'), ('5', '>'), ('7', '^')],
        '5' => vec![('2', 'v'), ('4', '<'), ('6', '>'), ('8', '^')],
        '6' => vec![('3', 'v'), ('5', '<'), ('9', '^')],
        '7' => vec![('4', 'v'), ('8', '>')],
        '8' => vec![('5', 'v'), ('7', '<'), ('9', '>')],
        '9' => vec![('6', 'v'), ('8', '<')],
        '0' => vec![('2', '^'), ('A', '>')],
        'A' => vec![('0', '<'), ('3', '^')],
        _ => panic!("Invalid key: {key}"),
    }
}

fn get_neighbors_for_directional_keyboard(key: char) -> Vec<(char, char)> {
    match key {
        '^' => vec![('v', 'v'), ('A', '>')],
        'v' => vec![('^', '^'), ('<', '<'), ('>', '>')],
        '<' => vec![('v', '>')],
        '>' => vec![('v', '<'), ('A', '^')],
        'A' => vec![('^', '<'), ('>', 'v')],
        _ => panic!("Invalid key: {}", key),
    }
}
