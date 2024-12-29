use solver::Solver;

mod day01;

fn main() {
    let day01_solver = day01::Day01Solver::new(include_str!("../input/day01.txt"));
    println!("Day 01, part 1: {}", day01_solver.solve_part_one());
    println!("Day 01, part 2: {}", day01_solver.solve_part_two());
}
