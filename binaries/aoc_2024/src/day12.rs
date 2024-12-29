use itertools::Itertools;

pub fn get_total_price_of_fencing() -> (u32, u32) {
    let mut map = get_map();
    visit_map(&mut map)
}

struct Plot {
    name: char,
    visited: bool,
}

struct Region {
    fence_count: u32,
    area: u32,
    left_fences: Vec<(usize, usize)>,
    right_fences: Vec<(usize, usize)>,
    top_fences: Vec<(usize, usize)>,
    bottom_fences: Vec<(usize, usize)>,
}

fn get_map() -> Vec<Vec<Plot>> {
    //     let input = r"AAAA
    // BBCD
    // BBCC
    // EEEC";
    // let input = r"OOOOO
    // OXOXO
    // OOOOO
    // OXOXO
    // OOOOO";
    // let input = r"RRRRIICCFF
    // RRRRIICCCF
    // VVRRRCCFFF
    // VVRCCCJFFF
    // VVVVCJJCFE
    // VVIVCCJJEE
    // VVIIICJJEE
    // MIIIIIJJEE
    // MIIISIJEEE
    // MMMISSJEEE";
    //     let input = r"EEEEE
    // EXXXX
    // EEEEE
    // EXXXX
    // EEEEE";
    // let input = r"AAAAAA
    // AAABBA
    // AAABBA
    // ABBAAA
    // ABBAAA
    // AAAAAA";
    let input = include_str!("../input/day12.txt");
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Plot {
                    name: c,
                    visited: false,
                })
                .collect()
        })
        .collect()
}

fn visit_map(map: &mut Vec<Vec<Plot>>) -> (u32, u32) {
    let mut total_segment_price = 0;
    let mut total_side_price = 0;
    for row_index in 0..map.len() {
        for column_index in 0..map[row_index].len() {
            let region = visit_plot(row_index, column_index, map);
            if region.area > 0 {
                let segment_price_of_region = region.fence_count * region.area;
                total_segment_price += segment_price_of_region;
                // println!(
                //     "Region: {:?}, fence_count: {:?}, area: {:?}, price: {:?}",
                //     map[row_index][column_index].name,
                //     region.fence_count,
                //     region.area,
                //     segment_price_of_region,
                // );
                // calculate sides

                let left_side_count = get_side_count_vertical(region.left_fences);
                let left_side_price_of_region = left_side_count as u32 * region.area;
                total_side_price += left_side_price_of_region;

                let right_side_count = get_side_count_vertical(region.right_fences);
                let right_side_price_of_region = right_side_count as u32 * region.area;
                total_side_price += right_side_price_of_region;

                let top_side_count = get_side_count_horizontal(region.top_fences);
                let top_side_price_of_region = top_side_count as u32 * region.area;
                total_side_price += top_side_price_of_region;

                let bottom_side_count = get_side_count_horizontal(region.bottom_fences);
                let bottom_side_price_of_region = bottom_side_count as u32 * region.area;
                total_side_price += bottom_side_price_of_region;

                //println!("side prices: {:?}, {:?}, {:?}, {:?}", left_side_price_of_region, right_side_price_of_region, top_side_price_of_region, bottom_side_price_of_region);
            }
        }
    }
    (total_segment_price, total_side_price)
}

fn get_side_count_horizontal(mut fences: Vec<(usize, usize)>) -> usize {
    fences.sort_by(|a, b| a.0.cmp(&b.0));
    let mut sum_side_count = 0;
    for chunk in fences.into_iter().chunk_by(|a| a.0).into_iter() {
        let mut row_fences: Vec<(usize, usize)> = chunk.1.collect();
        row_fences.sort_by(|a, b| a.1.cmp(&b.1));
        let mut side_count = 1;
        let mut previous_column_index: Option<usize> = None;
        for (_, column_index) in row_fences {
            if let Some(previous_column_index) = previous_column_index {
                if column_index - previous_column_index > 1 {
                    side_count += 1;
                }
            }
            previous_column_index = Some(column_index);
        }
        sum_side_count += side_count;
    }
    sum_side_count
}

