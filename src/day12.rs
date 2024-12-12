pub fn get_total_price_of_fencing() -> u32 {
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

fn visit_map(map: &mut Vec<Vec<Plot>>) -> u32 {
    let mut total_price = 0;
    for row_index in 0..map.len() {
        for column_index in 0..map[row_index].len() {
            let region = visit_plot(row_index, column_index, map);
            if region.area > 0 {
                let price_of_region = region.fence_count * region.area;
                total_price += price_of_region;
                println!(
                    "Region: {:?}, fence_count: {:?}, area: {:?}, price: {:?}",
                    map[row_index][column_index].name,
                    region.fence_count,
                    region.area,
                    price_of_region
                );
            }
        }
    }
    total_price
}

fn visit_plot(row_index: usize, column_index: usize, map: &mut Vec<Vec<Plot>>) -> Region {
    let name = map[row_index][column_index].name;
    if map[row_index][column_index].visited {
        return Region {
            fence_count: 0,
            area: 0,
        };
    }

    // visit current plot
    map[row_index][column_index].visited = true;

    // visit neighbors of same name
    let mut sum_region = Region {
        fence_count: 0,
        area: 1,
    };
    // left
    if column_index > 0 && map[row_index][column_index - 1].name == name {
        let left_region = visit_plot(row_index, column_index - 1, map);
        sum_region.fence_count += left_region.fence_count;
        sum_region.area += left_region.area;
    } else {
        sum_region.fence_count += 1;
    }
    // right
    if column_index < map[row_index].len() - 1 && map[row_index][column_index + 1].name == name {
        let right_region = visit_plot(row_index, column_index + 1, map);
        sum_region.fence_count += right_region.fence_count;
        sum_region.area += right_region.area;
    } else {
        sum_region.fence_count += 1;
    }
    // top
    if row_index > 0 && map[row_index - 1][column_index].name == name {
        let top_region = visit_plot(row_index - 1, column_index, map);
        sum_region.fence_count += top_region.fence_count;
        sum_region.area += top_region.area;
    } else {
        sum_region.fence_count += 1;
    }
    // bottom
    if row_index < map.len() - 1 && map[row_index + 1][column_index].name == name {
        let bottom_region = visit_plot(row_index + 1, column_index, map);
        sum_region.fence_count += bottom_region.fence_count;
        sum_region.area += bottom_region.area;
    } else {
        sum_region.fence_count += 1;
    }

    sum_region
}
