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
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;

fn main() {
    let solvers: Vec<Box<dyn SolverBase>> = vec![
        Box::new(day01::Solver::new(include_str!("../input/day01.txt"))),
        Box::new(day02::Solver::new(include_str!("../input/day02.txt"))),
        Box::new(day03::Solver::new(include_str!("../input/day03.txt"))),
        Box::new(day04::Solver::new("yzbqklnj")),
        Box::new(day05::Solver::new(include_str!("../input/day05.txt"))),
        Box::new(day06::Solver::new(include_str!("../input/day06.txt"))),
        Box::new(day07::Solver::new(include_str!("../input/day07.txt"))),
        Box::new(day08::Solver::new(include_str!("../input/day08.txt"))),
        Box::new(day09::Solver::new(include_str!("../input/day09.txt"))),
        Box::new(day10::Solver::new("1113122113", 40, 50)),
        Box::new(day11::Solver::new("cqjxjnds")),
        Box::new(day12::Solver::new(include_str!("../input/day12.json"))),
        Box::new(day13::Solver::new(include_str!("../input/day13.txt"))),
        Box::new(day14::Solver::new(include_str!("../input/day14.txt"), 2503)),
        Box::new(day15::Solver::new(include_str!("../input/day15.txt"))),
        Box::new(day16::Solver::new(include_str!("../input/day16.txt"))),
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
