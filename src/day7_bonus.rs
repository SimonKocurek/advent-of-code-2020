use std::collections::HashMap;

mod utils;

const GOLD_BAG: &str = "shiny gold";

fn main() {
    let lines = utils::read_input();
    let rules = parse_input(&lines);
    let bags_inside = get_bags_inside_gold_bag(&rules);

    println!("{}", bags_inside);
}

fn parse_input(lines: &Vec<String>) -> HashMap<&str, Vec<(&str, i32)>> {
    lines
        .iter()
        .map(|line| {
            // e.g., light red bags contain 1 bright white bag, 2 muted yellow bags.
            let mut parts = line.split(" bags contain ");
            let bag = parts.next().unwrap(); // light red
            let contents = parts.next().unwrap(); // 1 bright white bag, 2 muted yellow bags.
            (bag, contents)
        })
        .map(|(bag, contents)| {
            let content_bags = parse_contents(contents);
            (bag, content_bags)
        })
        .collect::<HashMap<&str, Vec<(&str, i32)>>>()
}

fn parse_contents(contents: &str) -> Vec<(&str, i32)> {
    if contents == "no other bags." {
        return Vec::new();
    }

    contents
        .split(", ")
        .map(|content| {
            // e.g., 4 muted yellow bags.
            let mut parts = content.splitn(2, ' ');

            let number = parts.next().unwrap().parse::<i32>().unwrap(); // 4
            let bag = parts
                .next()
                .unwrap() // muted yellow bags.
                .rsplitn(2, ' ')
                .nth(1)
                .unwrap(); // muted yellow
            (bag, number)
        })
        .collect()
}

fn get_bags_inside_gold_bag<'input, 'rules>(
    rules: &'rules HashMap<&'input str, Vec<(&'input str, i32)>>
) -> i32 {
    let mut cache = HashMap::<&str, i32>::new();
    get_bags_inside(GOLD_BAG, rules, &mut cache)
}

fn get_bags_inside<'input, 'rules, 'cache>(
    bag: &'input str,
    rules: &'rules HashMap<&'input str, Vec<(&'input str, i32)>>,
    cache: &'cache mut HashMap<&'input str, i32>
) -> i32 {
    if let Some(cached) = cache.get(bag) {
        return cached.clone();
    }

    // Bags cannot be cyclically nested, that wouldn't be physically possible
    // Therefore, we can safely recurse without worrying about infinite loops/ stack overflows
    let mut result = 0;
    for &(inside_bag, count) in rules.get(bag).unwrap() {
        result += count + count * get_bags_inside(inside_bag, rules, cache);
    }
    cache.insert(bag, result);
    result
}
