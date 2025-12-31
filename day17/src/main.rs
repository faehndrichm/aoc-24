use std::fs;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Operation {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

fn main() {
    let contents = fs::read_to_string("day17/input.txt").unwrap();
    let mut register_a = 0;
    let mut register_b = 0;
    let mut register_c = 0;
    let mut program: Vec<Operation> = Vec::new();

    for line in contents.lines() {
        if let Some(val) = line.strip_prefix("Register A: ") {
            register_a = val.trim().parse().unwrap();
        } else if let Some(val) = line.strip_prefix("Register B: ") {
            register_b = val.trim().parse().unwrap();
        } else if let Some(val) = line.strip_prefix("Register C: ") {
            register_c = val.trim().parse().unwrap();
        } else if let Some(val) = line.strip_prefix("Program: ") {
            program = val
                .split(',')
                .map(|x| match x.trim().parse::<u8>().unwrap() {
                    0 => Operation::Adv,
                    1 => Operation::Bxl,
                    2 => Operation::Bst,
                    3 => Operation::Jnz,
                    4 => Operation::Bxc,
                    5 => Operation::Out,
                    6 => Operation::Bdv,
                    7 => Operation::Cdv,
                    _ => panic!("Unknown opcode"),
                })
                .collect();
        }
    }

    let mut pointer: usize = 0;
    let base: u32 = 2;

    println!("Out: ");
    while pointer < program.len() - 1 {
        let opcode = program[pointer];
        let operand = program[pointer + 1];

        let combo = match operand {
            Operation::Adv => 0,
            Operation::Bxl => 1,
            Operation::Bst => 2,
            Operation::Jnz => 3,
            Operation::Bxc => register_a,
            Operation::Out => register_b,
            Operation::Bdv => register_c,
            Operation::Cdv => panic!("Reserver value used!"),
        };

        let literal = match opcode {
            Operation::Adv => 0,
            Operation::Bxl => 1,
            Operation::Bst => 2,
            Operation::Jnz => 3,
            Operation::Bxc => 4,
            Operation::Out => 5,
            Operation::Bdv => 6,
            Operation::Cdv => 7,
        };

        match opcode {
            Operation::Adv => {
                register_a = register_a / base.pow(combo);
            }
            Operation::Bxl => {
                register_b = register_b ^ literal;
            }
            Operation::Bst => {
                register_b = combo % 8;
            }
            Operation::Jnz => {
                if register_a != 0 {
                    pointer = literal as usize;
                    continue; // no increase
                }
            }
            Operation::Bxc => {
                register_b = register_b ^ register_c;
            }
            Operation::Out => {
                let out = combo % 8;
                print!("{},", out);
            }
            Operation::Bdv => {
                register_b = register_a / base.pow(combo);
            }
            Operation::Cdv => {
                register_c = register_a / base.pow(combo);
            }
        }

        pointer += 2;
    }
    println!("");
}