fn get_side_count_vertical(mut fences: Vec<(usize, usize)>) -> usize {
    fences.sort_by(|a, b| a.1.cmp(&b.1));
    let mut sum_side_count = 0;
    for chunk in fences.into_iter().chunk_by(|a| a.1).into_iter() {
        let mut column_fences: Vec<(usize, usize)> = chunk.1.collect();
        column_fences.sort_by(|a, b| a.0.cmp(&b.0));
        let mut side_count = 1;
        let mut previous_row_index: Option<usize> = None;
        for (row_index, _) in column_fences {
            if let Some(previous_column_index) = previous_row_index {
                if row_index - previous_column_index > 1 {
                    side_count += 1;
                }
            }
            previous_row_index = Some(row_index);
        }
        sum_side_count += side_count;
    }
    sum_side_count
}

fn visit_plot(row_index: usize, column_index: usize, map: &mut Vec<Vec<Plot>>) -> Region {
    let name = map[row_index][column_index].name;
    if map[row_index][column_index].visited {
        return Region {
            fence_count: 0,
            area: 0,
            left_fences: Vec::new(),
            right_fences: Vec::new(),
            top_fences: Vec::new(),
            bottom_fences: Vec::new(),
        };
    }

    // visit current plot
    map[row_index][column_index].visited = true;

    // visit neighbors of same name
    let mut sum_region = Region {
        fence_count: 0,
        area: 1,
        left_fences: Vec::new(),
        right_fences: Vec::new(),
        top_fences: Vec::new(),
        bottom_fences: Vec::new(),
    };
    // left
    if column_index > 0 && map[row_index][column_index - 1].name == name {
        let left_region = visit_plot(row_index, column_index - 1, map);
        sum_region.fence_count += left_region.fence_count;
        sum_region.area += left_region.area;
        sum_region.left_fences.extend(left_region.left_fences);
        sum_region.right_fences.extend(left_region.right_fences);
        sum_region.top_fences.extend(left_region.top_fences);
        sum_region.bottom_fences.extend(left_region.bottom_fences);
    } else {
        sum_region.fence_count += 1;
        sum_region.left_fences.push((row_index, column_index));
    }
    // right
    if column_index < map[row_index].len() - 1 && map[row_index][column_index + 1].name == name {
        let right_region = visit_plot(row_index, column_index + 1, map);
        sum_region.fence_count += right_region.fence_count;
        sum_region.area += right_region.area;
        sum_region.left_fences.extend(right_region.left_fences);
        sum_region.right_fences.extend(right_region.right_fences);
        sum_region.top_fences.extend(right_region.top_fences);
        sum_region.bottom_fences.extend(right_region.bottom_fences);
    } else {
        sum_region.fence_count += 1;
        sum_region.right_fences.push((row_index, column_index));
    }
    // top
    if row_index > 0 && map[row_index - 1][column_index].name == name {
        let top_region = visit_plot(row_index - 1, column_index, map);
        sum_region.fence_count += top_region.fence_count;
        sum_region.area += top_region.area;
        sum_region.left_fences.extend(top_region.left_fences);
        sum_region.right_fences.extend(top_region.right_fences);
        sum_region.top_fences.extend(top_region.top_fences);
        sum_region.bottom_fences.extend(top_region.bottom_fences);
    } else {
        sum_region.fence_count += 1;
        sum_region.top_fences.push((row_index, column_index));
    }
    // bottom
    if row_index < map.len() - 1 && map[row_index + 1][column_index].name == name {
        let bottom_region = visit_plot(row_index + 1, column_index, map);
        sum_region.fence_count += bottom_region.fence_count;
        sum_region.area += bottom_region.area;
        sum_region.left_fences.extend(bottom_region.left_fences);
        sum_region.right_fences.extend(bottom_region.right_fences);
        sum_region.top_fences.extend(bottom_region.top_fences);
        sum_region.bottom_fences.extend(bottom_region.bottom_fences);
    } else {
        sum_region.fence_count += 1;
        sum_region.bottom_fences.push((row_index, column_index));
    }

    sum_region
}
