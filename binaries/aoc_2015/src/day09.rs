use itertools::Itertools;
use std::collections::HashMap;

use solver::Solver;

type City = &'static str;

pub struct Day09Solver {
    graph: HashMap<City, HashMap<City, u32>>,
}

impl Day09Solver {
    pub fn new(input: &'static str) -> Self {
        let mut graph = HashMap::new();
        for line in input.lines() {
            let tokens = line.split_whitespace().collect::<Vec<_>>();
            if let [city_from, "to", city_to, "=", distance] = &tokens[..] {
                let distance = distance.parse().unwrap();
                graph
                    .entry(*city_from)
                    .or_insert(HashMap::new())
                    .insert(*city_to, distance);
                graph
                    .entry(*city_to)
                    .or_insert(HashMap::new())
                    .insert(*city_from, distance);
            }
        }
        println!("{:?}", graph);
        Day09Solver { graph }
    }

    fn get_all_distances(&self) -> Vec<u32> {
        let mut distances = Vec::new();
        for permutation in self.graph.keys().permutations(self.graph.len()) {
            let mut total_distance = 0;
            for (&&city_from, &&city_to) in permutation.iter().tuple_windows() {
                let distance = self.graph.get(city_from).unwrap().get(city_to).unwrap();
                total_distance += distance;
            }
            distances.push(total_distance);
        }
        distances
    }
}

impl Solver for Day09Solver {
    fn solve_part_one(&self) -> String {
        let distances = self.get_all_distances();
        let shortest_distance = *distances.iter().min().unwrap();
        shortest_distance.to_string()
    }

    fn solve_part_two(&self) -> String {
        let distances = self.get_all_distances();
        let longest_distance = *distances.iter().max().unwrap();
        longest_distance.to_string()
    }

    fn day_number(&self) -> usize {
        9
    }

    fn description(&self) -> &'static str {
        "Traveling salesman problem"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Day09Solver::new(
            r"London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141",
        )
        .solve_part_one();
        assert_eq!(result, "605");
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Day09Solver::new(
            r"London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141",
        )
        .solve_part_two();
        assert_eq!(result, "982");
    }
}
