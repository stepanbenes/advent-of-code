use solver::Solver;

struct Reindeer {
    speed: u32,     // km/s
    fly_time: u32,  // s
    rest_time: u32, // s
}

pub struct Day14Solver {
    reindeers: Vec<Reindeer>,
    time: u32,
}

impl Day14Solver {
    pub fn new(input: &'static str, time: u32) -> Self {
        let mut reindeers = Vec::new();
        for line in input.lines() {
            let tokens: Vec<_> = line.split_whitespace().collect();
            if let [_name, "can", "fly", speed, "km/s", "for", fly_time, "seconds,", "but", "then", "must", "rest", "for", rest_time, "seconds."] =
                &tokens[..]
            {
                reindeers.push(Reindeer {
                    speed: speed.parse().unwrap(),
                    fly_time: fly_time.parse().unwrap(),
                    rest_time: rest_time.parse().unwrap(),
                });
            }
        }
        Day14Solver { reindeers, time }
    }

    fn simulate_flying(&self, reindeer: &Reindeer) -> u32 {
        let mut distance = 0;
        let mut expired_time = 0;
        while expired_time < self.time {
            let remaining_time = self.time - expired_time;
            if remaining_time >= reindeer.fly_time {
                distance += reindeer.speed * reindeer.fly_time;
                expired_time += reindeer.fly_time;
            } else {
                distance += reindeer.speed * remaining_time;
                expired_time += remaining_time;
            }

            let remaining_time = self.time - expired_time;
            if remaining_time >= reindeer.rest_time {
                expired_time += reindeer.rest_time;
            } else {
                expired_time += remaining_time;
            }
        }
        distance
    }
}

impl Solver for Day14Solver {
    fn solve_part_one(&self) -> String {
        let distances: Vec<u32> = self
            .reindeers
            .iter()
            .map(|reindeer| self.simulate_flying(reindeer))
            .collect();
        let max = distances.iter().max().unwrap();
        max.to_string()
    }

    fn solve_part_two(&self) -> String {
        "".to_string()
    }

    fn day_number(&self) -> usize {
        14
    }

    fn description(&self) -> &'static str {
        "Flying reindeers"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Day14Solver::new(
            r"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
            1000,
        )
        .solve_part_one();
        assert_eq!(result, "1120");
    }
}

// #[cfg(test)]
// mod part2_tests {
//     use super::*;

//     #[test]
//     fn test_1() {
//         let result = Day14Solver::new("abc").solve_part_two();
//         assert_eq!(result, "0");
//     }
// }
