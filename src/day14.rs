use crossterm::event;
use crossterm::{cursor, execute, style::Print, terminal};
use std::collections::HashMap;

pub fn calculate_safety_factor() -> i32 {
    let mut robots = get_input();
    let space_width = 11;
    let space_height = 7;
    let seconds = 100;
    print_robots(&robots, space_width, space_height, 0);
    println!("moving robots...");
    move_robots(&mut robots, seconds, space_width, space_height);
    print_robots(&robots, space_width, space_height, 0);

    get_safety_factor(&robots, space_width, space_height)
}

pub fn show_easter_egg() {
    let mut robots = get_input();
    let space_width = 101;
    let space_height = 103;
    let seconds = space_width * space_height;
    print_robots(&robots, space_width, space_height, 0);

    let mut safety_factors: Vec<(i32, i32)> = Vec::new();

    for s in 1..seconds {
        move_robots(&mut robots, 1, space_width, space_height);
        let safety_factor = get_safety_factor(&robots, space_width, space_height);
        safety_factors.push((s, safety_factor));
    }

    let mut robots = get_input();
    // sort by lowest safety factor
    safety_factors.sort_by(|a, b| a.1.cmp(&b.1));
    for (second, factor) in safety_factors {
        println!("second: {:?}, safety factor: {:?}", second, factor);
        move_robots(&mut robots, second, space_width, space_height);
        print_robots(&robots, space_width, space_height, second);

        loop {
            if event::poll(std::time::Duration::from_millis(500)).unwrap() {
                if let event::Event::Key(_key_event) = event::read().unwrap() {
                    _ = event::read().unwrap();
                    break; // Exit the loop on key press
                }
            }
        }
    }
}

#[derive(Debug)]
struct Robot {
    position_x: i32,
    position_y: i32,
    velocity_x: i32,
    velocity_y: i32,
}

fn get_input() -> Vec<Robot> {
    //     let input = r"p=0,4 v=3,-3
    // p=6,3 v=-1,-3
    // p=10,3 v=-1,2
    // p=2,0 v=2,-1
    // p=0,0 v=1,3
    // p=3,0 v=-2,-2
    // p=7,6 v=-1,-3
    // p=3,0 v=-1,-2
    // p=9,3 v=2,3
    // p=7,3 v=-1,2
    // p=2,4 v=2,-3
    // p=9,5 v=-3,-3";
    let input = include_str!("../input/day14.txt");
    let mut robots: Vec<Robot> = Vec::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(['=', ',', ' ']).collect();
        if let ["p", px, py, "v", vx, vy] = parts.as_slice() {
            robots.push(Robot {
                position_x: px.parse().unwrap(),
                position_y: py.parse().unwrap(),
                velocity_x: vx.parse().unwrap(),
                velocity_y: vy.parse().unwrap(),
            });
        }
    }
    robots
}

fn move_robots(robots: &mut Vec<Robot>, seconds: i32, space_width: i32, space_height: i32) {
    for robot in robots {
        let new_position_x = (robot.position_x + robot.velocity_x * seconds) % space_width;
        if new_position_x < 0 {
            robot.position_x = space_width + new_position_x;
        } else {
            robot.position_x = new_position_x;
        }

        let new_position_y = (robot.position_y + robot.velocity_y * seconds) % space_height;
        if new_position_y < 0 {
            robot.position_y = space_height + new_position_y;
        } else {
            robot.position_y = new_position_y;
        }
    }
}

fn print_robots(
    robots: &Vec<Robot>,
    space_width: i32,
    space_height: i32,
    current_second: i32,
) -> bool {
    // Set terminal to raw mode so we can capture key presses directly
    //terminal::enable_raw_mode().unwrap();

    //let (width, height) = terminal::size()?; // Get terminal dimensions

    let mut robot_positions: HashMap<(i32, i32), i32> = HashMap::new();

    let mut stdout = std::io::stdout();
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap(); // Clear the terminal

    for y in 0..space_height {
        execute!(
            stdout,
            cursor::MoveTo(0, y as u16),
            Print(".".repeat(space_width as usize))
        )
        .unwrap();
    }

    for robot in robots {
        let count = robot_positions
            .entry((robot.position_x, robot.position_y))
            .or_insert(0);
        *count += 1;
        execute!(
            stdout,
            cursor::MoveTo(robot.position_x as u16, robot.position_y as u16),
            if *count > 9 {
                Print("#".to_string())
            } else {
                Print((*count).to_string())
            }
        )
        .unwrap();

        execute!(
            stdout,
            cursor::MoveTo(0, space_height as u16),
            Print(format!("second: {current_second}"))
        )
        .unwrap();
    }

    let weird = robot_positions.values().any(|&x| x >= 6);
    weird
    // println!("Press any key to continue...");

    // // Wait for a key press
    // loop {
    //     if event::poll(std::time::Duration::from_millis(500)).unwrap() {
    //         if let event::Event::Key(key_event) = event::read().unwrap() {
    //             println!("You pressed: {:?}", key_event);
    //             break; // Exit the loop on key press
    //         }
    //     }
    // }

    // Reset the terminal to normal mode
    //terminal::disable_raw_mode().unwrap();
    //execute!(stdout, terminal::LeaveAlternateScreen).unwrap();

    // for y in 0..height {
    //     execute!(stdout, cursor::MoveTo(0, y), Print("#".repeat(width as usize)))?;
    // }
    // Ok(())

    // let mut space = vec![vec!['.'; space_width as usize]; space_height as usize];
    // for robot in robots {
    //     space[robot.position_y as usize][robot.position_x as usize] = '#';
    // }
    // for row in space {
    //     for cell in row {
    //         print!("{}", cell);
    //     }
    //     println!();
    // }
}

fn get_safety_factor(robots: &Vec<Robot>, space_width: i32, space_height: i32) -> i32 {
    let mut robot_counts = [0, 0, 0, 0];
    let half_x = space_width / 2;
    let half_y = space_height / 2;
    for robot in robots {
        if robot.position_x == half_x || robot.position_y == half_y {
            continue;
        }
        if robot.position_x < half_x {
            if robot.position_y < half_y {
                robot_counts[0] += 1;
            } else {
                robot_counts[2] += 1;
            }
        }
        if robot.position_x > half_x {
            if robot.position_y < half_y {
                robot_counts[1] += 1;
            } else {
                robot_counts[3] += 1;
            }
        }
    }
    robot_counts.iter().product()
}
