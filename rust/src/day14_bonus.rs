use std::collections::HashMap;

mod utils;

#[derive(Debug)]
enum Operation {
    Mask(u64, Vec<usize>),
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

                    let floating_bits = value
                        .chars()
                        .enumerate()
                        .filter(|(_, c)| *c == 'X')
                        .map(|(i, _)| 35 - i)
                        .collect();

                    Operation::Mask(positive_mask, floating_bits)
                }
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
    let mut floating_bits = &vec![];

    for operation in operations {
        match operation {
            Operation::Mask(new_positive_mask, new_floating_bits) => {
                positive_mask = *new_positive_mask;
                floating_bits = new_floating_bits;
            }
            Operation::Write(i, value) => {
                apply_floating_bit(memory, floating_bits, 0, *i | positive_mask, *value);
            }
        }
    }
}

fn apply_floating_bit(
    memory: &mut HashMap<u64, u64>,
    floating_bits: &Vec<usize>,
    floating_bit_idx: usize,
    address: u64,
    value: u64,
) {
    if floating_bit_idx == floating_bits.len() {
        memory.insert(address, value);
        return;
    }

    let floating_bit = floating_bits[floating_bit_idx];

    apply_floating_bit(
        memory,
        floating_bits,
        floating_bit_idx + 1,
        address | (1 << floating_bit),
        value,
    );
    apply_floating_bit(
        memory,
        floating_bits,
        floating_bit_idx + 1,
        address & !(1 << floating_bit),
        value,
    );
}
