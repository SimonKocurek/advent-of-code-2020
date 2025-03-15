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
        .map(|field| field.split(":").collect::<Vec<&str>>())
        .filter(|field| {
            let key = field[0];
            let value = field[1];

            match key {
                "byr" => value.parse().map_or(false, |x| 1920 <= x && x <= 2002),
                "iyr" => value.parse().map_or(false, |x| 2010 <= x && x <= 2020),
                "eyr" => value.parse().map_or(false, |x| 2020 <= x && x <= 2030),
                "hgt" => {
                    let height = value.chars()
                        .take(value.len() - 2)
                        .collect::<String>()
                        .parse()
                        .unwrap_or(0);

                    match &value[value.len() - 2..] {
                        "cm" => 150 <= height && height <= 193,
                        "in" => 59 <= height && height <= 76,
                        _ => false
                    }
                },
                "hcl" => value.chars().nth(0) == Some('#') && value.chars().skip(1).all(|c| c.is_digit(16)),
                "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value),
                "pid" => value.len() == 9 && value.chars().all(|c| c.is_digit(10)),
                "cid" => true,
                _ => false
            }
        })
        .map(|field| field[0])
        .collect()
}

fn valid_passport(field_names: &Vec<&str>) -> bool {
    REQUIRED_FIELDS.iter().all(|field| field_names.contains(field))
}
