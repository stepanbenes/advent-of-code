mod day1;
mod day2;

fn main() {
    println!("day 1, distance: {:?}", day1::total_distance_between_lists());
    println!("day 1, similarity: {:?}", day1::similarity_score());
    println!("day 2, safe reports: {:?}", day2::safe_reports_count());
    println!("day 2, safe reports with problem dampener: {:?}", day2::safe_reports_count_with_problem_dampener());
}
