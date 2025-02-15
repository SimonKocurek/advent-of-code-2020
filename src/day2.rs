mod utils;

fn main() {
    let lines = utils::read_input();
    let mut result = 0;

    for line in lines {
        let parts: Vec<&str> = line.split(&['-', ' ', ':'][..])
            .filter(|s| !s.is_empty())
            .collect();

        let min: usize = parts[0].parse().expect("Failed to parse min");
        let max: usize = parts[1].parse().expect("Failed to parse max");
        let char = parts[2].chars().next().expect("Failed to get char");
        let sequence = parts[3];

        let char_count = sequence.chars().filter(|&c| c == char).count();

        if min <= char_count && char_count <= max {
            result += 1;
        }
    }

    println!("{}", result);
}