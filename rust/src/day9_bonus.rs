mod utils;

const HIDDEN_NUMBER: i64 = 70639851;

fn main() {
    let lines = utils::read_input();
    let input = parse_input(&lines);

    let result = find_wrong_sum_range(&input);
    println!("{}", result);
}

fn parse_input(lines: &Vec<String>) -> Vec<i64> {
    lines
        .iter()
        .map(|line| line.parse::<i64>().unwrap())
        .collect()
}

fn find_wrong_sum_range(input: &Vec<i64>) -> i64 {
    let mut sum = 0;
    let mut start = 0;
    let mut end = 0;

    while end < input.len() {
        if sum < HIDDEN_NUMBER || start == end {
            sum += input[end];
            end += 1;
        } else if sum > HIDDEN_NUMBER {
            sum -= input[start];
            start += 1;
        } else {
            return input[start..end].iter().min().unwrap()
                + input[start..end].iter().max().unwrap();
        }
    }

    panic!("No range that sums to wrong number found.");
}
