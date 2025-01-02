use itertools::Itertools;
use solver::Solver;
use std::collections::HashMap;

pub struct Day13Solver {
    graph: HashMap<&'static str, HashMap<&'static str, i32>>,
}

impl Day13Solver {
    pub fn new(input: &'static str) -> Self {
        let mut graph = HashMap::new();
        for line in input.lines() {
            let tokens = line.split([' ', '.']).collect::<Vec<_>>();
            // Alice would lose 57 happiness units by sitting next to Bob.
            if let [guest_name, "would", gain_or_lose, hapiness_units, "happiness", "units", "by", "sitting", "next", "to", neighbor_name, ""] =
                &tokens[..]
            {
                let hapiness_units: i32 = hapiness_units.parse().unwrap();
                let hapiness_units = match *gain_or_lose {
                    "gain" => hapiness_units,
                    "lose" => -hapiness_units,
                    _ => panic!("what?"),
                };
                graph
                    .entry(*guest_name)
                    .or_insert(HashMap::new())
                    .insert(*neighbor_name, hapiness_units);
            }
        }
        Day13Solver { graph }
    }

    fn get_max_happiness_seating(&self) -> (i32, Vec<&'static str>, Vec<i32>) {
        let mut result = Vec::new();
        for mut permutation in self.graph.keys().permutations(self.graph.len()) {
            permutation.push(permutation[0]);
            let mut sum_happiness = 0;
            let mut names = Vec::new();
            let mut happiness_values = Vec::new();
            for (&&name_from, &&name_to) in permutation.iter().tuple_windows() {
                let happiness_factor_from =
                    self.graph.get(name_from).unwrap().get(name_to).unwrap();
                let happiness_factor_to = self.graph.get(name_to).unwrap().get(name_from).unwrap();
                sum_happiness += happiness_factor_from;
                sum_happiness += happiness_factor_to;
                names.push(name_from);
                happiness_values.push(*happiness_factor_from);
                happiness_values.push(*happiness_factor_to);
            }
            result.push((sum_happiness, names, happiness_values));
        }
        let max = result
            .iter()
            .max_by(|(a, _, _), (b, _, _)| a.cmp(b))
            .unwrap();
        max.clone()
    }
}

impl Solver for Day13Solver {
    fn solve_part_one(&self) -> String {
        let (sum_hapiness, _names, _happiness_values) = self.get_max_happiness_seating();
        //println!("optimal seating: {sum_hapiness}, {names:?}, {happiness_values:?}");
        sum_hapiness.to_string()
    }

    fn solve_part_two(&self) -> String {
        "".to_string()
    }

    fn day_number(&self) -> usize {
        13
    }

    fn description(&self) -> &'static str {
        "Optimal seating arrangement"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Day13Solver::new(
            r"Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.",
        )
        .solve_part_one();
        assert_eq!(result, "330");
    }
}

// #[cfg(test)]
// mod part2_tests {
//     use super::*;

//     #[test]
//     fn test_1() {
//         let result = Day13Solver::new("abc").solve_part_two();
//         assert_eq!(result, "0");
//     }
// }
