mod day1;
mod day2;
mod day3;

fn main() {
    println!(
        "day 1, distance: {:?}",
        day1::total_distance_between_lists()
    );
    println!("day 1, similarity: {:?}", day1::similarity_score());
    println!("day 2, safe reports: {:?}", day2::safe_reports_count());
    println!(
        "day 2, safe reports with problem dampener: {:?}",
        day2::safe_reports_count_with_problem_dampener()
    );
    println!(
        "day 3, result of multiplications: {:?}",
        day3::add_results_of_multiplications()
    );
}
