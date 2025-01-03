use solver::SolverBase;

struct Reindeer {
    speed: u32,     // km/s
    fly_time: u32,  // s
    rest_time: u32, // s
}

#[derive(Debug, Clone)]
enum ReindeerState {
    Flying(u32),
    Resting(u32),
}

pub struct Solver {
    reindeers: Vec<Reindeer>,
    time: u32,
}

impl Solver {
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
        Solver { reindeers, time }
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

    fn simulate_flying_with_bonus_points(&self) -> u32 {
        let mut reindeer_states: Vec<ReindeerState> =
            vec![ReindeerState::Flying(0); self.reindeers.len()];
        let mut reindeer_distances: Vec<u32> = vec![0; self.reindeers.len()];
        let mut bonus_points: Vec<u32> = vec![0; self.reindeers.len()];
        for _ in 0..self.time {
            for (i, reindeer) in self.reindeers.iter().enumerate() {
                match reindeer_states[i] {
                    ReindeerState::Flying(time) => {
                        if time == reindeer.fly_time {
                            reindeer_states[i] = ReindeerState::Resting(1);
                        } else {
                            reindeer_states[i] = ReindeerState::Flying(time + 1);
                            reindeer_distances[i] += reindeer.speed;
                        }
                    }
                    ReindeerState::Resting(time) => {
                        if time == reindeer.rest_time {
                            reindeer_states[i] = ReindeerState::Flying(1);
                            reindeer_distances[i] += reindeer.speed;
                        } else {
                            reindeer_states[i] = ReindeerState::Resting(time + 1);
                        }
                    }
                }
            }

            let max_distance = reindeer_distances.iter().max().unwrap();
            for (i, reindeer_distance) in reindeer_distances.iter().enumerate() {
                if reindeer_distance == max_distance {
                    bonus_points[i] += 1;
                }
            }
        }
        *bonus_points.iter().max().unwrap()
    }
}

impl SolverBase for Solver {
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
        let points = self.simulate_flying_with_bonus_points();
        points.to_string()
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
        let result = Solver::new(
            r"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
            1000,
        )
        .solve_part_one();
        assert_eq!(result, "1120");
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
            1000,
        )
        .solve_part_two();
        assert_eq!(result, "689");
    }
}
