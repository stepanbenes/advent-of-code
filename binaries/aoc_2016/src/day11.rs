use std::collections::{HashMap, HashSet, VecDeque};

use solver::SolverBase;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Item {
    Microchip(&'static str),
    Generator(&'static str),
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::Microchip(name) => write!(f, "{}{}M", name[0..1].to_uppercase(), &name[1..2]),
            Item::Generator(name) => write!(f, "{}{}G", name[0..1].to_uppercase(), &name[1..2]),
        }
    }
}

type FloorIndex = usize;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    elevator_floor: FloorIndex,
    item_floors: Vec<(Item, FloorIndex)>,
}

impl State {
    fn generate_next_valid_states(&self, floor_count: usize) -> Vec<State> {
        // Generate all possible moves
        // Filter out invalid moves
        // Return valid moves

        fn get_items_on_elevator_floor(
            state: &mut State,
            elevator_floor: FloorIndex,
        ) -> Vec<&mut (Item, FloorIndex)> {
            let items_on_elevator_floor: Vec<&mut (Item, FloorIndex)> = state
                .item_floors
                .iter_mut()
                .filter(|(_item, floor_index)| *floor_index == elevator_floor)
                .collect();
            items_on_elevator_floor
        }

        let count_of_items_on_elevator_floor = self
            .item_floors
            .iter()
            .filter(|(_item, floor_index)| *floor_index == self.elevator_floor)
            .count();

        let mut valid_states = Vec::new();

        for i in 0..count_of_items_on_elevator_floor {
            // move single item up
            if self.elevator_floor < floor_count - 1 {
                let mut new_state = self.clone();
                new_state.elevator_floor += 1;
                let mut items_on_elevator_floor =
                    get_items_on_elevator_floor(&mut new_state, self.elevator_floor);
                items_on_elevator_floor[i].1 += 1;
                if new_state.is_valid() {
                    valid_states.push(new_state);
                }
            }
            // move single item down
            if self.elevator_floor > 0 {
                let mut new_state = self.clone();
                new_state.elevator_floor -= 1;
                let mut items_on_elevator_floor =
                    get_items_on_elevator_floor(&mut new_state, self.elevator_floor);
                items_on_elevator_floor[i].1 -= 1;
                if new_state.is_valid() {
                    valid_states.push(new_state);
                }
            }

            for j in i + 1..count_of_items_on_elevator_floor {
                // move two items up
                if self.elevator_floor < floor_count - 1 {
                    let mut new_state = self.clone();
                    new_state.elevator_floor += 1;
                    let mut items_on_elevator_floor =
                        get_items_on_elevator_floor(&mut new_state, self.elevator_floor);
                    items_on_elevator_floor[i].1 += 1;
                    items_on_elevator_floor[j].1 += 1;
                    if new_state.is_valid() {
                        valid_states.push(new_state);
                    }
                }
                // move two items down
                if self.elevator_floor > 0 {
                    let mut new_state = self.clone();
                    new_state.elevator_floor -= 1;
                    let mut items_on_elevator_floor =
                        get_items_on_elevator_floor(&mut new_state, self.elevator_floor);
                    items_on_elevator_floor[i].1 -= 1;
                    items_on_elevator_floor[j].1 -= 1;
                    if new_state.is_valid() {
                        valid_states.push(new_state);
                    }
                }
            }
        }

        valid_states
    }

    fn is_valid(&self) -> bool {
        let mut floor_items: HashMap<FloorIndex, Vec<Item>> = HashMap::new();
        for (item, floor_index) in &self.item_floors {
            floor_items.entry(*floor_index).or_default().push(*item);
        }
        for items in floor_items.values() {
            let microchips: HashSet<&str> = items
                .iter()
                .filter_map(|item| match item {
                    Item::Microchip(name) => Some(*name),
                    _ => None,
                })
                .collect();
            let generators: HashSet<&str> = items
                .iter()
                .filter_map(|item| match item {
                    Item::Generator(name) => Some(*name),
                    _ => None,
                })
                .collect();
            // if a chip is ever left in the same area as another RTG, and it's not connected to its own RTG, the chip will be fried.
            for name in microchips {
                if !generators.is_empty() && !generators.contains(name) {
                    return false;
                }
            }
        }
        true
    }

    fn print(&self, floor_count: usize) {
        for floor_index in (0..floor_count).rev() {
            print!("F{} ", floor_index + 1);
            if self.elevator_floor == floor_index {
                print!("E ");
            } else {
                print!(". ");
            }
            for (item, item_floor) in &self.item_floors {
                if *item_floor == floor_index {
                    print!("{} ", item);
                } else {
                    print!("... ");
                }
            }
            println!();
        }
        println!();
    }
}

pub struct Solver {
    init_state: State,
    floor_count: usize,
}

impl Solver {
    #[allow(dead_code)]
    fn new(elevator_floor: FloorIndex, items: Vec<(Item, FloorIndex)>, floor_count: usize) -> Self {
        Solver {
            init_state: State {
                elevator_floor,
                item_floors: items,
            },
            floor_count,
        }
    }

    pub fn new_from_input() -> Self {
        Solver {
            init_state: State {
                elevator_floor: 0,
                item_floors: vec![
                    (Item::Generator("polonium"), 0),
                    (Item::Generator("thulium"), 0),
                    (Item::Microchip("thulium"), 0),
                    (Item::Generator("promethium"), 0),
                    (Item::Generator("ruthenium"), 0),
                    (Item::Microchip("ruthenium"), 0),
                    (Item::Generator("cobalt"), 0),
                    (Item::Microchip("cobalt"), 0),
                    (Item::Microchip("polonium"), 1),
                    (Item::Microchip("promethium"), 1),
                ],
            },
            floor_count: 4,
        }
    }

