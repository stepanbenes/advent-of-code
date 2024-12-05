use std::collections::HashMap;

pub fn sum_of_middle_page_numbers() -> u32 {
    let ordering = get_ordering_input();
    let updates = get_updates_input();
    let result = updates
        .iter()
        .filter(|x| check_update_is_in_correct_order(x, &ordering))
        .map(|x| get_middle_page_number(x))
        .sum();
    result
}

fn get_ordering_input() -> HashMap<u32, Vec<u32>> {
    //     let ordering_str = r"47|53
    // 97|13
    // 97|61
    // 97|47
    // 75|29
    // 61|13
    // 75|53
    // 29|13
    // 97|29
    // 53|29
    // 61|53
    // 97|53
    // 61|29
    // 47|13
    // 75|47
    // 97|75
    // 47|61
    // 75|61
    // 47|29
    // 75|13
    // 53|13";
    let ordering_str = include_str!("../input/day5_ordering.txt");
    let mut ordering = HashMap::new();
    for line in ordering_str.lines() {
        let mut parts = line.split("|");
        let page_before: u32 = parts.next().unwrap().parse().unwrap();
        let page_after: u32 = parts.next().unwrap().parse().unwrap();
        ordering
            .entry(page_before)
            .or_insert(Vec::new())
            .push(page_after);
    }

    ordering
}

fn get_updates_input() -> Vec<Vec<u32>> {
    //     let updates_str = r"75,47,61,53,29
    // 97,61,53,29,13
    // 75,29,13
    // 75,97,47,61,53
    // 61,13,29
    // 97,13,75,29,47";
    let updates_str = include_str!("../input/day5_updates.txt");
    let updates = updates_str
        .lines()
        .map(|x| x.split(",").map(|y| y.parse().unwrap()).collect())
        .collect();
    updates
}

fn check_update_is_in_correct_order(update: &[u32], ordering: &HashMap<u32, Vec<u32>>) -> bool {
    for (page_index, page) in update.iter().enumerate() {
        if let Some(pages_after) = ordering.get(page) {
            for page_before in update.iter().take(page_index) {
                if pages_after.contains(page_before) {
                    return false;
                }
            }
        }
    }
    true
}

fn get_middle_page_number(update: &[u32]) -> u32 {
    update[update.len() / 2]
}
