use std::collections::HashSet;

mod utils;

fn main() {
    let lines = utils::read_input();

    let mut result = 0;
    let mut answers = HashSet::<char>::new();

    for line in lines {
        if line.is_empty() {
            result += answers.len();
            answers.clear();
        } else {
            for c in line.chars() {
                answers.insert(c);
            }
        }
    }
    result += answers.len();

    println!("{}", result);
}
