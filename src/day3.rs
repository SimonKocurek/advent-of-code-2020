mod utils;

fn main() {
    let lines = utils::read_input();

    let mut result = 0;
    let mut x = 0;

    lines.iter().for_each(|line| {
        if line.as_bytes()[x] == b'#' {
            result += 1;
        }

        x = (x + 3) % line.len();
    });

    println!("{}", result);
}
