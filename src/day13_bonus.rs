mod utils;

fn main() {
    let lines = utils::read_input();
    let buses = parse_input(&lines);

    let departed_at = find_departure(&buses);

    println!("{}", departed_at);
}

fn parse_input(lines: &Vec<String>) -> Vec<(i64, i64)> {
    let mut line_iter = lines.iter();

    line_iter.next(); // The first line is not used in this problem

    let buses = line_iter
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|&(_, bus)| bus != "x")
        .map(|(i, bus)| (i as i64, bus.parse().unwrap()))
        .collect();

    buses
}

fn find_departure(buses: &Vec<(i64, i64)>) -> i64 {
    let mut depart_at = 0;
    let mut step = 1;

    for &(offset, bus) in buses {
        while (depart_at + offset) % bus != 0 {
            depart_at += step;
        }

        step = lcm(step, bus); // For some inputs, `step *= bus` would be sufficient.
    }

    depart_at
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
