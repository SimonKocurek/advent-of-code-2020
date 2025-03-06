mod utils;

fn main() {
    let lines = utils::read_input();
    let (earliest_departure_timestamp, buses) = parse_input(&lines);

    let (departed_at, bus) = find_departure(earliest_departure_timestamp, &buses);
    let wait_time = departed_at - earliest_departure_timestamp;

    println!("{}", wait_time * bus);
}

fn parse_input(lines: &Vec<String>) -> (i64, Vec<i64>) {
    let mut line_iter = lines.iter();

    let timestamp = line_iter.next().unwrap().parse().unwrap();

    let buses = line_iter
        .next()
        .unwrap()
        .split(',')
        .filter(|&x| x != "x")
        .map(|x| x.parse().unwrap())
        .collect();

    (timestamp, buses)
}

fn find_departure(earliest_departure_timestamp: i64, buses: &Vec<i64>) -> (i64, &i64) {
    let mut depart_at = earliest_departure_timestamp;

    loop {
        for bus in buses {
            if depart_at % bus == 0 {
                return (depart_at, bus);
            }
        }

        depart_at += 1;
    }
}