    fn bfs(init_state: State, end_state: State, floor_count: usize) -> Vec<State> {
        let mut states: VecDeque<State> = VecDeque::new();
        let mut visited_states: HashSet<State> = HashSet::new();
        let mut parent_states: HashMap<State, State> = HashMap::new();
        states.push_back(init_state.clone());
        while let Some(state) = states.pop_front() {
            visited_states.insert(state.clone());

            if state == end_state {
                let mut path = Vec::new();
                path.push(state.clone());
                let mut step_tracker = state;
                while let Some(parent) = parent_states.get(&step_tracker) {
                    path.push(parent.clone());
                    step_tracker = parent.clone();
                }
                path.reverse();
                return path;
            }

            for next_state in state.generate_next_valid_states(floor_count) {
                if !visited_states.contains(&next_state) {
                    states.push_back(next_state.clone());
                    parent_states.insert(next_state, state.clone());
                }
            }
        }
        vec![init_state]
    }

    fn get_final_state(&self) -> State {
        let last_floor = self.floor_count - 1;
        State {
            elevator_floor: last_floor,
            item_floors: self
                .init_state
                .item_floors
                .iter()
                .map(|(item, _floor_index)| (*item, last_floor))
                .collect(),
        }
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        // println!("Initial state:");
        // self.init_state.print(self.floor_count);
        // println!("Final state:");
        // self.get_final_state().print(self.floor_count);
        // println!("Floor count: {}", self.floor_count);

        let path = Solver::bfs(
            self.init_state.clone(),
            self.get_final_state(),
            self.floor_count,
        );

        for state in &path {
            state.print(self.floor_count);
            println!();
        }

        (path.len() - 1).to_string()
    }

    fn solve_part_two(&self) -> String {
        "".to_string()
    }

    fn day_number(&self) -> usize {
        11
    }

    fn description(&self) -> &'static str {
        "Fox, cabbage, goat but with RTGs and microchips"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    // The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
    // The second floor contains a hydrogen generator.
    // The third floor contains a lithium generator.
    // The fourth floor contains nothing relevant."
    #[test]
    fn test_1() {
        let result = Solver::new(
            0,
            vec![
                (Item::Generator("hydrogen"), 1),
                (Item::Microchip("hydrogen"), 0),
                (Item::Generator("lithium"), 2),
                (Item::Microchip("lithium"), 0),
            ],
            4,
        )
        .solve_part_one();
        assert_eq!(result, "11");
    }

    #[test]
    fn test_final_state_1() {
        let result = Solver::new(
            0,
            vec![
                (Item::Microchip("hydrogen"), 0),
                (Item::Microchip("lithium"), 0),
                (Item::Generator("hydrogen"), 1),
                (Item::Generator("lithium"), 2),
            ],
            4,
        )
        .get_final_state();
        assert_eq!(
            result,
            State {
                elevator_floor: 3,
                item_floors: vec![
                    (Item::Microchip("hydrogen"), 3),
                    (Item::Microchip("lithium"), 3),
                    (Item::Generator("hydrogen"), 3),
                    (Item::Generator("lithium"), 3),
                ]
            }
        );
    }

    #[test]
    fn test_is_valid_1() {
        let result = Solver::new(
            0,
            vec![
                (Item::Microchip("hydrogen"), 0),
                (Item::Microchip("lithium"), 0),
                (Item::Generator("hydrogen"), 1),
                (Item::Generator("lithium"), 2),
            ],
            4,
        )
        .init_state
        .is_valid();
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_valid_2() {
        let result = Solver::new(
            0,
            vec![
                (Item::Microchip("hydrogen"), 1),
                (Item::Microchip("lithium"), 1),
                (Item::Generator("hydrogen"), 1),
                (Item::Generator("lithium"), 1),
            ],
            4,
        )
        .init_state
        .is_valid();
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_valid_3() {
        let result = Solver::new(
            0,
            vec![
                (Item::Microchip("hydrogen"), 1),
                (Item::Microchip("lithium"), 0),
                (Item::Generator("hydrogen"), 1),
                (Item::Generator("lithium"), 1),
            ],
            4,
        )
        .init_state
        .is_valid();
        assert_eq!(result, true);
    }

    #[test]
    fn test_next_valid_states() {
        let result = State {
            elevator_floor: 0,
            item_floors: vec![
                (Item::Microchip("hydrogen"), 0),
                (Item::Microchip("lithium"), 0),
                (Item::Generator("hydrogen"), 1),
                (Item::Generator("lithium"), 2),
            ],
        }
        .generate_next_valid_states(4);

        assert_eq!(
            result,
            vec![State {
                elevator_floor: 1,
                item_floors: vec![
                    (Item::Microchip("hydrogen"), 1),
                    (Item::Microchip("lithium"), 0),
                    (Item::Generator("hydrogen"), 1),
                    (Item::Generator("lithium"), 2),
                ]
            }]
        );
    }
}

// #[cfg(test)]
// mod part2_tests {
//     use super::*;

//     #[test]
//     fn test_1() {
//         let result = Solver::new("abc").solve_part_two();
//         assert_eq!(result, "0");
//     }
// }
