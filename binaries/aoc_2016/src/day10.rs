use std::collections::{HashMap, HashSet};

use solver::SolverBase;

enum Target {
    Output(u8),
    Bot(u8),
}

enum Action {
    ValueGoesTo {
        value: u8,
        bot_number: u8,
    },
    BotGives {
        bot_number: u8,
        low_target: Target,
        high_target: Target,
    },
}

pub struct Solver {
    actions: Vec<Action>,
    low_value_microchip: u8,
    high_value_microchip: u8,
}

enum RequiredResult {
    BotValue,
    ValueFactor,
}

impl Solver {
    pub fn new(input: &'static str, low_value_microchip: u8, high_value_microchip: u8) -> Self {
        fn parse_target(target_type: &str, target_number: &str) -> Target {
            let target_number = target_number.parse().unwrap();
            match target_type {
                "bot" => Target::Bot(target_number),
                "output" => Target::Output(target_number),
                _ => panic!("unrecognized target type"),
            }
        }

        let mut actions = Vec::new();
        for line in input.lines() {
            let tokens: Vec<_> = line.split_whitespace().collect();
            let action = match &tokens[..] {
                ["value", value_number, "goes", "to", "bot", bot_number] => Action::ValueGoesTo {
                    value: value_number.parse().unwrap(),
                    bot_number: bot_number.parse().unwrap(),
                },
                [
                    "bot",
                    bot_number,
                    "gives",
                    "low",
                    "to",
                    low_target_type,
                    low_target_number,
                    "and",
                    "high",
                    "to",
                    high_target_type,
                    high_target_number,
                ] => Action::BotGives {
                    bot_number: bot_number.parse().unwrap(),
                    low_target: parse_target(low_target_type, low_target_number),
                    high_target: parse_target(high_target_type, high_target_number),
                },

                _ => panic!("unrecognized action"),
            };
            actions.push(action);
        }
        Solver {
            actions,
            low_value_microchip,
            high_value_microchip,
        }
    }

    fn solve(&self, result_type: RequiredResult) -> String {
        let mut bot_values = HashMap::<u8, HashSet<u8>>::new();
        let mut output_values = HashMap::<u8, u8>::new();

        fn get_bot_low_high(
            bot_values: &HashMap<u8, HashSet<u8>>,
            bot_number: u8,
        ) -> Option<(u8, u8)> {
            if let Some(values) = bot_values.get(&bot_number) {
                if values.len() == 2 {
                    let low_value = values.iter().min().unwrap();
                    let high_value = values.iter().max().unwrap();
                    return Some((*low_value, *high_value));
                }
            }
            None
        }

        let result: u32 = loop {
            match result_type {
                RequiredResult::ValueFactor => {
                    if let (Some(&v0), Some(&v1), Some(&v2)) = (
                        output_values.get(&0),
                        output_values.get(&1),
                        output_values.get(&2),
                    ) {
                        break v0 as u32 * v1 as u32 * v2 as u32;
                    }
                }
                RequiredResult::BotValue => {
                    let result: Vec<_> = bot_values
                        .iter()
                        .filter_map(|(bot_number, values)| {
                            if values.contains(&self.low_value_microchip)
                                && values.contains(&self.high_value_microchip)
                            {
                                Some(*bot_number)
                            } else {
                                None
                            }
                        })
                        .collect();
                    if !result.is_empty() {
                        //println!("Result: {:?}", result);
                        break result[0] as u32;
                    }
                }
            }

            for action in self.actions.iter() {
                match action {
                    Action::ValueGoesTo { value, bot_number } => {
                        bot_values.entry(*bot_number).or_default().insert(*value);
                    }
                    Action::BotGives {
                        bot_number,
                        low_target: Target::Output(low_output),
                        high_target: Target::Output(high_output),
                    } => {
                        if let Some((low_value, high_value)) =
                            get_bot_low_high(&bot_values, *bot_number)
                        {
                            output_values.insert(*low_output, low_value);
                            output_values.insert(*high_output, high_value);
                            bot_values.entry(*bot_number).or_default().clear();
                        }
                    }
                    Action::BotGives {
                        bot_number,
                        low_target: Target::Output(low_output),
                        high_target: Target::Bot(high_bot),
                    } => {
                        if let Some((low_value, high_value)) =
                            get_bot_low_high(&bot_values, *bot_number)
                        {
                            output_values.insert(*low_output, low_value);
                            bot_values.entry(*high_bot).or_default().insert(high_value);
                            bot_values.entry(*bot_number).or_default().clear();
                        }
                    }
                    Action::BotGives {
                        bot_number,
                        low_target: Target::Bot(low_bot),
                        high_target: Target::Output(high_output),
                    } => {
                        if let Some((low_value, high_value)) =
                            get_bot_low_high(&bot_values, *bot_number)
                        {
                            bot_values.entry(*low_bot).or_default().insert(low_value);
                            output_values.insert(*high_output, high_value);
                            bot_values.entry(*bot_number).or_default().clear();
                        }
                    }
                    Action::BotGives {
                        bot_number,
                        low_target: Target::Bot(low_bot),
                        high_target: Target::Bot(high_bot),
                    } => {
                        if let Some((low_value, high_value)) =
                            get_bot_low_high(&bot_values, *bot_number)
                        {
                            bot_values.entry(*low_bot).or_default().insert(low_value);
                            bot_values.entry(*high_bot).or_default().insert(high_value);
                            bot_values.entry(*bot_number).or_default().clear();
                        }
                    }
                }
            }
        };

        // for (bot, values) in bot_values.iter() {
        //     println!("Bot {bot}: {:?}", values);
        // }

        // for (output, value) in output_values.iter() {
        //     println!("Output {output}: {value}");
        // }

        result.to_string()
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        self.solve(RequiredResult::BotValue)
    }

    fn solve_part_two(&self) -> String {
        self.solve(RequiredResult::ValueFactor)
    }

    fn day_number(&self) -> usize {
        10
    }

    fn description(&self) -> &'static str {
        "Bots and microchips"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r"value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2",
            2,
            5,
        )
        .solve_part_one();
        assert_eq!(result, "2");
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
