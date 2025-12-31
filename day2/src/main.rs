use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());

    let levels: Vec<Vec<i32>> = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            line.split(' ')
                .filter_map(|s| s.parse::<i32>().ok())
                .collect()
        })
        .collect();

    let mut res = 0;

    for original in levels {
        let mut perm_ok = true;

        for i in 0..original.len() {
            perm_ok = true;
            let mut level = original.clone();
            level.remove(i);

            let mut increasing = None;

            for (cur, next) in level.iter().zip(level.iter().skip(1)) {
                if let None = increasing {
                    increasing = Some(cur < next);
                }

                let diff: i32 = if increasing == Some(true) {
                    next - cur
                } else {
                    cur - next
                };

                if diff > 3 || diff < 1 {
                    perm_ok = false;
                    break;
                }
            }

            // stop searching if we found a valid level
            if perm_ok {
                break;
            }
        }

        if perm_ok {
            res += 1;
        }
    }

    println!("{}", res);
}
