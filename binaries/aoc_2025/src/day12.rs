use solver::SolverBase;

const SHAPE_WIDTH: usize = 3;
const SHAPE_LENGTH: usize = 3;

pub struct Solver {
    shapes: Vec<Shape>,
    regions: Vec<Region>,
}

#[derive(Debug)]
struct Shape {
    diagram: [[bool; SHAPE_WIDTH]; SHAPE_LENGTH],
}

#[derive(Debug)]
struct Region {
    width: usize,
    length: usize,
    presents: Vec<usize>,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        let mut shapes = Vec::new();
        let mut regions = Vec::new();
        let mut current_shape_index: Option<usize> = None;
        let mut current_shape = Vec::<[bool; SHAPE_WIDTH]>::new();
        for line in input.lines() {
            if line.is_empty() {
                if !current_shape.is_empty() {
                    let mut diagram = [[false; SHAPE_WIDTH]; SHAPE_LENGTH];
                    assert!(current_shape.len() == SHAPE_LENGTH);
                    for (i, row) in current_shape.iter().enumerate() {
                        diagram[i][..SHAPE_WIDTH].copy_from_slice(&row[..SHAPE_WIDTH]);
                    }
                    assert!(current_shape_index.is_some_and(|index| index == shapes.len()));
                    shapes.push(Shape { diagram });
                }
                continue;
            }
            if let Some((head, tail)) = line.split_once(':') {
                // present region
                if let Some((width, length)) = head.split_once('x') {
                    let width: usize = width.parse().unwrap();
                    let length: usize = length.parse().unwrap();
                    let presents: Vec<usize> = tail
                        .split_whitespace()
                        .map(|token| token.parse().unwrap())
                        .collect();
                    regions.push(Region {
                        width,
                        length,
                        presents,
                    });
                }
                // shape header
                else {
                    current_shape_index = Some(head.parse().unwrap());
                    current_shape.clear();
                }
            }
            // shape diagram
            else {
                let shape_diagram_row_vec: Vec<bool> = line
                    .chars()
                    .map(|c| match c {
                        '#' => true,
                        '.' => false,
                        c => panic!("unexpected shape diagram symbol '{}'", c),
                    })
                    .collect();
                assert!(shape_diagram_row_vec.len() == SHAPE_WIDTH);
                let mut shape_diagram_row: [bool; SHAPE_WIDTH] = [false; SHAPE_WIDTH];
                for (i, value) in shape_diagram_row_vec.iter().enumerate() {
                    shape_diagram_row[i] = *value;
                }
                current_shape.push(shape_diagram_row);
            }
        }
        Solver { shapes, regions }
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        println!("shapes: {:?}", self.shapes);
        println!("regions: {:?}", self.regions);
        "".to_string()
    }

    fn solve_part_two(&self) -> String {
        "".to_string()
    }

    fn day_number(&self) -> usize {
        12
    }

    fn description(&self) -> &'static str {
        "Christmas Tree Farm"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2",
        )
        .solve_part_one();
        assert_eq!(result, "2");
    }
}

// #[cfg(test)]
// mod part2_tests {
//     use super::*;

//     #[test]
//     fn test_1() {
//         let result = Solver::new("abc").solve_part_two();
//         assert_eq!(result, "0");
//     }
// }
