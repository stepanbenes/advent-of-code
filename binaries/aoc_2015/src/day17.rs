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

    fn count_combinations(
        containers: Vec<i32>,
        volume: i32,
        container_count_limit: Option<usize>,
    ) -> (i32, usize) {
        fn backtrack(
            containers: &Vec<i32>,
            index: usize,
            current_sum: i32,
            target: i32,
            container_count: usize,
            container_count_limit: Option<usize>,
            min_countainer_count: &mut usize,
        ) -> i32 {
            if let Some(container_count_limit) = container_count_limit {
                if container_count > container_count_limit {
                    return 0;
                }
            }
            // Base case: current_sum equals target
            if current_sum == target {
                if container_count < *min_countainer_count {
                    *min_countainer_count = container_count;
                }
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
                container_count + 1,
                container_count_limit,
                min_countainer_count,
            );
            let exclude = backtrack(
                containers,
                index + 1,
                current_sum,
                target,
                container_count,
                container_count_limit,
                min_countainer_count,
            );

            include + exclude
        }
        let mut min_countainer_count = containers.len();

        let count = backtrack(
            &containers,
            0,
            0,
            volume,
            0,
            container_count_limit,
            &mut min_countainer_count,
        );
        (count, min_countainer_count)
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let (count, _) = Solver::count_combinations(self.containers.clone(), self.volume, None);
        count.to_string()
    }

    fn solve_part_two(&self) -> String {
        let (_, min_container_count) =
            Solver::count_combinations(self.containers.clone(), self.volume, None);
        let (count, _) = Solver::count_combinations(
            self.containers.clone(),
            self.volume,
            Some(min_container_count),
        );
        count.to_string()
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
