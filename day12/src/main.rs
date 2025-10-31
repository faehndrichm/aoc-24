use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    time::SystemTime,
};

fn main() {
    let start = SystemTime::now();

    let reader = BufReader::new(File::open("day12/input.txt").unwrap());

    let field: Vec<Vec<char>> = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| line.chars().collect())
        .collect();

    let (res1, res2) = solve(&field);
    println!("Result 1: {}", res1);
    println!("Result 2: {}", res2);

    let duration = start.elapsed().unwrap();
    println!("Time elapsed: {:?}", duration);
}

fn solve(field: &Vec<Vec<char>>) -> (usize, usize) {
    let mut global_visited: HashSet<(usize, usize)> = HashSet::new();
    let mut regions: Vec<(char, usize, HashSet<(usize, usize)>)> = Vec::new();

    let field_rows = field.len() - 1;
    let field_cols = field[0].len() - 1;

    for (row_idx, row) in field.iter().enumerate() {
        for (col_idx, c) in row.iter().enumerate() {
            if global_visited.contains(&(row_idx, col_idx)) {
                continue;
            }

            let (region, perimeter) = fill_region(field, row_idx, col_idx, field_rows, field_cols);
            global_visited.extend(&region);
            regions.push((*c, perimeter, region));
        }
    }

    let mut res1 = 0;
    for (_, perimeter, region) in &regions {
        res1 += region.len() * perimeter;
    }

    let mut res2 = 0;
    for (c, _, region) in &regions {
        let sides = count_corners(region, field_rows, field_cols);
        //println!("{} {} {}", c, region.len(), sides);
        res2 += region.len() * sides;
    }

    (res1, res2)
}

fn count_corners(region: &HashSet<(usize, usize)>, field_rows: usize, field_cols: usize) -> usize {
    let mut corners = 0;

    // Thats the only case where a tile with 4 borders/edges exist
    if region.len() == 1 {
        return 4;
    }

    for &(row, col) in region.iter() {
        // horizontal & vertical
        let top = row != 0 && region.contains(&(row - 1, col));
        let bottom = row != field_rows && region.contains(&(row + 1, col));
        let left = col != 0 && region.contains(&(row, col - 1));
        let right = col != field_cols && region.contains(&(row, col + 1));

        let adj_count = [top, bottom, left, right].iter().filter(|&&x| x).count();

        // For 4 or 3 adjacent cells, there is no corner

        if adj_count == 2
            && ((top && right) || (top && left) || (bottom && right) || (bottom && left))
        {
            corners += 1;
        } else if adj_count == 1 {
            // in this case we have 2 corners no matter where the adjacent cells are
            corners += 2;
        }

        // diagonal
        let top_right = row != 0 && col != field_cols && region.contains(&(row - 1, col + 1));
        let bottom_right =
            row != field_rows && col != field_cols && region.contains(&(row + 1, col + 1));
        let top_left = row != 0 && col != 0 && region.contains(&(row - 1, col - 1));
        let bottom_left = row != field_rows && col != 0 && region.contains(&(row + 1, col - 1));

        // to catch inward corners we check if a cell has two adjacent cells,
        // but the diagonal cell is empty
        if top && right && !top_right {
            corners += 1;
        }
        if bottom && right && !bottom_right {
            corners += 1;
        }
        if top && left && !top_left {
            corners += 1;
        }
        if bottom && left && !bottom_left {
            corners += 1;
        }
    }

    corners
}

fn fill_region(
    field: &Vec<Vec<char>>,
    start_row: usize,
    start_col: usize,
    field_rows: usize,
    field_cols: usize,
) -> (HashSet<(usize, usize)>, usize) {
    let mut perimeter = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: Vec<(usize, usize)> = vec![(start_row, start_col)];

    let character = field[start_row][start_col];
    while let Some((row, col)) = queue.pop() {
        if visited.contains(&(row, col)) {
            continue;
        } else {
            visited.insert((row, col));
        }

        if row < field_rows && field[row + 1][col] == character {
            queue.push((row + 1, col));
        } else {
            perimeter += 1;
        }

        if row > 0 && field[row - 1][col] == character {
            queue.push((row - 1, col));
        } else {
            perimeter += 1;
        }

        if col < field_cols && field[row][col + 1] == character {
            queue.push((row, col + 1));
        } else {
            perimeter += 1;
        }

        if col > 0 && field[row][col - 1] == character {
            queue.push((row, col - 1));
        } else {
            perimeter += 1;
        }
    }

    (visited, perimeter)
}
