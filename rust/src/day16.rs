mod utils;

#[derive(Debug)]
struct Range {
    start: i32,
    end: i32,
}

#[derive(Debug)]
struct Tickets {
    line_ranges: Vec<Vec<Range>>,
    my_ticket: Vec<i32>,
    nearby_tickets: Vec<Vec<i32>>,
}

fn main() {
    let lines = utils::read_input();
    let tickets = parse_input(&lines);

    let result = error_rate(&tickets);

    println!("{}", result);
}

fn parse_input(lines: &Vec<String>) -> Tickets {
    let mut line_iter = lines.iter();

    let line_ranges = line_iter
        .by_ref()
        .take_while(|line| !line.is_empty()) // "class: 1-3 or 5-7"
        .map(|line| line.split_once(": ").unwrap().1) // "1-3 or 5-7"
        .map(|line| line.split(" or ")) // "1-3", "5-7"
        .map(|ranges| {
            ranges
                .map(|range| {
                    let range_nums = range
                        .split("-")
                        .map(|num| num.parse::<i32>().unwrap())
                        .take(2)
                        .collect::<Vec<i32>>();

                    Range {
                        start: range_nums[0],
                        end: range_nums[1],
                    }
                })
                .collect()
        })
        .collect();

    line_iter.next(); // "your ticket:"
    let my_ticket = line_iter
        .next()
        .unwrap()
        .split(",")
        .map(|num| num.parse::<i32>().unwrap())
        .collect();

    line_iter.next();
    line_iter.next(); // "nearby tickets:"

    let nearby_tickets = line_iter
        .map(|line| {
            line.split(",")
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    Tickets {
        line_ranges,
        my_ticket,
        nearby_tickets,
    }
}


fn error_rate(tickets: &Tickets) -> i32 {
    let all_ranges: Vec<&Range> = tickets.line_ranges.iter().flatten().collect();

    tickets
        .nearby_tickets
        .iter()
        .map(|nearby_ticket| {
            nearby_ticket
                .iter()
                .filter(|&&num| {
                    !all_ranges
                        .iter()
                        .any(|range| range.start <= num && num <= range.end)
                })
                .sum::<i32>()
        })
        .sum()
}
