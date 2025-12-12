use solver::SolverBase;

pub struct Solver {
    columns_part1: Vec<Column>,
    columns_part2: Vec<Column>,
}

#[derive(Debug, Clone)]
pub struct Column {
    operator: Operator,
    operands: Vec<u64>,
}

#[derive(Debug, Copy, Clone)]
pub enum Operator {
    Add,
    Multiply,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        let mut operand_lines = Vec::<Vec<u64>>::new();
        let mut operator_line = Vec::<Operator>::new();

        for line in input.lines() {
            let tokens = line.split_whitespace().collect::<Vec<_>>();
            match tokens[0] {
                "+" | "*" => {
                    operator_line = tokens
                        .iter()
                        .map(|x| match *x {
                            "+" => Operator::Add,
                            "*" => Operator::Multiply,
                            _ => panic!(),
                        })
                        .collect();
                }
                _ => {
                    operand_lines.push(tokens.iter().map(|x| x.parse().unwrap()).collect());
                }
            }
        }

        let mut columns_part1 = Vec::<Column>::new();
        for column_index in 0..operator_line.len() {
            let column = Column {
                operator: operator_line[column_index],
                operands: operand_lines
                    .iter()
                    .map(|line| line[column_index])
                    .collect(),
            };
            columns_part1.push(column);
        }

        // part 2 input parsing

        let byte_lines = input
            .lines()
            .map(|line| line.as_bytes())
            .collect::<Vec<_>>();

        let column_count = byte_lines[0].len();
        let row_count = byte_lines.len();
        let operator_line_index = row_count - 1;

        let mut columns_part2 = Vec::<Column>::new();

        let mut current_column: Option<Column> = None;
        for column_index in 0..column_count {
            if let op_sign @ b'+' | op_sign @ b'*' = byte_lines[operator_line_index][column_index] {
                if let Some(current_column) = current_column.take() {
                    columns_part2.push(current_column);
                }
                current_column = Some(Column {
                    operator: match op_sign {
                        b'+' => Operator::Add,
                        b'*' => Operator::Multiply,
                        _ => unreachable!(),
                    },
                    operands: Vec::new(),
                });
            }
            let mut operand = 0;
            for row in byte_lines.iter().take(row_count - 1) {
                let symbol = row[column_index];
                let n = match symbol {
                    b'0' => 0,
                    b'1' => 1,
                    b'2' => 2,
                    b'3' => 3,
                    b'4' => 4,
                    b'5' => 5,
                    b'6' => 6,
                    b'7' => 7,
                    b'8' => 8,
                    b'9' => 9,
                    _ => continue,
                };
                operand = operand * 10 + n;
            }
            if operand != 0
                && let Some(mut column) = current_column.take()
            {
                column.operands.push(operand);
                current_column = Some(column);
            }
        }

        if let Some(current_column) = current_column.take() {
            columns_part2.push(current_column);
        }

        Solver {
            columns_part1,
            columns_part2,
        }
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let mut sum = 0;
        for column in &self.columns_part1 {
            let result = match column.operator {
                Operator::Add => column.operands.iter().sum::<u64>(),
                Operator::Multiply => column.operands.iter().product::<u64>(),
            };
            sum += result;
        }
        sum.to_string()
    }

    fn solve_part_two(&self) -> String {
        let mut sum = 0;
        for column in &self.columns_part2 {
            let result = match column.operator {
                Operator::Add => column.operands.iter().sum::<u64>(),
                Operator::Multiply => column.operands.iter().product::<u64>(),
            };
            sum += result;
        }
        sum.to_string()
    }

    fn day_number(&self) -> usize {
        6
    }

    fn description(&self) -> &'static str {
        "Trash Compactor"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ",
        )
        .solve_part_one();
        assert_eq!(result, "4277556");
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ",
        )
        .solve_part_two();
        assert_eq!(result, "3263827");
    }
}
