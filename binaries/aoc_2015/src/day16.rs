use std::collections::HashMap;

use solver::SolverBase;

#[derive(Debug)]
struct Sue {
    number: usize,
    things: HashMap<&'static str, u8>,
}

pub struct Solver {
    sues: Vec<Sue>,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        let mut sues = Vec::new();
        for line in input.lines() {
            // Sue 1: goldfish: 6, trees: 9, akitas: 0
            let tokens: Vec<_> = line
                .split([' ', ':', ','])
                .filter(|x| !x.is_empty())
                .collect();
            assert!(tokens[0] == "Sue");
            let number: usize = tokens[1].parse().unwrap();
            let mut things = HashMap::new();
            for i in (2..tokens.len()).step_by(2) {
                let thing = tokens[i];
                let value = tokens[i + 1].parse().unwrap();
                things.insert(thing, value);
            }
            sues.push(Sue { number, things });
        }
        Solver { sues }
    }

    fn find_best_fit(
        &self,
        detector_result: &HashMap<&'static str, u8>,
        compensate_for_outdated_retroencabulator: bool,
    ) -> Vec<&Sue> {
        fn get_similarity_index(
            a: &HashMap<&'static str, u8>,
            b: &HashMap<&'static str, u8>,
            compensate_for_outdated_retroencabulator: bool,
        ) -> i32 {
            let mut total_diff = 0;
            for (name, value) in a {
                let test_value = b.get(name).unwrap();
                let mut diff = value.abs_diff(*test_value);
                if compensate_for_outdated_retroencabulator {
                    if *name == "cats" || *name == "trees" {
                        if value > test_value {
                            diff = 0;
                        } else {
                            diff = u8::MAX;
                        }
                    }
                    if *name == "pomeranians" || *name == "goldfish" {
                        if value < test_value {
                            diff = 0;
                        } else {
                            diff = u8::MAX;
                        }
                    }
                }
                total_diff += diff as i32;
            }
            total_diff
        }
        let mut best_sues: Vec<&Sue> = vec![&self.sues[0]];
        let mut best_similarity_index: i32 = get_similarity_index(
            &self.sues[0].things,
            detector_result,
            compensate_for_outdated_retroencabulator,
        );
        for sue in self.sues.iter() {
            let similarity_index = get_similarity_index(
                &sue.things,
                detector_result,
                compensate_for_outdated_retroencabulator,
            );
            match similarity_index.cmp(&best_similarity_index) {
                std::cmp::Ordering::Less => {
                    best_sues = vec![sue];
                    best_similarity_index = similarity_index;
                }
                std::cmp::Ordering::Equal => {
                    best_sues.push(sue);
                }
                std::cmp::Ordering::Greater => {}
            }
        }
        best_sues
    }

    fn get_detector_result() -> HashMap<&'static str, u8> {
        let mut detector_result = HashMap::new();
        detector_result.insert("children", 3);
        detector_result.insert("cats", 7);
        detector_result.insert("samoyeds", 2);
        detector_result.insert("pomeranians", 3);
        detector_result.insert("akitas", 0);
        detector_result.insert("vizslas", 0);
        detector_result.insert("goldfish", 5);
        detector_result.insert("trees", 3);
        detector_result.insert("cars", 2);
        detector_result.insert("perfumes", 1);
        detector_result
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let detector_result = Solver::get_detector_result();
        let best_fit_sues = self.find_best_fit(&detector_result, false);
        //println!("{best_fit_sues:?}");
        assert!(best_fit_sues.len() == 1);
        best_fit_sues[0].number.to_string()
    }

    fn solve_part_two(&self) -> String {
        let detector_result = Solver::get_detector_result();
        let best_fit_sues = self.find_best_fit(&detector_result, true);
        //println!("{best_fit_sues:?}");
        assert!(best_fit_sues.len() == 1);
        best_fit_sues[0].number.to_string()
    }

    fn day_number(&self) -> usize {
        16
    }

    fn description(&self) -> &'static str {
        "Aunt Sue"
    }
}
