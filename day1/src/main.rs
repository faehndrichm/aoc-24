use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());

    let (mut first, mut second): (Vec<i32>, Vec<i32>) = reader
        .lines()
        .map_while(Result::ok) // unwrap lines safely
        .filter_map(|line| {
            line.split_once("   ")
                .and_then(|(a, b)| Some((a.parse::<i32>().ok()?, b.parse::<i32>().ok()?)))
        })
        .unzip();

    first.sort();
    second.sort();

    println!("{}", first.len());
    println!("{}", second.len());

    let sum: u32 = first
        .iter()
        .zip(second.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum();

    println!("{}", sum);

    let mut dict = std::collections::HashMap::new();

    for s in second {
        match dict.get(&s) {
            Some(v) => {
                dict.insert(s, v + 1);
            }
            None => {
                dict.insert(s, 1);
            }
        }
    }

    let sum2 = first
        .iter()
        .filter_map(|f| dict.get(&f).map(|v| v * f))
        .sum::<i32>();

    println!("{}", sum2);
}
