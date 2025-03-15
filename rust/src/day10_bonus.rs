mod utils;

fn main() {
    let lines = utils::read_input();
    let mut numbers = parse_input(&lines).to_vec();
    numbers.sort();
    numbers.push(numbers.last().unwrap() + 3);

    let ways_to_reach_last = get_ways_to_reach_last(&mut numbers);

    println!("{}", ways_to_reach_last)
}

fn parse_input(lines: &Vec<String>) -> Vec<i32> {
    lines
        .iter()
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}

fn get_ways_to_reach_last(numbers: &Vec<i32>) -> i64 {
    let mut ways_to_reach: Vec<i64> = vec![0; numbers.len()];

    for (i, &num) in numbers.iter().enumerate() {
        if num <= 3 {
            ways_to_reach[i] += 1;
        }

        for j in (0..i).rev() {
            if numbers[j] < num - 3 {
                break;
            }
            ways_to_reach[i] += ways_to_reach[j];
        }
    }

    ways_to_reach.last().unwrap().clone()
}
