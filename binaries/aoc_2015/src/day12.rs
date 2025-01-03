use serde_json::Value;
use solver::SolverBase;

pub struct Solver {
    json: Value,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        let parsed_json: Value = serde_json::from_str(input).expect("Invalid JSON");
        Solver { json: parsed_json }
    }

    fn collect_numbers(json: &Value, numbers: &mut Vec<f64>, filter: Option<&'static str>) {
        match json {
            Value::Number(num) => {
                if let Some(n) = num.as_f64() {
                    numbers.push(n);
                }
            }
            Value::Array(arr) => {
                for item in arr {
                    Solver::collect_numbers(item, numbers, filter);
                }
            }
            Value::Object(obj) => {
                if filter.is_none() || !obj.values().any(|value| value.as_str() == filter) {
                    for (_, value) in obj {
                        Solver::collect_numbers(value, numbers, filter);
                    }
                }
            }
            _ => {}
        }
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let mut numbers = Vec::new();
        Solver::collect_numbers(&self.json, &mut numbers, None);
        let sum = numbers.iter().sum::<f64>();
        (sum as i64).to_string()
    }

    fn solve_part_two(&self) -> String {
        let mut numbers = Vec::new();
        Solver::collect_numbers(&self.json, &mut numbers, Some("red"));
        let sum = numbers.iter().sum::<f64>();
        (sum as i64).to_string()
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
        let result = Solver::new(r#"[1,2,3]"#).solve_part_one();
        assert_eq!(result, "6");
    }

    #[test]
    fn test_2() {
        let result = Solver::new(r#"{"a":2,"b":4}"#).solve_part_one();
        assert_eq!(result, "6");
    }

    #[test]
    fn test_3() {
        let result = Solver::new(r#"[[[3]]]"#).solve_part_one();
        assert_eq!(result, "3");
    }

    #[test]
    fn test_4() {
        let result = Solver::new(r#"{"a":{"b":4},"c":-1}"#).solve_part_one();
        assert_eq!(result, "3");
    }

    #[test]
    fn test_5() {
        let result = Solver::new(r#"{"a":[-1,1]}"#).solve_part_one();
        assert_eq!(result, "0");
    }

    #[test]
    fn test_6() {
        let result = Solver::new(r#"[-1,{"a":1}]"#).solve_part_one();
        assert_eq!(result, "0");
    }

    #[test]
    fn test_7() {
        let result = Solver::new(r#"[]"#).solve_part_one();
        assert_eq!(result, "0");
    }

    #[test]
    fn test_8() {
        let result = Solver::new(r#"{}"#).solve_part_one();
        assert_eq!(result, "0");
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(r#"[1,2,3]"#).solve_part_two();
        assert_eq!(result, "6");
    }

    #[test]
    fn test_2() {
        let result = Solver::new(r#"[1,{"c":"red","b":2},3]"#).solve_part_two();
        assert_eq!(result, "4");
    }

    #[test]
    fn test_3() {
        let result = Solver::new(r#"[[[3]]]"#).solve_part_two();
        assert_eq!(result, "3");
    }

    #[test]
    fn test_4() {
        let result = Solver::new(r#"{"a":{"b":4},"c":-1}"#).solve_part_two();
        assert_eq!(result, "3");
    }

    #[test]
    fn test_5() {
        let result = Solver::new(r#"{"d":"red","e":[1,2,3,4],"f":5}"#).solve_part_two();
        assert_eq!(result, "0");
    }
}
