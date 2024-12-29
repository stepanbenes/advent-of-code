use solver::Solver;

struct Box {
    length: u32,
    width: u32,
    height: u32,
}

impl Box {
    fn get_sides(&self) -> Vec<u32> {
        let a = self.length * self.width;
        let b = self.length * self.height;
        let c = self.width * self.height;
        vec![a, b, c, a, b, c]
    }

    fn get_dimensions_sorted(&self) -> Vec<u32> {
        let mut dimensions = vec![
            self.length,
            self.width,
            self.height,
            self.length,
            self.width,
            self.height,
        ];
        dimensions.sort();
        dimensions
    }

    fn get_cubic_volume(&self) -> u32 {
        self.length * self.width * self.height
    }
}

pub struct Day02Solver {
    boxes: Vec<Box>,
}

impl Day02Solver {
    pub fn new(input: &'static str) -> Self {
        let boxes = input
            .lines()
            .map(|line| {
                let mut dimensions = line.split('x').map(|s| s.parse().unwrap());
                Box {
                    length: dimensions.next().unwrap(),
                    width: dimensions.next().unwrap(),
                    height: dimensions.next().unwrap(),
                }
            })
            .collect();
        Day02Solver { boxes }
    }
}

impl Solver for Day02Solver {
    fn solve_part_one(&self) -> String {
        let mut total_area = 0;
        for b in self.boxes.iter() {
            total_area += b.get_sides().iter().sum::<u32>() + b.get_sides().iter().min().unwrap();
        }
        total_area.to_string()
    }

    fn solve_part_two(&self) -> String {
        let mut total_length = 0;
        for b in self.boxes.iter() {
            total_length +=
                b.get_dimensions_sorted().iter().take(4).sum::<u32>() + b.get_cubic_volume();
        }
        total_length.to_string()
    }

    fn day_number(&self) -> usize {
        2
    }

    fn description(&self) -> &'static str {
        "Wrapping paper"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod part1 {
        use super::*;

        #[test]
        fn wrapping_paper_area_1() {
            let result = Day02Solver::new("2x3x4").solve_part_one();
            assert_eq!(result, "58");
        }

        #[test]
        fn wrapping_paper_area_2() {
            let result = Day02Solver::new("1x1x10").solve_part_one();
            assert_eq!(result, "43");
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn ribbon_1() {
            let result = Day02Solver::new("2x3x4").solve_part_two();
            assert_eq!(result, "34");
        }

        #[test]
        fn ribbon_2() {
            let result = Day02Solver::new("1x1x10").solve_part_two();
            assert_eq!(result, "14");
        }
    }
}
