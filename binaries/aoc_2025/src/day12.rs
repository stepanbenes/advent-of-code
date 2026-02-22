use solver::SolverBase;

const SHAPE_WIDTH: usize = 3;
const SHAPE_LENGTH: usize = 3;

pub struct Solver {
    shapes: Vec<Shape>,
    regions: Vec<Region>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Shape {
    diagram: [[bool; SHAPE_WIDTH]; SHAPE_LENGTH],
}

impl Shape {
    pub fn rotate(&self) -> Shape {
        let mut result = self.clone();
        for i in 0..SHAPE_LENGTH {
            let new_i = SHAPE_LENGTH - 1 - i;
            for j in 0..SHAPE_WIDTH {
                result.diagram[j][new_i] = self.diagram[i][j];
            }
        }
        result
    }

    pub fn flip_horizontally(&self) -> Shape {
        let mut result = self.clone();
        for i in 0..SHAPE_LENGTH {
            for j in 0..SHAPE_WIDTH {
                let new_j = SHAPE_WIDTH - 1 - j;
                result.diagram[i][new_j] = self.diagram[i][j];
            }
        }
        result
    }

    pub fn flip_vertically(&self) -> Shape {
        let mut result = self.clone();
        for i in 0..SHAPE_LENGTH {
            let new_i = SHAPE_LENGTH - 1 - i;
            for j in 0..SHAPE_WIDTH {
                result.diagram[new_i][j] = self.diagram[i][j];
            }
        }
        result
    }
}

impl ToString for Shape {
    fn to_string(&self) -> String {
        let mut s = String::new();
        for i in 0..SHAPE_LENGTH {
            for j in 0..SHAPE_WIDTH {
                s.push(if self.diagram[i][j] { '#' } else { '.' });
            }
            s.push('\n');
        }
        s
    }
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

    #[test]
    fn test_flip_horizontally() {
        let solver = Solver::new(
            r"0:
###
##.
##.

",
        );
        let flipped_shape = solver.shapes[0].flip_horizontally();
        assert_eq!(
            flipped_shape.to_string(),
            r"###
.##
.##
"
        );
    }

    #[test]
    fn test_flip_vertically() {
        let solver = Solver::new(
            r"0:
###
##.
##.

",
        );
        let flipped_shape = solver.shapes[0].flip_vertically();
        assert_eq!(
            flipped_shape.to_string(),
            r"##.
##.
###
"
        );
    }

    #[test]
    fn test_rotate() {
        let solver = Solver::new(
            r"0:
###
##.
##.

",
        );
        let flipped_shape = solver.shapes[0].rotate();
        assert_eq!(
            flipped_shape.to_string(),
            r"###
###
..#
"
        );
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
