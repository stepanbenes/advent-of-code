mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    if cfg!(feature = "all_days") {
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
        println!(
            "day 3, result of conditional multiplications: {:?}",
            day3::add_results_of_conditional_multiplications()
        );
        println!(
            "day 4, count of XMAS appearances: {:?}",
            day4::count_of_xmas_appearances()
        );
        println!(
            "day 4, count of X-MAS appearances: {:?}",
            day4::count_of_x_mas_appearances()
        );
        println!(
            "day 5, sum of middle page numbers: {:?}",
            day5::sum_of_middle_page_numbers()
        );
        println!(
            "day 5, sum of middle page numbers of corrected updates: {:?}",
            day5::sum_of_middle_page_numbers_of_corrected_updates()
        );
        println!(
            "day 6, count of distinct positions in map: {:?}",
            day6::count_of_distinct_positions_in_map()
        );
        println!(
            "day 6, count of different positions for obstructions: {:?}",
            day6::count_of_different_positions_for_obstructions()
        );
        println!(
            "day 7, total calibration result: {:?}",
            day7::get_total_calibration_result(false)
        );
        println!(
            "day 7, total calibration result with concatenation: {:?}",
            day7::get_total_calibration_result(true)
        );
        println!(
            "day 8, count of antinodes: {:?}",
            day8::count_of_antinodes(false)
        );
        println!(
            "day 8, count of antinodes with resonant harmonics: {:?}",
            day8::count_of_antinodes(true)
        );
        println!(
            "day 9, filesystem checksum: {:?}",
            day9::get_filesystem_checksum()
        );
        println!(
            "day 9, filesystem checksum without fragmentation: {:?}",
            day9::get_filesystem_checksum_without_fragmentation()
        );
        let (score, rating) = day10::get_sum_of_all_trailhead_scores();
        println!("day 10, sum of all trailhead scores: {:?}", score);
        println!("day 10, sum of all trailhead ratings: {:?}", rating);
        println!(
            "day 11, count of stones after 25 blinks: {:?}",
            day11::count_of_stones::<25>()
        );
        println!(
            "day 11, count of stones after 75 blinks: {:?}",
            day11::count_of_stones::<75>()
        );
        let (total_segment_price, total_side_price) = day12::get_total_price_of_fencing();
        println!(
            "day 12, total price of segment fencing: {:?}",
            total_segment_price
        );
        println!(
            "day 12, total price of side fencing: {:?}",
            total_side_price
        );
        println!(
            "day 13, fewest tokens to win all possible prices: {:?}",
            day13::fewest_tokens_to_win_all_possible_prices(false)
        );
        println!(
            "day 13, fewest tokens to win all possible prices with units correction: {:?}",
            day13::fewest_tokens_to_win_all_possible_prices(true)
        );

        println!(
            "day 14, safety factor: {:?}",
            day14::calculate_safety_factor()
        );

        day14::show_easter_egg();
    }

    println!(
        "day 15, sum of all boxes GPS coordinates: {:?}",
        day15::sum_of_all_boxes_gps_coordinates(false)
    );

    println!(
        "day 15, sum of all boxes GPS coordinates on doubled map: {:?}",
        day15::sum_of_all_boxes_gps_coordinates(true)
    );
}
