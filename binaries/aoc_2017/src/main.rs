use std::vec;

use solver::SolverBase;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

fn main() {
    let solvers: Vec<Box<dyn SolverBase>> = vec![
        Box::new(day01::Solver::new(include_str!("../input/day01.txt"))),
        Box::new(day02::Solver::new(include_str!("../input/day02.txt"))),
        Box::new(day03::Solver::new("265149")),
        Box::new(day04::Solver::new(include_str!("../input/day04.txt"))),
        Box::new(day05::Solver::new(include_str!("../input/day05.txt"))),
        Box::new(day06::Solver::new("11	11	13	7	0	15	5	5	4	4	1	1	7	1	15	11")),
        Box::new(day07::Solver::new(include_str!("../input/day07.txt"))),
        Box::new(day08::Solver::new(include_str!("../input/day08.txt"))),
        Box::new(day09::Solver::new(include_str!("../input/day09.txt"))),
    ];

    for solver in solvers {
        println!("Day {}: {}", solver.day_number(), solver.description());
        if solver.skip_run() {
            println!("  skipping...")
        } else {
            println!("  Part 1: {}", solver.solve_part_one());
            println!("  Part 2: {}", solver.solve_part_two());
        }
    }
}
