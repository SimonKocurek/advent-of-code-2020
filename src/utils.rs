use std::fs::read_to_string;

pub fn read_input() -> Vec<String> {
    read_to_string("input")
        .expect("Failed to read input file")
        .lines()
        .map(String::from)
        .collect()
}
