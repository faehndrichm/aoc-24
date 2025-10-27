use regex::Regex;
use std::fs::{self};

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let res1 = re
        .captures_iter(&contents)
        .map(|capture| {
            let a: i32 = capture[1].parse().unwrap();
            let b: i32 = capture[2].parse().unwrap();
            a * b
        })
        .sum::<i32>();

    println!("res1: {}", res1);

    let re2 = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();

    let mut res2: i32 = 0;

    let DO_STRING = String::from("do()");
    let DONT_STRING = String::from("don't()");

    let mut active = true;
    for capture in re2.captures_iter(&contents) {
        if &capture[0] == DO_STRING {
            active = true;
        } else if &capture[0] == DONT_STRING {
            active = false;
        } else if active {
            let a: i32 = capture[1].parse().unwrap();
            let b: i32 = capture[2].parse().unwrap();
            res2 += a * b;
        }
    }
    println!("res2: {}", res2);
}
