mod utils;

fn main() {
    let lines = utils::read_input();
    let nums: Vec<i32> = lines
        .iter()
        .map(|line| line.parse().expect("Failed to parse number"))
        .collect();
    let target = 2020;

    println!("{}", three_sum(&nums, target));
}

fn three_sum(nums: &[i32], target: i32) -> i32 {
    let seen: std::collections::HashMap<i32, usize> = nums
        .iter()
        .enumerate()
        .map(|(i, &num)| (num, i))
        .collect();

    for (i, &num) in nums.iter().enumerate() {
        for j in (i + 1)..nums.len() {
            let num2 = nums[j];
            let num3 = target - num - num2;

            if let Some(&k) = seen.get(&num3) {
                if k != i && k != j {
                    return num * num2 * num3;
                }
            }
        }
    }

    panic!("No solution found")
}
