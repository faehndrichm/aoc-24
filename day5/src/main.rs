use std::{collections::HashSet, fs::File, io::BufRead, time::SystemTime};

fn main() {
    let start = SystemTime::now();

    let reader = std::io::BufReader::new(File::open("day5/input.txt").unwrap());

    let mut rules = Vec::new();
    let mut orders = Vec::new();
    let mut invalid_orders = Vec::new();
    let mut second_section = false;
    for line in reader.lines().map_while(Result::ok) {
        if line.is_empty() {
            second_section = true;
            continue;
        }

        if !second_section {
            let (val_a, val_b): (i32, i32) = line
                .split_once('|')
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                .unwrap();

            rules.push((val_a, val_b));
        } else {
            let order: Vec<i32> = line
                .split(',')
                .filter_map(|s| s.parse::<i32>().ok())
                .collect();
            orders.push(order);
        }
    }

    let mut res1 = 0;
    for order in orders.iter() {
        let mut order_valid = true;
        for (u, v) in rules.iter() {
            let u_index = order.iter().position(|x| x == u);
            let v_index = order.iter().position(|x| x == v);

            if let (Some(ui), Some(vi)) = (u_index, v_index) {
                if vi < ui {
                    order_valid = false;
                    break;
                }
            }
        }

        if order_valid {
            let val = order[order.len() / 2];
            res1 += val;
        } else {
            invalid_orders.push(order.clone());
        }
    }

    println!("{}", res1);

    let mut res2 = 0;

    for order in invalid_orders.iter() {
        let mut current_rules: Vec<(i32, i32)> = rules
            .iter()
            .filter(|(u, v)| order.contains(u) && order.contains(v))
            .map(|t| t.clone())
            .collect();

        let mut last_node = -1;
        let mut ordered = HashSet::new();
        for _ in 0..order.len() / 2 {
            // find sink node
            for node in order {
                if ordered.contains(node) {
                    continue;
                }
                let is_root = current_rules.iter().all(|(_, v)| *v != *node);
                if is_root {
                    ordered.insert(*node);
                    last_node = *node;
                    // remove all edges from the selected node
                    current_rules.retain(|(u, _)| *u != *node);
                    break;
                }
            }
        }
        res2 += last_node;
    }
    println!("{}", res2);

    let duration = start.elapsed().unwrap();
    println!("Time elapsed: {:?}", duration);
}
