use serde_json::Value;
use solver::Solver;

pub struct Day12Solver {
    numbers: Vec<f64>,
}

impl Day12Solver {
    pub fn new(input: &'static str) -> Self {
        let parsed_json: Value = serde_json::from_str(input).expect("Invalid JSON");
        let mut numbers = Vec::new();
        Day12Solver::collect_numbers(&parsed_json, &mut numbers);
        println!("{:?}", numbers);
        Day12Solver { numbers }
    }

    fn collect_numbers(json: &Value, numbers: &mut Vec<f64>) {
        match json {
            Value::Number(num) => {
                if let Some(n) = num.as_f64() {
                    numbers.push(n);
                }
            }
            Value::Array(arr) => {
                for item in arr {
                    Day12Solver::collect_numbers(item, numbers);
                }
            }
            Value::Object(obj) => {
                for (_, value) in obj {
                    Day12Solver::collect_numbers(value, numbers);
                }
            }
            _ => {}
        }
    }
}

impl Solver for Day12Solver {
    fn solve_part_one(&self) -> String {
        let sum = self.numbers.iter().sum::<f64>();
        (sum as i64).to_string()
    }

    fn solve_part_two(&self) -> String {
        "".to_string()
    }

    fn day_number(&self) -> usize {
        12
    }

    fn description(&self) -> &'static str {
        "JSON parsing"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Day12Solver::new(r#"[1,2,3]"#).solve_part_one();
        assert_eq!(result, "6");
    }

    #[test]
    fn test_2() {
        let result = Day12Solver::new(r#"{"a":2,"b":4}"#).solve_part_one();
        assert_eq!(result, "6");
    }

    #[test]
    fn test_3() {
        let result = Day12Solver::new(r#"[[[3]]]"#).solve_part_one();
        assert_eq!(result, "3");
    }

    #[test]
    fn test_4() {
        let result = Day12Solver::new(r#"{"a":{"b":4},"c":-1}"#).solve_part_one();
        assert_eq!(result, "3");
    }

    #[test]
    fn test_5() {
        let result = Day12Solver::new(r#"{"a":[-1,1]}"#).solve_part_one();
        assert_eq!(result, "0");
    }

    #[test]
    fn test_6() {
        let result = Day12Solver::new(r#"[-1,{"a":1}]"#).solve_part_one();
        assert_eq!(result, "0");
    }

    #[test]
    fn test_7() {
        let result = Day12Solver::new(r#"[]"#).solve_part_one();
        assert_eq!(result, "0");
    }

    #[test]
    fn test_8() {
        let result = Day12Solver::new(r#"{}"#).solve_part_one();
        assert_eq!(result, "0");
    }
}

// #[cfg(test)]
// mod part2_tests {
//     use super::*;

//     #[test]
//     fn test_1() {
//         let result = Day12Solver::new("abc").solve_part_two();
//         assert_eq!(result, "0");
//     }
// }
