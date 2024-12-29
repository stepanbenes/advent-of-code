pub trait Solver<'a> {
    type Part1Output;
    type Part2Output;
    fn new(input: &'a str) -> Self;
    fn solve_part_one(&self) -> Self::Part1Output;
    fn solve_part_two(&self) -> Self::Part2Output;
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
