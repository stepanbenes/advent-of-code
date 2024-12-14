pub fn calculate_safety_factor() -> i32 {
    let mut robots = get_input();
    let space_width = 101;
    let space_height = 103;
    let seconds = 100;
    print_robots(&robots, space_width, space_height);
    println!("moving robots...");
    move_robots(&mut robots, seconds, space_width, space_height);
    print_robots(&robots, space_width, space_height);

    get_safety_factor(&robots, space_width, space_height)
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

fn print_robots(robots: &Vec<Robot>, space_width: i32, space_height: i32) {
    let mut space = vec![vec!['.'; space_width as usize]; space_height as usize];
    for robot in robots {
        space[robot.position_y as usize][robot.position_x as usize] = '#';
    }
    for row in space {
        println!("{:?}", row);
    }
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
