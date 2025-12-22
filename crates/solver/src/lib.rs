mod circular_linked_list;
mod union_find;

pub use union_find::*;

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
