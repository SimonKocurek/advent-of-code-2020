mod utils;

fn main() {
    let lines = utils::read_input();

    let mut result = 1;
    for diff in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter() {
        let x_diff = diff.0;
        let y_diff = diff.1;
        result *= trees_on_slope(&lines, x_diff, y_diff);
    }

    println!("{}", result);
}

fn trees_on_slope(lines: &Vec<String>, x_diff: usize, y_diff: usize) -> usize {
    let mut result = 0;
    let mut x = 0;

    for y in (0..lines.len()).step_by(y_diff) {
        let line = &lines[y];

        if line.as_bytes()[x] == b'#' {
            result += 1;
        }

        x = (x + x_diff) % line.len();
    }

    result
}

