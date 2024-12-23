const MAX_DEPTH: usize = 3;

pub fn sum_of_code_complexities() -> u32 {
    let input = get_input();
    let mut keyboard_state = ['A'; MAX_DEPTH + 1];
    let mut complexity_sum = 0;
    for code in input.iter() {
        println!("Code: {:?}", code);
        let shortest_path = get_all_sequences(code, MAX_DEPTH, &mut keyboard_state);
        print_path(&shortest_path);
        let numeric_part_of_code = code.iter().fold(0, |acc, c| {
            if let Some(value) = c.to_digit(10) {
                acc * 10 + value
            } else {
                acc
            }
        });
        let complexity = shortest_path.len() as u32 * numeric_part_of_code;
        complexity_sum += complexity;
    }
    complexity_sum
}

fn get_input() -> Vec<Vec<char>> {
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
    real_input
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn get_all_sequences(
    start_path: &Vec<char>,
    depth: usize,
    keyboard_state: &mut [char; MAX_DEPTH + 1],
) -> Vec<char> {
    if depth == 0 {
        return start_path.clone();
    }

    let mut result: Vec<char> = Vec::new();
    for c in start_path {
        let all_paths = find_all_paths(
            keyboard_state[depth],
            *c,
            if depth == MAX_DEPTH {
                get_neighbors_for_numerical_keyboard
            } else {
                get_neighbors_for_directional_keyboard
            },
        );
        keyboard_state[depth] = *c;

        let mut all_sub_paths: Vec<Vec<char>> = Vec::new();
        for path in all_paths {
            let next_level_path = get_next_level_path_from_path(&path);
            let sub_path = get_all_sequences(&next_level_path, depth - 1, keyboard_state);
            all_sub_paths.push(sub_path);
        }
        let shortest_sub_path = all_sub_paths
            .iter()
            .min_by(|a, b| a.len().cmp(&b.len()))
            .unwrap();
        result.extend(shortest_sub_path);
    }
    result
}

fn get_next_level_path_from_path(path: &[(char, char)]) -> Vec<char> {
    let mut result: Vec<_> = path.iter().map(|(_, direction)| *direction).collect();
    result.push('A');
    result
}

fn print_path(path: &[char]) {
    for c in path {
        print!("{}", c);
    }
    println!()
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
