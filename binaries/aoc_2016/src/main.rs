use solver::SolverBase;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() {
    let solvers: Vec<Box<dyn SolverBase>> = vec![
        Box::new(day01::Solver::new(include_str!("../input/day01.txt"))),
        Box::new(day02::Solver::new(include_str!("../input/day02.txt"))),
        Box::new(day03::Solver::new(include_str!("../input/day03.txt"))),
        Box::new(day04::Solver::new(include_str!("../input/day04.txt"))),
        Box::new(day05::Solver::new("abbhdwsy")),
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
