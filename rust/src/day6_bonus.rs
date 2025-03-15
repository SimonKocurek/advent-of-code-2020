use std::collections::HashSet;

mod utils;

fn main() {
    let lines = utils::read_input();

    let mut result = 0;
    let mut answers = HashSet::<char>::new();
    let mut first_in_group = true;

    for line in lines {
        if line.is_empty() {
            result += answers.len();
            answers.clear();
            first_in_group = true;

        } else {
            if first_in_group {
                answers.extend(line.chars());
                first_in_group = false;
            } else {
                answers = line
                    .chars()
                    .filter(|c| answers.contains(c))
                    .collect();
            }
        }
    }
    result += answers.len();

    println!("{}", result);
}
