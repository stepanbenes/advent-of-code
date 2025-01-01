use solver::Solver;

pub struct Day04Solver {
    input: &'static str,
}

impl Day04Solver {
    pub fn new(input: &'static str) -> Self {
        Day04Solver { input }
    }
}

impl Solver for Day04Solver {
    fn solve_part_one(&self) -> String {
        for n in 0..u32::MAX {
            let test = format!("{}{}", self.input, n);
            let hash = md5::compute(test);
            let hash_hex = format!("{:x}", hash);
            if hash_hex.starts_with("00000") {
                return n.to_string();
            }
        }
        "".to_string()
    }

    fn solve_part_two(&self) -> String {
        for n in 0..u32::MAX {
            let test = format!("{}{}", self.input, n);
            let hash = md5::compute(test);
            let hash_hex = format!("{:x}", hash);
            if hash_hex.starts_with("000000") {
                return n.to_string();
            }
        }
        "".to_string()
    }

    fn day_number(&self) -> usize {
        4
    }

    fn description(&self) -> &'static str {
        "MD5 hashes"
    }

    fn skip_run(&self) -> bool {
        true // too slow
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Day04Solver::new("abcdef").solve_part_one();
        assert_eq!(result, "609043");
    }

    #[test]
    fn test_2() {
        let result = Day04Solver::new("pqrstuv").solve_part_one();
        assert_eq!(result, "1048970");
    }
}
