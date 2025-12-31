use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());

    let board: Vec<Vec<char>> = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| line.chars().collect())
        .collect();

    let XMAS = ['X', 'M', 'A', 'S'];
    let XMAS_REV = ['S', 'A', 'M', 'X'];

    let mut count = 0;
    for (row_idx, row) in board.iter().enumerate() {
        for (col_idx, c) in row.iter().enumerate() {
            if *c != 'X' {
                //print!("0");
                continue;
            }

            let right_bound = col_idx + 4 <= row.len();
            let left_bound = col_idx >= 3;
            let bottom_bound = row_idx + 4 <= board.len();
            let top_bound = row_idx >= 3;

            let mut cur_count = 0;

            // check right
            if right_bound && board[row_idx][col_idx..col_idx + 4] == XMAS {
                cur_count += 1;
            }

            // check left
            if left_bound && board[row_idx][col_idx - 3..col_idx + 1] == XMAS_REV {
                cur_count += 1;
            }

            // check down
            if bottom_bound && vertical_slice(&board, row_idx..row_idx + 4, col_idx) == XMAS {
                cur_count += 1;
            }

            // check up
            if top_bound && vertical_slice(&board, row_idx - 3..row_idx + 1, col_idx) == XMAS_REV {
                cur_count += 1;
            }

            // check down-right
            if bottom_bound
                && right_bound
                && diagonal_slice(&board, row_idx..row_idx + 4, col_idx..col_idx + 4) == XMAS
            {
                cur_count += 1;
            }

            // check down-left
            if bottom_bound
                && left_bound
                && diagonal_slice(
                    &board,
                    row_idx..row_idx + 4,
                    (col_idx - 3..col_idx + 1).rev(),
                ) == XMAS
            {
                cur_count += 1;
            }

            // check up-right
            if top_bound
                && right_bound
                && diagonal_slice(
                    &board,
                    (row_idx - 3..row_idx + 1).rev(),
                    col_idx..col_idx + 4,
                ) == XMAS
            {
                cur_count += 1;
            }

            // check up-left
            if top_bound
                && left_bound
                && diagonal_slice(
                    &board,
                    (row_idx - 3..row_idx + 1).rev(),
                    (col_idx - 3..col_idx + 1).rev(),
                ) == XMAS
            {
                cur_count += 1;
            }
            //print!("{}", cur_count);
            count += cur_count;
        }
    }

    println!("{}", count);

    let mut count_2 = 0;
    for (row_idx, row) in board.iter().enumerate() {
        for (col_idx, c) in row.iter().enumerate() {
            if *c != 'A' {
                //print!("0");
                continue;
            }

            if col_idx == 0
                || col_idx == row.len() - 1
                || row_idx == 0
                || row_idx == board.len() - 1
            {
                continue;
            }

            let corners = [
                board[row_idx - 1][col_idx - 1],
                board[row_idx - 1][col_idx + 1],
                board[row_idx + 1][col_idx - 1],
                board[row_idx + 1][col_idx + 1],
            ];

            // check if cornes contains M two times and S two times
            if corners == ['M', 'M', 'S', 'S']
                || corners == ['M', 'S', 'M', 'S']
                || corners == ['S', 'M', 'S', 'M']
                || corners == ['S', 'S', 'M', 'M']
            {
                count_2 += 1;
            }
        }
    }

    println!("{}", count_2);
}

fn vertical_slice(board: &[Vec<char>], range: std::ops::Range<usize>, col: usize) -> Vec<char> {
    board[range].iter().map(|row| row[col]).collect()
}

fn diagonal_slice(
    board: &[Vec<char>],
    row_range: impl Iterator<Item = usize>,
    col_range: impl Iterator<Item = usize>,
) -> Vec<char> {
    row_range.zip(col_range).map(|(r, c)| board[r][c]).collect()
}
