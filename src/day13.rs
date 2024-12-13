pub fn fewest_tokens_to_win_all_possible_prices() -> i32 {
    let machines = get_input();
    let total_price = machines
        .iter()
        .filter_map(calculate_pushes)
        .map(calculate_price)
        .sum();
    total_price
}

#[derive(Debug, Default, Clone)]
struct Machine {
    a_x_increment: i32,
    a_y_increment: i32,
    b_x_increment: i32,
    b_y_increment: i32,

    price_location_x: i32,
    price_location_y: i32,
}

fn get_input() -> Vec<Machine> {
    //     let input = r"Button A: X+94, Y+34
    // Button B: X+22, Y+67
    // Prize: X=8400, Y=5400

    // Button A: X+26, Y+66
    // Button B: X+67, Y+21
    // Prize: X=12748, Y=12176

    // Button A: X+17, Y+86
    // Button B: X+84, Y+37
    // Prize: X=7870, Y=6450

    // Button A: X+69, Y+23
    // Button B: X+27, Y+71
    // Prize: X=18641, Y=10279";
    let input = include_str!("../input/day13.txt");

    let mut machines = vec![];
    let mut current_machine: Machine = Default::default();
    for line in input.lines() {
        let line_parts: Vec<_> = line
            .split([':', ' ', '+', ',', '='])
            .filter(|x| !x.is_empty())
            .collect();
        match &line_parts[..] {
            ["Button", "A", "X", x_str, "Y", y_str] => {
                current_machine.a_x_increment = x_str.parse().unwrap();
                current_machine.a_y_increment = y_str.parse().unwrap();
            }
            ["Button", "B", "X", x_str, "Y", y_str] => {
                current_machine.b_x_increment = x_str.parse().unwrap();
                current_machine.b_y_increment = y_str.parse().unwrap();
            }
            ["Prize", "X", x_str, "Y", y_str] => {
                current_machine.price_location_x = x_str.parse().unwrap();
                current_machine.price_location_y = y_str.parse().unwrap();
                machines.push(current_machine.clone());
            }
            _ => {}
        }
    }

    machines
}

fn calculate_price((a, b): (i32, i32)) -> i32 {
    a * 3 + b
}

fn calculate_pushes(machine: &Machine) -> Option<(i32, i32)> {
    let determinant = machine.a_x_increment * machine.b_y_increment
        - machine.a_y_increment * machine.b_x_increment;
    if determinant == 0 {
        return None;
    }
    // let x = (c1 * b2 - c2 * b1) / determinant;
    // let y = (a1 * c2 - a2 * c1) / determinant;
    let x = (machine.price_location_x * machine.b_y_increment
        - machine.price_location_y * machine.b_x_increment)
        / determinant;
    let y = (machine.a_x_increment * machine.price_location_y
        - machine.a_y_increment * machine.price_location_x)
        / determinant;

    if x * machine.a_x_increment + y * machine.b_x_increment != machine.price_location_x {
        return None;
    }
    if x * machine.a_y_increment + y * machine.b_y_increment != machine.price_location_y {
        return None;
    }

    Some((x, y))
}
