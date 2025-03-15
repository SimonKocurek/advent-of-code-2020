use std::collections::{HashMap, VecDeque};

mod utils;

const CONTEXT_SIZE: i32 = 25;

fn main() {
    let lines = utils::read_input();
    let input = parse_input(&lines);

    let wrong = find_first_wrong(&input);
    println!("{}", wrong);
}

fn parse_input(lines: &Vec<String>) -> Vec<i64> {
    lines
        .iter()
        .map(|line| line.parse::<i64>().unwrap())
        .collect()
}

fn find_first_wrong(input: &Vec<i64>) -> i64 {
    let mut context = VecDeque::<i64>::with_capacity(CONTEXT_SIZE as usize);
    let mut nums = HashMap::<i64, i64>::new();
    let mut iter = input.iter();

    for _ in 0..CONTEXT_SIZE {
        let num = iter.next().unwrap();
        context.push_back(*num);
        *nums.entry(*num).or_insert(0) += 1;
    }

    while let Some(num) = iter.next() {
        if !has_two_sum(&mut context, &mut nums, num) {
            return *num;
        }

        let popped = context.pop_front().unwrap();
        *nums.entry(popped).or_insert(0) -= 1;

        context.push_back(*num);
        *nums.entry(*num).or_insert(0) += 1;
    }

    panic!("All numbers are correct.");
}

fn has_two_sum(context: &VecDeque<i64>, nums: &HashMap<i64, i64>, target: &i64) -> bool {
    for &first in context {
        let needed = target - first;
        let needed_count = if needed == first { 2 } else { 1 };

        if let Some(number_of_needed) = nums.get(&needed) {
            if number_of_needed >= &needed_count {
                return true;
            }
        }
    }

    false
}
