use solver::Solver;

mod day01;
mod day02;
mod day03;
mod day04;

fn main() {
    let solvers: Vec<Box<dyn Solver>> = vec![
        Box::new(day01::Day01Solver::new(include_str!("../input/day01.txt"))),
        Box::new(day02::Day02Solver::new(include_str!("../input/day02.txt"))),
        Box::new(day03::Day03Solver::new(include_str!("../input/day03.txt"))),
        Box::new(day04::Day04Solver::new("yzbqklnj")),
    ];

    for solver in solvers {
        println!("Day {}: {}", solver.day_number(), solver.description());
        println!("  Part 1: {}", solver.solve_part_one());
        println!("  Part 2: {}", solver.solve_part_two());
    }
}
