use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    time::SystemTime,
};

use rayon::prelude::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let start = SystemTime::now();

    let reader = BufReader::new(File::open("day6/input.txt").unwrap());

    let board: Vec<Vec<char>> = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| line.chars().collect())
        .collect();

    let (res, visited) = solve_1(&board);
    println!("{}", res);

    let res2 = solve_2(&board, &visited);
    println!("{}", res2);

    let duration = start.elapsed().unwrap();
    println!("Time elapsed: {:?}", duration);
}

fn solve_1(board: &Vec<Vec<char>>) -> (usize, HashSet<(usize, usize)>) {
    let (mut r, mut c) = get_start_position(&board);

    let mut dir = Direction::North; // assume starting direction is North
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut res = 0;
    while !((dir == Direction::North && r == 0)
        || (dir == Direction::South && r == board.len() - 1)
        || (dir == Direction::West && c == 0)
        || (dir == Direction::East && c == board[0].len() - 1))
    {
        if !visited.contains(&(r, c)) {
            visited.insert((r, c));
            res += 1;
        }

        let next_char = match dir {
            Direction::North => board[r - 1][c],
            Direction::South => board[r + 1][c],
            Direction::East => board[r][c + 1],
            Direction::West => board[r][c - 1],
        };

        if next_char == '#' {
            // turn right
            dir = match dir {
                Direction::North => Direction::East,
                Direction::South => Direction::West,
                Direction::East => Direction::South,
                Direction::West => Direction::North,
            }
        } else {
            // move forward
            match dir {
                Direction::North => {
                    r -= 1;
                }
                Direction::South => {
                    r += 1;
                }
                Direction::East => {
                    c += 1;
                }
                Direction::West => {
                    c -= 1;
                }
            }
        }
    }

    if !visited.contains(&(r, c)) {
        visited.insert((r, c));
        res += 1;
    }

    (res, visited)
}

fn solve_2(board: &Vec<Vec<char>>, visited: &HashSet<(usize, usize)>) -> usize {
    let (row_start, col_start) = get_start_position(&board);

    visited
        .par_iter() // parallel iterator
        .map(|(location_row, location_col)| {
            if find_loop(&board, *location_row, *location_col, row_start, col_start) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn find_loop(
    board: &Vec<Vec<char>>,
    obstacle_row: usize,
    obstacle_col: usize,
    row_start: usize,
    col_start: usize,
) -> bool {
    if obstacle_row == row_start && obstacle_col == col_start {
        return false;
    }

    let (mut r, mut c) = (row_start, col_start);
    let mut dir = Direction::North; // assume starting direction is North

    let mut is_loop = false;

    let mut visited: HashSet<(usize, usize, Direction)> = HashSet::new();

    while !((dir == Direction::North && r == 0)
        || (dir == Direction::South && r == board.len() - 1)
        || (dir == Direction::West && c == 0)
        || (dir == Direction::East && c == board[0].len() - 1))
    {
        let (next_r, next_c) = match dir {
            Direction::North => (r - 1, c),
            Direction::South => (r + 1, c),
            Direction::East => (r, c + 1),
            Direction::West => (r, c - 1),
        };

        let is_obstacle = board[next_r][next_c] == '#';

        let is_inserted_obstacle = next_r == obstacle_row && next_c == obstacle_col;

        if is_obstacle || is_inserted_obstacle {
            // check if we have visited this state before
            if visited.contains(&(next_r, next_c, dir)) {
                is_loop = true;
                break;
            }

            visited.insert((next_r, next_c, dir));

            // turn right
            dir = match dir {
                Direction::North => Direction::East,
                Direction::South => Direction::West,
                Direction::East => Direction::South,
                Direction::West => Direction::North,
            }
        } else {
            // move forward
            r = next_r;
            c = next_c;
        }
    }

    is_loop
}

fn get_start_position(board: &Vec<Vec<char>>) -> (usize, usize) {
    let mut start_row: usize = 0;
    let mut start_col: usize = 0;

    for (row_idx, row) in board.iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate() {
            if *col == '^' {
                start_row = row_idx;
                start_col = col_idx;
                break;
            }
        }
    }
    (start_row, start_col)
}
