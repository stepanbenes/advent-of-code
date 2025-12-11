use solver::SolverBase;

pub struct Solver {
    columns: Vec<Column>,
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

        let mut columns = Vec::<Column>::new();
        for column_index in 0..operator_line.len() {
            let column = Column {
                operator: operator_line[column_index],
                operands: operand_lines
                    .iter()
                    .map(|line| line[column_index])
                    .collect(),
            };
            columns.push(column);
        }
        Solver { columns }
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let mut sum = 0;
        for column in &self.columns {
            let result = match column.operator {
                Operator::Add => column.operands.iter().sum::<u64>(),
                Operator::Multiply => column.operands.iter().product::<u64>(),
            };
            sum += result;
        }
        sum.to_string()
    }

    fn solve_part_two(&self) -> String {
        "".to_string()
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
