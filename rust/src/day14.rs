use std::collections::HashMap;

mod utils;

#[derive(Debug)]
enum Operation {
    Mask(u64, u64),
    Write(u64, u64),
}

const UNINITIALIZED: u64 = u64::MAX;

fn main() {
    let lines = utils::read_input();
    let operations = parse_input(&lines);

    let mut memory: HashMap<u64, u64> = HashMap::new();
    run_program(&operations, &mut memory);

    let result: u64 = memory.values().sum();
    println!("{}", result);
}

fn parse_input(lines: &Vec<String>) -> Vec<Operation> {
    lines
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(" = ").collect();
            let op = parts[0];
            let value = parts[1];

            match op {
                "mask" => {
                    let positive_mask: u64 = value
                        .chars()
                        .enumerate()
                        .filter(|(_, c)| *c == '1')
                        .fold(0, |acc, (i, _)| acc | (1 << (35 - i)));

                    let negative_mask: u64 = value
                        .chars()
                        .enumerate()
                        .filter(|(_, c)| *c == '0')
                        .fold(u64::MAX, |acc, (i, _)| acc ^ (1 << (35 - i)));

                    Operation::Mask(positive_mask, negative_mask)
                },
                op if op.starts_with("mem") => {
                    let address = op
                        .trim_start_matches("mem[")
                        .trim_end_matches("]")
                        .parse()
                        .unwrap();

                    let value = parts[1].parse().unwrap();

                    Operation::Write(address, value)
                }

                _ => panic!("Invalid input: {}", line),
            }
        })
        .collect()
}

fn run_program(operations: &Vec<Operation>, memory: &mut HashMap<u64, u64>) {
    // Does not matter what value we initialize to.
    // The program (operations) should ensure to initialize before usage.
    let mut positive_mask = UNINITIALIZED;
    let mut negative_mask = UNINITIALIZED;

    for operation in operations {
        match *operation {
            Operation::Mask(new_positive_mask, new_negative_mask) => {
                positive_mask = new_positive_mask;
                negative_mask = new_negative_mask;
            }
            Operation::Write(i, value) => {
                memory.insert(i, (value | positive_mask) & negative_mask);
            }
        }
    }
}
