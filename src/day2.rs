use itertools::Itertools;

pub fn safe_reports_count() -> u32 {
    let levels = get_reports();
    let count = levels.iter().filter(|report| is_report_safe(report)).count() as u32;
    count
}

pub fn safe_reports_count_with_problem_dampener() -> u32 {
    let levels = get_reports();
    let count = levels.iter().filter(|report| get_report_variations(report).iter().any(|v| is_report_safe(v))).count() as u32;
    count
}

fn get_report_variations(report: &Vec<u32>) -> Vec<Vec<u32>> {
    // return all variations of the report with one number removed
    report.iter().enumerate().map(|(i, _)| {
        let mut variation = report.clone();
        variation.remove(i);
        variation
    }).collect()
}

fn get_reports() -> Vec<Vec<u32>> {
    // read input from file day1.txt
    let input = include_str!("../input/day2.txt");

    // split input by newlines
    let lines = input.lines();

    // convert each line to a u32
    let reports: Vec<Vec<u32>> = lines.map(|line| line.split_whitespace().map(|token| token.parse().unwrap()).collect()).collect();

    reports
}

#[derive(Debug, PartialEq)]
enum ReportBehaviour {
    Undetermined,
    Increasing,
    Decreasing,
    Constant,
}

fn is_report_safe(report: &Vec<u32>) -> bool {
    let mut behaviour = ReportBehaviour::Undetermined;
    let mut max_diff = 0;
    for (&a, &b) in report.iter().tuple_windows() {
        match behaviour {
            ReportBehaviour::Undetermined => {
                if a < b {
                    behaviour = ReportBehaviour::Increasing;
                } else if a > b {
                    behaviour = ReportBehaviour::Decreasing;
                } else {
                    behaviour = ReportBehaviour::Constant;
                }
            }
            ReportBehaviour::Increasing => {
                if a >= b {
                    return false;
                }
            }
            ReportBehaviour::Decreasing => {
                if a <= b {
                    return false;
                }
            }
            ReportBehaviour::Constant => {
                if a != b {
                    return false;
                }
            }
        }
        max_diff = max_diff.max(a.abs_diff(b));
    }
    (behaviour == ReportBehaviour::Increasing || behaviour == ReportBehaviour::Decreasing) && max_diff <= 3
}
