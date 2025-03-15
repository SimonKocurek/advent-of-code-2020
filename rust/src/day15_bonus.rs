use std::collections::HashMap;

mod utils;

const FINAL_NUMBER: usize = 30_000_000;

fn main() {
    let lines = utils::read_input();
    let numbers = parse_input(&lines);

    let last_spoken = get_last_spoken_number(&numbers);

    println!("{}", last_spoken);
}

fn parse_input(lines: &Vec<String>) -> Vec<i32> {
    lines
        .iter()
        .flat_map(|line| line.split(','))
        .map(|x| x.parse().unwrap())
        .collect()
}

fn get_last_spoken_number(numbers: &Vec<i32>) -> i32 {
    let mut spoken_at: HashMap<i32, usize> = HashMap::new();

    // Initialize with input numbers
    numbers[0..numbers.len() - 1]
        .iter()
        .enumerate()
        .for_each(|(i, number)| {
            spoken_at.insert(*number, i);
        });

    // Iterate to the result
    let mut last_spoken = *numbers.last().unwrap();
    (numbers.len() - 1..FINAL_NUMBER - 1).for_each(|i| {
        let next_spoken = match spoken_at.get(&last_spoken) {
            Some(last_spoken_at) => i - last_spoken_at,
            None => 0,
        };

        spoken_at.insert(last_spoken, i);
        last_spoken = next_spoken as i32;
    });

    last_spoken
}
