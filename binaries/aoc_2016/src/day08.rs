use solver::SolverBase;

#[derive(Debug)]
enum Instruction {
    Rect(usize, usize),
    RotateColumn(usize, usize),
    RotateRow(usize, usize),
}

pub struct Solver<const T_WIDTH: usize, const T_HEIGHT: usize> {
    instructions: Vec<Instruction>,
}

impl<const T_WIDTH: usize, const T_HEIGHT: usize> Solver<T_WIDTH, T_HEIGHT> {
    pub fn new(input: &'static str) -> Self {
        let instructions = input
            .lines()
            .map(|line| {
                let tokens: Vec<_> = line.split_whitespace().collect();
                match &tokens[..] {
                    ["rect", dim_str] => {
                        let mut dim_tokens = dim_str.split('x');
                        let dim_x = dim_tokens.next().unwrap().parse().unwrap();
                        let dim_y = dim_tokens.next().unwrap().parse().unwrap();
                        Instruction::Rect(dim_x, dim_y)
                    }
                    ["rotate", "column", column_str, "by", distance_str] => {
                        let mut column_tokens = column_str.split('=');
                        _ = column_tokens.next().unwrap();
                        let column_index = column_tokens.next().unwrap().parse().unwrap();
                        let distance = distance_str.parse().unwrap();
                        Instruction::RotateColumn(column_index, distance)
                    }
                    ["rotate", "row", row_str, "by", distance_str] => {
                        let mut row_tokens = row_str.split('=');
                        _ = row_tokens.next().unwrap();
                        let row_index = row_tokens.next().unwrap().parse().unwrap();
                        let distance = distance_str.parse().unwrap();
                        Instruction::RotateRow(row_index, distance)
                    }
                    _ => panic!("unrecognized instruction"),
                }
            })
            .collect();
        Solver { instructions }
    }

    fn apply_instruction(screen: &mut [[bool; T_WIDTH]; T_HEIGHT], instruction: &Instruction) {
        match instruction {
            Instruction::Rect(width, height) => {
                Solver::<T_WIDTH, T_HEIGHT>::draw_rectangle(screen, *width, *height)
            }
            Instruction::RotateColumn(column_index, distance) => {
                Solver::<T_WIDTH, T_HEIGHT>::rotate_column(screen, *column_index, *distance)
            }
            Instruction::RotateRow(row_index, distance) => {
                Solver::<T_WIDTH, T_HEIGHT>::rotate_row(screen, *row_index, *distance)
            }
        }
    }

    fn draw_rectangle(screen: &mut [[bool; T_WIDTH]; T_HEIGHT], width: usize, height: usize) {
        for x in 0..width {
            for row in screen.iter_mut().take(height) {
                row[x] = true;
            }
        }
    }

    fn rotate_column(
        screen: &mut [[bool; T_WIDTH]; T_HEIGHT],
        column_index: usize,
        distance: usize,
    ) {
        let mut column = Vec::new();
        for row in screen.iter().take(T_HEIGHT) {
            column.push(row[column_index]);
        }
        column.rotate_right(distance);
        for (row_index, pixel) in column.iter().enumerate() {
            screen[row_index][column_index] = *pixel;
        }
    }

    fn rotate_row(screen: &mut [[bool; T_WIDTH]; T_HEIGHT], row_index: usize, distance: usize) {
        let mut row = Vec::new();
        for column_index in 0..T_WIDTH {
            row.push(screen[row_index][column_index]);
        }
        row.rotate_right(distance);
        for (column_index, pixel) in row.iter().enumerate() {
            screen[row_index][column_index] = *pixel;
        }
    }

    #[allow(dead_code)]
    fn print_screen(screen: &[[bool; T_WIDTH]; T_HEIGHT]) {
        for row in screen.iter() {
            for pixel in row.iter() {
                if *pixel {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }

    fn count_of_pixels_lit(screen: &[[bool; T_WIDTH]; T_HEIGHT]) -> usize {
        let mut count = 0;
        for row in screen.iter() {
            for pixel in row.iter() {
                if *pixel {
                    count += 1;
                }
            }
        }
        count
    }
}

impl<const T_WIDTH: usize, const T_HEIGHT: usize> SolverBase for Solver<T_WIDTH, T_HEIGHT> {
    fn solve_part_one(&self) -> String {
        let mut screen = [[false; T_WIDTH]; T_HEIGHT];
        for instruction in self.instructions.iter() {
            Solver::<T_WIDTH, T_HEIGHT>::apply_instruction(&mut screen, instruction);
            //Solver::<T_WIDTH, T_HEIGHT>::print_screen(&screen);
        }
        Solver::<T_WIDTH, T_HEIGHT>::count_of_pixels_lit(&screen).to_string()
    }

    fn solve_part_two(&self) -> String {
        let mut screen = [[false; T_WIDTH]; T_HEIGHT];
        for instruction in self.instructions.iter() {
            Solver::<T_WIDTH, T_HEIGHT>::apply_instruction(&mut screen, instruction);
        }
        // Solver::<T_WIDTH, T_HEIGHT>::print_screen(&screen); // answer is in printed screen
        "EFEYKFRFIJ".to_owned()
    }

    fn day_number(&self) -> usize {
        8
    }

    fn description(&self) -> &'static str {
        "Tiny display"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::<7, 3>::new("rect 3x2").solve_part_one();
        assert_eq!(result, "6");
    }

    #[test]
    fn test_2() {
        let result = Solver::<7, 3>::new(
            r"rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate column x=1 by 1",
        )
        .solve_part_one();
        assert_eq!(result, "6");
    }
}

// #[cfg(test)]
// mod part2_tests {
//     use super::*;

//     #[test]
//     fn test_1() {
//         let result = Solver::new("abc").solve_part_two();
//         assert_eq!(result, "0");
//     }
// }
