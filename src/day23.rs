use std::collections::{HashMap, HashSet};

pub fn count_of_computers_with_name_that_starts_with_t() -> usize {
    let connections = get_input();
    let map: HashMap<&str, HashSet<&str>> =
        connections.iter().fold(HashMap::new(), |mut map, &(a, b)| {
            map.entry(a).or_default().insert(b);
            map.entry(b).or_default().insert(a);
            map
        });
    println!("{:?}", map);
    let mut triplets: HashSet<(&str, &str, &str)> = HashSet::new();
    for (a, b) in connections.iter() {
        for c in map.get(b).unwrap() {
            if map.get(c).unwrap().contains(a) {
                let mut triplet = [a, b, c];
                triplet.sort();
                triplets.insert((triplet[0], triplet[1], triplet[2]));
            }
        }
    }
    let mut count = 0;
    for (a, b, c) in triplets {
        if a.starts_with("t") || b.starts_with("t") || c.starts_with("t") {
            println!("{}, {}, {}", a, b, c);
            count += 1;
        }
    }
    count
}

fn get_input() -> Vec<(&'static str, &'static str)> {
    let input = include_str!("../input/day23.txt");
    //     let input = r"kh-tc
    // qp-kh
    // de-cg
    // ka-co
    // yn-aq
    // qp-ub
    // cg-tb
    // vc-aq
    // tb-ka
    // wh-tc
    // yn-cg
    // kh-ub
    // ta-co
    // de-co
    // tc-td
    // tb-wq
    // wh-td
    // ta-ka
    // td-qp
    // aq-cg
    // wq-ub
    // ub-vc
    // de-ta
    // wq-aq
    // wq-vc
    // wh-yn
    // ka-de
    // kh-ta
    // co-tc
    // wh-qp
    // tb-vc
    // td-yn";
    input
        .lines()
        .map(|line| {
            let mut parts = line.split("-");
            let first = parts.next().unwrap();
            let second = parts.next().unwrap();
            (first, second)
        })
        .collect()
}
