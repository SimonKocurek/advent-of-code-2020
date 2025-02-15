mod utils;

fn main() {
    let lines = utils::read_input();
    let nums: Vec<i32> = lines
        .iter()
        .map(|line| line.parse().expect("Failed to parse number"))
        .collect();

    let target = 2020;

    println!("{}", two_sum(&nums, target));
}

fn two_sum(nums: &[i32], target: i32) -> i32 {
    let mut seen = std::collections::HashSet::new();

    for &num in nums {
        let needed = target - num;
        if seen.contains(&needed) {
            return num * needed;
        }
        seen.insert(num);
    }

    panic!("No solution found")
}
