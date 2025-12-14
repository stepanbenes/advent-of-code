use std::cmp::Ordering;

use solver::SolverBase;

pub struct Solver {
    points: Vec<Point>,
}

#[derive(Debug, Clone, Copy)]
pub struct Point(i32, i32, i32);

#[derive(Debug)]
pub struct PointPair<'a> {
    a: &'a Point,
    b: &'a Point,
    distance_squared: f64,
}

impl<'a> PointPair<'a> {
    pub fn get_distance_squared(a: &'a Point, b: &'a Point) -> f64 {
        let dx = b.0 as f64 - a.0 as f64;
        let dy = b.1 as f64 - a.1 as f64;
        let dz = b.2 as f64 - a.2 as f64;
        dx * dx + dy * dy + dz * dz
    }
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        let points = input
            .lines()
            .map(|line| {
                let mut tokens = line.split(',');
                let x = tokens.next().unwrap().parse().unwrap();
                let y = tokens.next().unwrap().parse().unwrap();
                let z = tokens.next().unwrap().parse().unwrap();
                Point(x, y, z)
            })
            .collect();
        Solver { points }
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let mut pairs: Vec<PointPair> = Vec::new();
        for i in 0..self.points.len() {
            for j in i + 1..self.points.len() {
                pairs.push(PointPair {
                    a: &self.points[i],
                    b: &self.points[j],
                    distance_squared: PointPair::get_distance_squared(
                        &self.points[i],
                        &self.points[j],
                    ),
                });
            }
        }

        fn comparer(a: &PointPair, b: &PointPair) -> Ordering {
            a.distance_squared.partial_cmp(&b.distance_squared).unwrap()
        }

        pairs.sort_by(comparer);
        let first = pairs.first().unwrap();
        format!("({:?}, {:?})", first.a, first.b)
    }

    fn solve_part_two(&self) -> String {
        "".to_string()
    }

    fn day_number(&self) -> usize {
        8
    }

    fn description(&self) -> &'static str {
        "Playground"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689",
        )
        .solve_part_one();
        assert_eq!(result, "40");
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
