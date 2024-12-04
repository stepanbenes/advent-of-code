pub fn count_of_xmas_appearances() -> usize {
    const XMAS: &str = "XMAS";
    let input = read_input();
    //let input = read_test_input();

    let mut total_count = 0;

    // horizontal lines
    {
        let mut horizontal_lines = read_all_horizontal_lines(&input);
        let horizontal_lines_count: usize = horizontal_lines
            .iter()
            .map(|line| line.matches(XMAS).count())
            .sum();

        total_count += horizontal_lines_count;

        reverse_strings(&mut horizontal_lines);
        let horizontal_lines_reversed_count: usize = horizontal_lines
            .iter()
            .map(|line| line.matches(XMAS).count())
            .sum();

        total_count += horizontal_lines_reversed_count;
    }

    // vertical lines
    {
        let mut vertical_lines = read_all_vertical_lines(&input);
        let vertical_lines_count: usize = vertical_lines
            .iter()
            .map(|line| line.matches(XMAS).count())
            .sum();

        total_count += vertical_lines_count;

        reverse_strings(&mut vertical_lines);
        let vertical_lines_reversed_count: usize = vertical_lines
            .iter()
            .map(|line| line.matches(XMAS).count())
            .sum();

        total_count += vertical_lines_reversed_count;
    }

    // diagonal lines
    {
        let mut diagonal_lines = read_all_diagonal_lines(&input);

        let diagonal_lines_count: usize = diagonal_lines
            .iter()
            .map(|line| line.matches(XMAS).count())
            .sum();

        total_count += diagonal_lines_count;

        reverse_strings(&mut diagonal_lines);
        let diagonal_lines_reversed_count: usize = diagonal_lines
            .iter()
            .map(|line| line.matches(XMAS).count())
            .sum();

        total_count += diagonal_lines_reversed_count;
    }

    total_count
}

pub fn count_of_x_mas_appearances() -> usize {
    let input = read_input();
    //let input = read_test_input();

    let check_mas_fn = |i: usize, j: usize| {
        const MAS: &str = "MAS";
        if i == 0 || j == 0 {
            return false;
        }
        if i == input.len() - 1 || j == input[0].len() - 1 {
            return false;
        }
        let a1 = format!(
            "{}{}{}",
            input[i - 1][j - 1] as char,
            input[i][j] as char,
            input[i + 1][j + 1] as char
        );
        let a2 = format!(
            "{}{}{}",
            input[i + 1][j + 1] as char,
            input[i][j] as char,
            input[i - 1][j - 1] as char
        );
        let b1 = format!(
            "{}{}{}",
            input[i - 1][j + 1] as char,
            input[i][j] as char,
            input[i + 1][j - 1] as char
        );
        let b2 = format!(
            "{}{}{}",
            input[i + 1][j - 1] as char,
            input[i][j] as char,
            input[i - 1][j + 1] as char
        );
        (a1 == MAS || a2 == MAS) && (b1 == MAS || b2 == MAS)
    };

    let mut count = 0;
    for (r, line) in input.iter().enumerate() {
        for c in 0..line.len() {
            if check_mas_fn(r, c) {
                count += 1;
            }
        }
    }

    count
}

fn read_input() -> Vec<Vec<u8>> {
    let input = include_str!("../input/day4.txt");
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

fn _read_test_input() -> Vec<Vec<u8>> {
    let input = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

fn read_all_horizontal_lines(input: &Vec<Vec<u8>>) -> Vec<String> {
    let mut result = Vec::<String>::new();
    for line in input {
        let mut line_string = String::new();
        for c in line {
            line_string.push(*c as char);
        }
        result.push(line_string);
    }
    result
}

fn read_all_vertical_lines(input: &Vec<Vec<u8>>) -> Vec<String> {
    let mut result = Vec::<String>::new();
    for i in 0..input[0].len() {
        let mut line_string = String::new();
        for line in input {
            line_string.push(line[i] as char);
        }
        result.push(line_string);
    }
    result
}

fn reverse_strings(v: &mut Vec<String>) {
    for s in v {
        unsafe {
            // Access the underlying bytes of the string
            let bytes = s.as_mut_vec();

            // Reverse the bytes
            bytes.reverse();
        }
    }
}

fn read_all_diagonal_lines(matrix: &[Vec<u8>]) -> Vec<String> {
    let mut result = Vec::<String>::new();
    let rows = matrix.len();
    if rows == 0 {
        return result;
    }
    let cols = matrix[0].len();

    // Top-left to bottom-right diagonals
    for start_col in 0..cols {
        let mut row = 0;
        let mut col = start_col;
        let mut diagonal = String::new();
        while row < rows && col < cols {
            diagonal.push(matrix[row][col] as char);
            row += 1;
            col += 1;
        }
        result.push(diagonal);
        //println!("Diagonal from top-left: {:?}", diagonal);
    }

    for start_row in 1..rows {
        let mut row = start_row;
        let mut col = 0;
        let mut diagonal = String::new();
        while row < rows && col < cols {
            diagonal.push(matrix[row][col] as char);
            row += 1;
            col += 1;
        }
        result.push(diagonal);
        //println!("Diagonal from left: {:?}", diagonal);
    }

    // Top-right to bottom-left diagonals
    for start_col in (0..cols).rev() {
        let mut row = 0;
        let mut col = start_col;
        let mut diagonal = String::new();
        while row < rows && col < cols {
            diagonal.push(matrix[row][col] as char);
            row += 1;
            col = col.wrapping_sub(1); // Avoid underflow
        }
        result.push(diagonal);
        //println!("Diagonal from top-right: {:?}", diagonal);
    }

    for start_row in 1..rows {
        let mut row = start_row;
        let mut col = cols - 1;
        let mut diagonal = String::new();
        while row < rows && col < cols {
            diagonal.push(matrix[row][col] as char);
            row += 1;
            col = col.wrapping_sub(1); // Avoid underflow
        }
        result.push(diagonal);
        //println!("Diagonal from right: {:?}", diagonal);
    }

    result
}
