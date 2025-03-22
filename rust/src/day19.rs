use std::collections::HashMap;

mod utils;

#[derive(Debug)]
enum Rule {
    Subrules(Vec<Vec<i32>>),
    Terminal(char),
}

#[derive(Debug)]
struct Input {
    rules: HashMap<i32, Rule>,
    lines: Vec<String>,
}

const STARTING_RULE: i32 = 0;

fn main() {
    let lines = utils::read_input();
    let input = parse_input(&lines);

    let result = input
        .lines
        .iter()
        .filter(|line| match_line(line, &input.rules))
        .count();

    println!("{}", result);
}

fn parse_input(lines: &[String]) -> Input {
    let mut iter = lines.iter();

    let rules = iter
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (raw_rule_id, raw_rule_description) = line.split_once(": ").unwrap();
            let rule_id = raw_rule_id.parse::<i32>().unwrap();

            if raw_rule_description.contains('"') {
                let terminal_char = raw_rule_description
                    .chars()
                    .find(|c| c.is_alphabetic())
                    .unwrap();
                return (rule_id, Rule::Terminal(terminal_char));
            }

            let subrules = raw_rule_description
                .split(" | ")
                .map(|subrule| {
                    subrule
                        .split_whitespace()
                        .map(|subrule_id| subrule_id.parse().unwrap())
                        .collect()
                })
                .collect();

            (rule_id, Rule::Subrules(subrules))
        })
        .collect();

    let matched_lines = iter.map(|line| line.clone()).collect();

    Input {
        rules,
        lines: matched_lines,
    }
}

fn match_line(line: &str, rules: &HashMap<i32, Rule>) -> bool {
    match match_subrule(line, STARTING_RULE, rules) {
        Ok(remaining) => remaining.is_empty(),
        Err(_) => false,
    }
}

fn match_subrule<'instr, 'rules>(
    substr: &'instr str,
    rule_id: i32,
    rules: &'rules HashMap<i32, Rule>,
) -> Result<&'instr str, &'static str> {
    match rules.get(&rule_id) {
        Some(Rule::Terminal(terminal)) => {
            match substr.chars().next() {
                Some(next_c) => if next_c == *terminal {
                    Ok(&substr[1..])
                } else {
                    Err("First character does not match terminal")
                }
                None => Err("No characters left to match terminal")
            }
        }

        Some(Rule::Subrules(subrules)) => {
            for subrule_group in subrules {
                let mut remaining = substr;
                let mut matched = true;

                for subrule_id in subrule_group {
                    match match_subrule(remaining, *subrule_id, rules) {
                        Ok(new_remaining) => remaining = new_remaining,
                        Err(_) => {
                            matched = false;
                            break;
                        }
                    }
                }

                if matched {
                    return Ok(remaining);
                }
            }

            Err("No subrule group matched")
        }

        None => Err("Rule with not found for ID"),
    }
}
