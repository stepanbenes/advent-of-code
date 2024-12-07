pub fn get_total_calibration_result(with_concatanation: bool) -> u64 {
    let equations = get_input();
    equations
        .iter()
        .filter(|equation| check_satisfiability(equation, with_concatanation))
        .map(|equation| equation.result)
        .sum()
}

#[derive(Debug)]
struct Equation {
    result: u64,
    operands: Vec<u64>,
}

fn get_input() -> Vec<Equation> {
    let input = include_str!("../input/day7.txt");
    //     let input = r"190: 10 19
    // 3267: 81 40 27
    // 83: 17 5
    // 156: 15 6
    // 7290: 6 8 6 15
    // 161011: 16 10 13
    // 192: 17 8 14
    // 21037: 9 7 18 13
    // 292: 11 6 16 20";
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let result = parts.next().unwrap().parse().unwrap();
            let operands = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|operand| operand.parse().unwrap())
                .collect();
            Equation { result, operands }
        })
        .collect()
}

fn check_satisfiability(equation: &Equation, with_concatanation: bool) -> bool {
    let Equation { result, operands } = equation;
    if with_concatanation {
        calculate_equation_result_with_concatanation(&operands[1..], operands[0], *result)
    } else {
        calculate_equation_result(&operands[1..], operands[0], *result)
    }
}

fn calculate_equation_result(operands: &[u64], running_result: u64, total_result: u64) -> bool {
    match operands {
        [] => running_result == total_result,
        [a, rest @ ..] => {
            calculate_equation_result(rest, a + running_result, total_result)
                || calculate_equation_result(rest, a * running_result, total_result)
        }
    }
}

fn calculate_equation_result_with_concatanation(
    operands: &[u64],
    running_result: u64,
    total_result: u64,
) -> bool {
    match operands {
        [] => running_result == total_result,
        [a, rest @ ..] => {
            calculate_equation_result_with_concatanation(rest, running_result + a, total_result)
                || calculate_equation_result_with_concatanation(
                    rest,
                    running_result * a,
                    total_result,
                )
                || calculate_equation_result_with_concatanation(
                    rest,
                    concetenate_numbers(running_result, *a),
                    total_result,
                )
        }
    }
}

pub fn concetenate_numbers(first: u64, second: u64) -> u64 {
    let mut b = second;
    let mut shift = 1;
    while b > 0 {
        shift *= 10;
        b /= 10;
    }
    first * shift + second
}
