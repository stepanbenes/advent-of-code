use solver::SolverBase;

pub struct Solver {
    start_value_a: u64,
    start_value_b: u64,
}

impl Solver {
    pub fn new(start_value_a: u64, start_value_b: u64) -> Self {
        Solver {
            start_value_a,
            start_value_b,
        }
    }

    fn get_matches(&self, count: usize) -> usize {
        let mut match_count = 0;
        let mut a = self.start_value_a;
        let mut b = self.start_value_b;
        let factor_a = 16807;
        let factor_b = 48271;
        let mod_n = 2147483647;
        for _ in 0..count {
            a = (a * factor_a) % mod_n;
            b = (b * factor_b) % mod_n;
            let a_low16 = a & (u16::MAX as u64);
            let b_low16 = b & (u16::MAX as u64);
            if a_low16 == b_low16 {
                match_count += 1;
            }
        }
        match_count
    }

    fn get_matches_2(&self, count: usize) -> usize {
        let mut match_count = 0;
        let mut a = self.start_value_a;
        let mut b = self.start_value_b;
        let factor_a = 16807;
        let factor_b = 48271;
        let mod_n = 2147483647;
        let mut get_next_a = move || {
            loop {
                a = (a * factor_a) % mod_n;
                if a % 4 == 0 {
                    break;
                }
            }
            a
        };
        let mut get_next_b = move || {
            loop {
                b = (b * factor_b) % mod_n;
                if b % 8 == 0 {
                    break;
                }
            }
            b
        };
        for _ in 0..count {
            //a = (a * factor_a) % mod_n;
            //b = (b * factor_b) % mod_n;
            a = get_next_a();
            b = get_next_b();
            let a_low16 = a & (u16::MAX as u64);
            let b_low16 = b & (u16::MAX as u64);
            if a_low16 == b_low16 {
                match_count += 1;
            }
        }
        match_count
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let match_count = self.get_matches(40_000_000);
        match_count.to_string()
    }

    fn solve_part_two(&self) -> String {
        let match_count = self.get_matches_2(5_000_000);
        match_count.to_string()
    }

    fn day_number(&self) -> usize {
        15
    }

    fn description(&self) -> &'static str {
        "Dueling Generators"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(65, 8921).solve_part_one();
        assert_eq!(result, "588");
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(65, 8921).solve_part_two();
        assert_eq!(result, "309");
    }
}
