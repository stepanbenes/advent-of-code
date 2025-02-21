use std::vec;

use solver::SolverBase;

mod day01;
mod day02;

fn main() {
    let solvers: Vec<Box<dyn SolverBase>> = vec![
        Box::new(day01::Solver::new(include_str!("../input/day01.txt"))),
        Box::new(day02::Solver::new(include_str!("../input/day02.txt"))),
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
