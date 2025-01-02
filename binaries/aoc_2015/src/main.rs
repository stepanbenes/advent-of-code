use solver::Solver;

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

fn main() {
    let solvers: Vec<Box<dyn Solver>> = vec![
        Box::new(day01::Day01Solver::new(include_str!("../input/day01.txt"))),
        Box::new(day02::Day02Solver::new(include_str!("../input/day02.txt"))),
        Box::new(day03::Day03Solver::new(include_str!("../input/day03.txt"))),
        Box::new(day04::Day04Solver::new("yzbqklnj")),
        Box::new(day05::Day05Solver::new(include_str!("../input/day05.txt"))),
        Box::new(day06::Day06Solver::new(include_str!("../input/day06.txt"))),
        Box::new(day07::Day07Solver::new(include_str!("../input/day07.txt"))),
        Box::new(day08::Day08Solver::new(include_str!("../input/day08.txt"))),
        Box::new(day09::Day09Solver::new(include_str!("../input/day09.txt"))),
        Box::new(day10::Day10Solver::new("1113122113", 40, 50)),
        Box::new(day11::Day11Solver::new("cqjxjnds")),
        Box::new(day12::Day12Solver::new(include_str!("../input/day12.json"))),
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
