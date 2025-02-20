use std::collections::HashMap;

mod utils;

const GOLD_BAG: &str = "shiny gold";

fn main() {
    let lines = utils::read_input();
    let rules = parse_input(&lines);
    let inside_bags = invert_bag_rules(&rules);
    let gold_bag_holders = get_gold_bag_holders(&inside_bags);

    println!("{}", gold_bag_holders.len());
}
fn parse_input(lines: &Vec<String>) -> Vec<(&str, Vec<&str>)> {
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
        .collect()
}

fn parse_contents(contents: &str) -> Vec<&str> {
    if contents == "no other bags." {
        return Vec::new();
    }

    contents
        .split(", ")
        .map(|content| {
            // e.g., 4 muted yellow bags.
            let mut parts = content.splitn(2, ' ');

            parts.next().unwrap(); // 4
            parts
                .next()
                .unwrap() // muted yellow bags.
                .rsplitn(2, ' ')
                .nth(1)
                .unwrap() // muted yellow
        })
        .collect()
}

fn invert_bag_rules<'a>(rules: &Vec<(&'a str, Vec<&'a str>)>) -> HashMap<&'a str, Vec<&'a str>> {
    let mut inside_bags = HashMap::<&'a str, Vec<&'a str>>::new();

    for (bag, bag_contents) in rules {
        for nested_bag in bag_contents {
            let inside = inside_bags.entry(nested_bag).or_insert(Vec::new());

            inside.push(bag);
        }
    }

    inside_bags
}

fn get_gold_bag_holders<'a>(inside_bags: &HashMap<&'a str, Vec<&'a str>>) -> Vec<&'a str> {
    let mut gold_bag_holders = inside_bags[GOLD_BAG].clone();

    let mut possible_holders = gold_bag_holders.clone();
    while !possible_holders.is_empty() {
        let mut new_holders = Vec::<&str>::new();

        for possible_holder in possible_holders {
            if let Some(new_possible_holders) = inside_bags.get(&possible_holder) {
                for new_possible_holder in new_possible_holders {
                    if gold_bag_holders.contains(new_possible_holder) {
                        continue; // Bag already counted
                    }
                    new_holders.push(new_possible_holder);
                    gold_bag_holders.push(new_possible_holder);
                }
            }
        }

        possible_holders = new_holders;
    }

    gold_bag_holders
}
