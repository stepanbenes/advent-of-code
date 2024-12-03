pub fn total_distance_between_lists() -> u32 {
    let (mut left_numbers, mut right_numbers) = get_lists();

    left_numbers.sort();
    right_numbers.sort();

    let result: u32 = left_numbers
        .iter()
        .zip(right_numbers)
        .map(|(left, right)| left.abs_diff(right))
        .sum();

    result
}

pub fn similarity_score() -> u32 {
    let (left_numbers, right_numbers) = get_lists();
    let score: u32 = left_numbers
        .iter()
        .map(|x| x * right_numbers.iter().filter(|&y| x == y).count() as u32)
        .sum();
    score
}

fn get_lists() -> (Vec<u32>, Vec<u32>) {
    // read input from file day1.txt
    let input = include_str!("../input/day1.txt");
    // split input by newlines
    let lines = input.lines();
    // convert each line to a u32
    let numbers: Vec<Vec<u32>> = lines
        .map(|line| {
            line.split_whitespace()
                .map(|token| token.parse().unwrap())
                .collect()
        })
        .collect();

    let left_numbers: Vec<u32> = numbers.iter().map(|row| row[0]).collect();
    let right_numbers: Vec<u32> = numbers.iter().map(|row| row[1]).collect();

    (left_numbers, right_numbers)
}
