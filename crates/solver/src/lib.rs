mod circular_linked_list;

pub trait SolverBase {
    fn solve_part_one(&self) -> String {
        "".to_owned()
    }
    fn solve_part_two(&self) -> String {
        "".to_owned()
    }
    fn day_number(&self) -> usize;
    fn description(&self) -> &'static str {
        ""
    }
    fn skip_run(&self) -> bool {
        false
    }
}
