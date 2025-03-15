mod utils;

fn main() {
    let lines = utils::read_input();
    let mut numbers = parse_input(&lines);
    numbers.sort();
    let diffs_of_size = get_adapter_diffs(&mut numbers);

    println!("{}", diffs_of_size[1] * diffs_of_size[3]);
}

fn parse_input(lines: &Vec<String>) -> Vec<i32> {
    lines
        .iter()
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}

fn get_adapter_diffs(numbers: &Vec<i32>) -> [i32; 4] {
    let mut diffs = [0, 0, 0, 1];
    diffs[numbers[0] as usize] += 1;

    for i in 1..numbers.len() {
        let diff = numbers[i] - numbers[i - 1];
        match diff {
            0 => continue,
            1..=3 => diffs[diff as usize] += 1,
            _ => panic!("Adapter chain not possible. Found diff {}", diff),
        }
    }

    diffs
}
