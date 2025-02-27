use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;
use solver::SolverBase;

#[derive(Debug)]
struct Room {
    codes: Vec<&'static str>,
    id: u32,
    checksum: &'static str,
}

pub struct Solver {
    rooms: Vec<Room>,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        let mut rooms = Vec::new();
        for line in input.lines() {
            let parts = line.split(['[', ']']).collect::<Vec<_>>();
            if let [code_strings, checksum, ..] = &parts[..] {
                let tokens = code_strings.split('-').collect::<Vec<_>>();
                let codes = tokens[..tokens.len() - 1].to_vec();
                let id = tokens[tokens.len() - 1].parse().unwrap();
                rooms.push(Room {
                    codes,
                    id,
                    checksum,
                });
            }
        }
        Solver { rooms }
    }

    fn is_real(room: &Room) -> bool {
        let mut char_counts = HashMap::<char, usize>::new();
        for code in room.codes.iter() {
            for c in code.chars() {
                *char_counts.entry(c).or_default() += 1;
            }
        }

        fn comparer(
            (a_char, a_count): &(&char, &usize),
            (b_char, b_count): &(&char, &usize),
        ) -> Ordering {
            let count_ordering = b_count.cmp(a_count);
            match count_ordering {
                Ordering::Equal => a_char.cmp(b_char),
                _ => count_ordering,
            }
        }

        let most_common_chars = char_counts
            .iter()
            .sorted_by(comparer)
            .map(|(a, _)| a)
            .take(5)
            .collect::<String>();
        //println!("{most_common_chars}");
        most_common_chars == room.checksum
    }

    fn shift(room: &Room) -> String {
        fn shift_char(c: char, shift: u32) -> char {
            if c.is_ascii_lowercase() {
                // Calculate the new character with wrapping
                let base = b'a';
                let offset = ((c as u8 - base) as u32 + shift).rem_euclid(26);
                (base + offset as u8) as char
            } else {
                c // Return the character unchanged if not in range
            }
        }

        let mut result = Vec::new();
        for code in room.codes.iter() {
            let mut shifted_code = String::new();
            for c in code.chars() {
                let shifted_c = shift_char(c, room.id);
                shifted_code.push(shifted_c);
            }
            result.push(shifted_code);
        }
        result.join(" ")
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let result: u32 = self
            .rooms
            .iter()
            .filter(|room| Solver::is_real(room))
            .map(|room| room.id)
            .sum();
        result.to_string()
    }

    fn solve_part_two(&self) -> String {
        for room in self.rooms.iter() {
            let decoded_room = Solver::shift(room);
            //println!("{decoded_room}");
            if decoded_room == "northpole object storage" {
                return room.id.to_string();
            }
        }
        "".to_string()
    }

    fn day_number(&self) -> usize {
        4
    }

    fn description(&self) -> &'static str {
        "Encrypted list of rooms"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new("aaaaa-bbb-z-y-x-123[abxyz]").solve_part_one();
        assert_eq!(result, "123");
    }

    #[test]
    fn test_2() {
        let result = Solver::new("a-b-c-d-e-f-g-h-987[abcde]").solve_part_one();
        assert_eq!(result, "987");
    }

    #[test]
    fn test_3() {
        let result = Solver::new("not-a-real-room-404[oarel]").solve_part_one();
        assert_eq!(result, "404");
    }

    #[test]
    fn test_4() {
        let result = Solver::new("totally-real-room-200[decoy]").solve_part_one();
        assert_eq!(result, "0");
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::shift(&Solver::new("qzmt-zixmtkozy-ivhz-343[x]").rooms[0]);
        assert_eq!(result, "very encrypted name");
    }
}
