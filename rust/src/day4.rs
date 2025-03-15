mod utils;

const REQUIRED_FIELDS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn main() {
    let lines = utils::read_input();

    let mut result = 0;
    let mut field_names: Vec<&str> = Vec::new();

    lines.iter().for_each(|line| {
        if line.is_empty() {
            result += if valid_passport(&field_names) { 1 } else { 0 };
            field_names.clear(); // Empty line means we start collecting fields for a new passport
        } else {
            field_names.extend(get_field_names(line));
        }
    });
    result += if valid_passport(&field_names) { 1 } else { 0 };

    println!("{}", result);
}

fn get_field_names(line: &str) -> Vec<&str> {
    let fields: Vec<&str> = line.split_whitespace().collect();
    fields.iter()
        .map(|field| field.split(":").next().unwrap())
        .collect()
}

fn valid_passport(field_names: &Vec<&str>) -> bool {
    REQUIRED_FIELDS.iter().all(|field| field_names.contains(field))
}
