use std::collections::HashMap;

mod utils;

#[derive(Debug, Clone)]
enum Rule {
    Subrules(Vec<Vec<i32>>),
    Terminal(char),
}

#[derive(Debug, Clone)]
struct Input {
    rules: HashMap<i32, Rule>,
    lines: Vec<String>,
}

const STARTING_RULE: i32 = 0;

fn main() {
    let lines = utils::read_input();
    let input = parse_input(&lines);

    let updated_input = update_input(&input);

    let result = updated_input
        .lines
        .iter()
        .filter(|line| match_line(line, &updated_input.rules))
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

fn update_input(input: &Input) -> Input {
    let mut updated_rules = input.rules.clone();
    
    updated_rules.insert(8, Rule::Subrules(vec![vec![42], vec![42, 8]])).unwrap();
    updated_rules.insert(11, Rule::Subrules(vec![vec![42, 31], vec![42, 11, 31]])).unwrap();

    Input {
        rules: updated_rules,
        lines: input.lines.clone(),
    }
}

fn match_line(line: &str, rules: &HashMap<i32, Rule>) -> bool {
    match_subrule(line, STARTING_RULE, rules).iter().any(|remaining| remaining.is_empty())
}

fn match_subrule<'instr, 'rules>(
    substr: &'instr str,
    rule_id: i32,
    rules: &'rules HashMap<i32, Rule>,
) -> Vec<&'instr str> {
    match rules.get(&rule_id) {
        Some(Rule::Terminal(terminal)) => {
            match substr.chars().next() {
                Some(next_c) => if next_c == *terminal {
                    vec![&substr[1..]]
                } else {
                    vec![] // First character does not match terminal
                }
                None => vec![] // No characters left to match terminal
            }
        }

        Some(Rule::Subrules(subrules)) => {
            let mut result = vec![];

            for subrule_group in subrules {
                let mut remaining_options = vec![substr];
                let mut matched = true;

                for subrule_id in subrule_group {
                    let mut new_remaining_options = vec![];

                    remaining_options.iter().for_each(|remaining| {
                        new_remaining_options.extend(match_subrule(remaining, *subrule_id, rules));
                    });

                    remaining_options = new_remaining_options;
                    if remaining_options.is_empty() {
                        matched = false;
                        break;
                    }
                }

                if matched {
                    result.extend(remaining_options);
                }
            }

            result
        }

        None => vec![] // Rule with not found for ID
    }
}
