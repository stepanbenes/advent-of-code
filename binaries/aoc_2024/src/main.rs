mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
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

        // println!(
        //     "day 15, sum of all boxes GPS coordinates: {:?}",
        //     day15::sum_of_all_boxes_gps_coordinates(false)
        // );

        println!(
            "day 15, sum of all boxes GPS coordinates on doubled map: {:?}",
            day15::sum_of_all_boxes_gps_coordinates(true)
        );

        let (score, count_of_tiles_on_best_paths) =
            day16::lowest_score_a_reindeer_could_possibly_get();

        println!(
            "day 16, lowest score a reindeer could possibly get: {score}, count of tiles on best paths: {count_of_tiles_on_best_paths}",
        );

        println!(
            "day 17, output of chronospacial computer program: {}",
            day17::run_chronospacial_computer_program()
        );

        println!(
            "day 17, lowest positive value of register A to print copy of itself: {}",
            day17::lowest_positive_value_of_register_a_to_print_copy_of_itself()
        );

        println!(
            "day 18, minimum steps to reach exit: {:?}",
            day18::minimum_steps_to_reach_exit()
        );

        let (x, y) = day18::get_first_byte_that_blocks_path();
        println!("day 18, first byte that blocks path: {x},{y}",);

        let (possible_designs, possible_designs_combinations) = day19::count_of_possible_designs();

        println!(
            "day 19, possible designs: {}, possible designs combinations: {}",
            possible_designs, possible_designs_combinations
        );

        println!(
            "day 20, count of cheats that save at least 100 picoseconds (cheat length = 2): {}",
            day20::count_of_cheats_that_save_at_least_100_picoseconds(2)
        );

        println!(
            "day 20, count of cheats that save at least 100 picoseconds (cheat length = 20): {}",
            day20::count_of_cheats_that_save_at_least_100_picoseconds(20)
        );

        println!(
            "day 21, sum of code complexities: {:?}",
            day21::sum_of_code_complexities()
        );

        println!(
            "day 22, sum of 2000th secret numbers: {}",
            day22::sum_of_2000th_secret_numbers()
        );

        println!(
            "day 22, most bananas you can get: {}",
            day22::most_bananas_you_can_get()
        );

        println!(
            "day 23, count of computers with name that starts with 't': {}",
            day23::count_of_computers_with_name_that_starts_with_t()
        );

        println!(
            "day 23, password to LAN party: {}",
            day23::password_to_lan_party()
        );

        println!(
            "day 24, decimal output on the wires starting with z: {}",
            day24::decimal_output_on_wires()
        );
    }

    println!(
        "day 25, count of unique lock pin pairs that fit together without overlapping: {}",
        day25::count_of_unique_lock_pin_pairs_that_fit_together_without_overlapping()
    );
}
