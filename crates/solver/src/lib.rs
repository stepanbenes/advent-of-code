pub trait Solver {
    fn solve_part_one(&self) -> String {
        "".to_owned()
    }
    fn solve_part_two(&self) -> String {
        "".to_owned()
    }
    fn day_number(&self) -> usize;
    fn description(&self) -> &'static str {
        ""
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
