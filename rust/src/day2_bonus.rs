mod utils;

fn main() {
    let lines = utils::read_input();
    let mut result = 0;

    for line in lines {
        let parts: Vec<&str> = line.split(&['-', ' ', ':'][..])
            .filter(|s| !s.is_empty())
            .collect();

        let first: usize = parts[0].parse().expect("Failed to parse first");
        let second: usize = parts[1].parse().expect("Failed to parse second");
        let char = parts[2].bytes().next().expect("Failed to get char");
        let sequence = parts[3].as_bytes();

        if (sequence[first - 1] == char) ^ (sequence[second - 1] == char) {
            result += 1;
        }
    }

    println!("{}", result);
}
