use solver::SolverBase;

pub struct Solver {
    containers: Vec<i32>,
    volume: i32,
}

impl Solver {
    pub fn new(input: &'static str, volume: i32) -> Self {
        let buckets: Vec<_> = input.lines().map(|line| line.parse().unwrap()).collect();
        Solver {
            containers: buckets,
            volume,
        }
    }

    fn count_combinations(containers: Vec<i32>, volume: i32) -> i32 {
        fn backtrack(containers: &Vec<i32>, index: usize, current_sum: i32, target: i32) -> i32 {
            // Base case: current_sum equals target
            if current_sum == target {
                return 1;
            }
            // Base case: current_sum exceeds target or index is out of bounds
            if current_sum > target || index >= containers.len() {
                return 0;
            }

            // Recursive case: include or exclude the current container
            let include = backtrack(
                containers,
                index + 1,
                current_sum + containers[index],
                target,
            );
            let exclude = backtrack(containers, index + 1, current_sum, target);

            include + exclude
        }

        // Sort canisters to handle duplicates consistently
        //let mut sorted_containers = containers;
        //sorted_containers.sort();
        //backtrack(&sorted_containers, 0, 0, volume)
        backtrack(&containers, 0, 0, volume)
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let count = Solver::count_combinations(self.containers.clone(), self.volume);
        count.to_string()
    }

    fn solve_part_two(&self) -> String {
        "".to_string()
    }

    fn day_number(&self) -> usize {
        17
    }

    fn description(&self) -> &'static str {
        "Combinations of containers"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r"20
15
10
5
5",
            25,
        )
        .solve_part_one();
        assert_eq!(result, "4");
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r"20
15
10
5
5",
            25,
        )
        .solve_part_two();
        assert_eq!(result, "3");
    }
}
