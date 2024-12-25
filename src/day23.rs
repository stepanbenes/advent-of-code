use std::collections::{HashMap, HashSet};

pub fn count_of_computers_with_name_that_starts_with_t() -> usize {
    let connections = get_input();
    let adjacency_map: HashMap<&str, HashSet<&str>> =
        connections.iter().fold(HashMap::new(), |mut map, &(a, b)| {
            map.entry(a).or_default().insert(b);
            map.entry(b).or_default().insert(a);
            map
        });
    println!("{:?}", adjacency_map);
    let mut triplets: HashSet<(&str, &str, &str)> = HashSet::new();
    for (a, b) in connections.iter() {
        for c in adjacency_map.get(b).unwrap() {
            if adjacency_map.get(c).unwrap().contains(a) {
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

pub fn password_to_lan_party() -> String {
    let mut largest_set = get_largest_set_of_computers_that_are_all_connected_to_each_other();
    largest_set.sort();
    largest_set.join(",")
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

fn get_largest_set_of_computers_that_are_all_connected_to_each_other() -> Vec<&'static str> {
    let connections = get_input();
    let result = largest_clique(connections);
    result
}

// // Helper function to check if a set of nodes forms a clique
// fn is_clique(graph: &HashMap<&str, HashSet<&str>>, nodes: &Vec<&str>) -> bool {
//     for &node in nodes {
//         for &other_node in nodes {
//             if node != other_node && !graph[&node].contains(other_node) {
//                 return false;
//             }
//         }
//     }
//     true
// }

// // Function to find the largest clique
// fn largest_clique<'a>(edges: Vec<(&'a str, &'a str)>) -> Vec<&'a str> {
//     // Build the adjacency list as a HashMap<&str, HashSet<&str>>
//     let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();
//     for (u, v) in edges {
//         graph.entry(u).or_default().insert(v);
//         graph.entry(v).or_default().insert(u);
//     }

//     let nodes: Vec<&str> = graph.keys().copied().collect();
//     let mut largest_clique = Vec::new();

//     // Iterate through all subsets of nodes
//     let total_subsets = 1i64 << nodes.len(); // 2^N subsets
//     for subset_mask in 0..total_subsets {
//         let mut subset = Vec::new();
//         for i in 0..nodes.len() {
//             if (subset_mask & (1 << i)) != 0 {
//                 subset.push(nodes[i]);
//             }
//         }

//         // Check if this subset forms a clique
//         if is_clique(&graph, &subset) && subset.len() > largest_clique.len() {
//             largest_clique = subset;
//         }
//     }

//     largest_clique
// }

// Recursive function for the Bron-Kerbosch algorithm
fn bron_kerbosch<'a>(
    r: &mut HashSet<&'a str>, // Current clique
    p: &mut HashSet<&'a str>, // Candidate nodes
    x: &mut HashSet<&'a str>, // Nodes to exclude
    graph: &HashMap<&'a str, HashSet<&'a str>>,
    largest_clique: &mut HashSet<&'a str>, // Keep track of the largest clique found
) {
    if p.is_empty() && x.is_empty() {
        // Found a maximal clique
        if r.len() > largest_clique.len() {
            *largest_clique = r.clone();
        }
        return;
    }

    let p_clone = p.clone(); // Clone candidates to iterate over while modifying `p`
    for &v in &p_clone {
        // Add node `v` to the current clique
        r.insert(v);

        // Compute new sets of candidates and exclusions
        let empty = HashSet::new();
        let neighbors = graph.get(v).unwrap_or(&empty);
        let mut new_p: HashSet<&str> = p.intersection(neighbors).copied().collect();
        let mut new_x: HashSet<&str> = x.intersection(neighbors).copied().collect();

        // Recursive call
        bron_kerbosch(r, &mut new_p, &mut new_x, graph, largest_clique);

        // Backtrack: remove `v` from the current clique
        r.remove(v);
        p.remove(v);
        x.insert(v);
    }
}

// Function to find the largest clique in the graph
fn largest_clique<'a>(edges: Vec<(&'a str, &'a str)>) -> Vec<&'a str> {
    // Build the adjacency list as a HashMap<&str, HashSet<&str>>
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (u, v) in edges {
        graph.entry(u).or_default().insert(v);
        graph.entry(v).or_default().insert(u);
    }

    let mut r = HashSet::new();
    let mut p: HashSet<&str> = graph.keys().copied().collect();
    let mut x = HashSet::new();
    let mut largest_clique = HashSet::new();

    // Start Bron-Kerbosch
    bron_kerbosch(&mut r, &mut p, &mut x, &graph, &mut largest_clique);

    largest_clique.into_iter().collect()
}
