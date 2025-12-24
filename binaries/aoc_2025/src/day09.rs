use solver::SolverBase;

pub struct Solver {
    tile_positions: Vec<(u64, u64)>,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        let tile_positions = input
            .lines()
            .map(|line| line.split_once(',').unwrap())
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .collect();
        Solver { tile_positions }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Copy, Clone, Debug)]
struct Edge {
    a: Point,
    b: Point,
}

//
// Explicit boundary test
//
fn point_on_edge(p: Point, e: &Edge) -> bool {
    if e.a.x == e.b.x {
        // Vertical edge
        p.x == e.a.x && p.y >= e.a.y.min(e.b.y) && p.y <= e.a.y.max(e.b.y)
    } else {
        // Horizontal edge
        p.y == e.a.y && p.x >= e.a.x.min(e.b.x) && p.x <= e.a.x.max(e.b.x)
    }
}

//
// Point-in-polygon (boundary = inside)
// Ray cast to the right, orthogonal-safe
//
fn point_inside_polygon(p: Point, polygon: &[Edge]) -> bool {
    // Explicit boundary check
    for e in polygon {
        if point_on_edge(p, e) {
            return true;
        }
    }

    let mut crossings = 0;

    for e in polygon {
        // Only vertical edges participate in ray casting
        if e.a.x != e.b.x {
            continue;
        }

        let x = e.a.x;
        let y_min = e.a.y.min(e.b.y);
        let y_max = e.a.y.max(e.b.y);

        // Half-open interval avoids vertex double counting
        if p.y >= y_min && p.y < y_max && p.x < x {
            crossings += 1;
        }
    }

    crossings % 2 == 1
}

//
// Segment fully inside polygon
// Axis-aligned, boundary allowed
//
fn segment_inside_polygon(a: Point, b: Point, polygon: &[Edge]) -> bool {
    // Endpoints must be inside or on boundary
    if !point_inside_polygon(a, polygon) || !point_inside_polygon(b, polygon) {
        return false;
    }

    // Horizontal segment
    if a.y == b.y {
        let y = a.y;
        let x_min = a.x.min(b.x);
        let x_max = a.x.max(b.x);

        for e in polygon {
            // Only vertical edges can block a horizontal segment
            if e.a.x != e.b.x {
                continue;
            }

            let ex = e.a.x;
            let ey_min = e.a.y.min(e.b.y);
            let ey_max = e.a.y.max(e.b.y);

            // Proper crossing only
            if ex > x_min && ex < x_max && y >= ey_min && y < ey_max {
                return false;
            }
        }
    }
    // Vertical segment
    else if a.x == b.x {
        let x = a.x;
        let y_min = a.y.min(b.y);
        let y_max = a.y.max(b.y);

        for e in polygon {
            // Only horizontal edges can block a vertical segment
            if e.a.y != e.b.y {
                continue;
            }

            let ey = e.a.y;
            let ex_min = e.a.x.min(e.b.x);
            let ex_max = e.a.x.max(e.b.x);

            if ey > y_min && ey < y_max && x >= ex_min && x < ex_max {
                return false;
            }
        }
    } else {
        // Should never happen (non-axis-aligned)
        return false;
    }

    true
}

//
// Rectangle fully inside polygon
// Two opposite corners known to be on boundary
//
fn rectangle_inside_polygon(a: Point, b: Point, polygon: &[Edge]) -> bool {
    let x1 = a.x.min(b.x);
    let x2 = a.x.max(b.x);
    let y1 = a.y.min(b.y);
    let y2 = a.y.max(b.y);

    // 1. Interior parity test (minimal)
    let interior = Point {
        x: if x2 - x1 > 1 { x1 + 1 } else { x1 },
        y: if y2 - y1 > 1 { y1 + 1 } else { y1 },
    };

    if !point_inside_polygon(interior, polygon) {
        return false;
    }

    // Rectangle corners
    let bl = Point { x: x1, y: y1 };
    let br = Point { x: x2, y: y1 };
    let tr = Point { x: x2, y: y2 };
    let tl = Point { x: x1, y: y2 };

    // Rectangle edges
    let edges = [
        (bl, br), // bottom
        (tl, tr), // top
        (bl, tl), // left
        (br, tr), // right
    ];

    // 2. Edge containment tests
    for (p, q) in edges {
        if !segment_inside_polygon(p, q, polygon) {
            return false;
        }
    }

    true
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let mut max_area = None;
        for i in 0..self.tile_positions.len() {
            for j in i + 1..self.tile_positions.len() {
                let (a_x, a_y) = self.tile_positions[i];
                let (b_x, b_y) = self.tile_positions[j];
                let area = (a_x.abs_diff(b_x) + 1) * (a_y.abs_diff(b_y) + 1);
                if let Some(max) = max_area
                    && area > max
                {
                    max_area = Some(area);
                } else if max_area.is_none() {
                    max_area = Some(area);
                }
            }
        }
        max_area.unwrap().to_string()
    }

    fn solve_part_two(&self) -> String {
        let mut polygon: Vec<Edge> = Vec::new();
        let mut previous: (u64, u64) = self.tile_positions[self.tile_positions.len() - 1];
        for &(x, y) in &self.tile_positions {
            polygon.push(Edge {
                a: Point {
                    x: previous.0,
                    y: previous.1,
                },
                b: Point { x, y },
            });
            previous = (x, y);
        }

        let mut max_area = None;
        for i in 0..self.tile_positions.len() {
            for j in i + 1..self.tile_positions.len() {
                let (a_x, a_y) = self.tile_positions[i];
                let (b_x, b_y) = self.tile_positions[j];
                if rectangle_inside_polygon(
                    Point { x: a_x, y: a_y },
                    Point { x: b_x, y: b_y },
                    &polygon,
                ) {
                    let area = (a_x.abs_diff(b_x) + 1) * (a_y.abs_diff(b_y) + 1);
                    if let Some(max) = max_area
                        && area > max
                    {
                        max_area = Some(area);
                    } else if max_area.is_none() {
                        max_area = Some(area);
                    }
                }
            }
        }
        max_area.unwrap().to_string()
    }

    fn day_number(&self) -> usize {
        9
    }

    fn description(&self) -> &'static str {
        "Movie Theater"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3",
        )
        .solve_part_one();
        assert_eq!(result, "50");
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3",
        )
        .solve_part_two();
        assert_eq!(result, "24");
    }
}
