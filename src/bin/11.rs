use aoc2023::input_lines;

fn main() {
    part2()
}

pub fn part1() {
    println!("{}", calculate(2));
}

pub fn part2() {
    println!("{}", calculate(1000000));
}

fn calculate(expand_factor: usize) -> usize {
    let image = read();
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for (y, row) in image.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '#' {
                galaxies.push((x, y));
            }
        }
    }
    let empty_rows: Vec<usize> = image.iter().enumerate().filter(|(_y, row)| !row.contains(&'#')).map(|(y, _row)| y).collect();
    let empty_columns: Vec<usize> = transpose(image).into_iter().enumerate().filter(|(_x, row)| !row.contains(&'#')).map(|(x, _row)| x).collect();
    let expanded_galaxies: Vec<(usize, usize)> = galaxies.into_iter().map(|(x, y)| {
        let empty_rows_before = empty_rows.iter().filter(|r| r < &&y).count();
        let empty_columns_before = empty_columns.iter().filter(|c| c < &&x).count();
        (x + empty_columns_before * (expand_factor - 1), y + empty_rows_before * (expand_factor - 1))
    }).collect();
    let mut sum = 0;
    for i in 0..expanded_galaxies.len() - 1 {
        for j in i + 1..expanded_galaxies.len() {
            sum += distance(expanded_galaxies[i], expanded_galaxies[j]);
        }
    }
    sum
}

fn distance((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
    ((x1 as i64 - x2 as i64).abs() + (y1 as i64 - y2 as i64).abs()) as usize
}

fn read() -> Vec<Vec<char>> {
    let mut image: Vec<Vec<char>> = Vec::new();
    for line in input_lines(file!()) {
        image.push(line.chars().collect());
    }
    image
}

fn transpose(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let width = matrix[0].len();
    let mut transposed: Vec<Vec<char>> = Vec::new();
    let mut row_to_clone: Vec<char> = Vec::new();
    row_to_clone.resize(matrix.len(), 0 as char);
    transposed.resize(width, row_to_clone);
    for y in 0..matrix.len() {
        for x in 0..width {
            transposed[x][y] = matrix[y][x];
        }
    }
    transposed
}
