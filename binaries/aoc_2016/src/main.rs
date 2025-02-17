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
mod day10;
mod day11;

fn main() {
    let solvers: Vec<Box<dyn SolverBase>> = vec![
        Box::new(day01::Solver::new(include_str!("../input/day01.txt"))),
        Box::new(day02::Solver::new(include_str!("../input/day02.txt"))),
        Box::new(day03::Solver::new(include_str!("../input/day03.txt"))),
        Box::new(day04::Solver::new(include_str!("../input/day04.txt"))),
        Box::new(day05::Solver::new("abbhdwsy")),
        Box::new(day06::Solver::new(include_str!("../input/day06.txt"))),
        Box::new(day07::Solver::new(include_str!("../input/day07.txt"))),
        Box::new(day08::Solver::<50, 6>::new(include_str!(
            "../input/day08.txt"
        ))),
        Box::new(day09::Solver::new(include_str!("../input/day09.txt"))),
        Box::new(day10::Solver::new(
            include_str!("../input/day10.txt"),
            17,
            61,
        )),
        Box::new(day11::Solver::new_from_input()),
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
