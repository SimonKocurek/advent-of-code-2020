mod utils;

fn main() {
    let lines = utils::read_input();
    let instructions = parse_input(&lines);
    let acc = run_program(&instructions);

    println!("{}", acc);
}

fn run_program(instructions: &Vec<(&str, i32)>) -> i32 {
    let mut already_executed = vec![false; instructions.len()];

    let mut acc = 0;
    let mut i = 0;

    // The assignment asks us to prevent infinite loops by never running the same instruction twice
    while i < instructions.len() && !already_executed[i] {
        already_executed[i] = true;
        let (instruction, value) = instructions[i];

        match instruction {
            "acc" => {
                acc += value;
                i += 1;
            },
            "jmp" => {
                i = (i as i32 + value) as usize;
            },
            "nop" => {
                i += 1;
            },
            _ => panic!("Invalid instruction {}", instruction)
        }
    }

    acc
}

fn parse_input(lines: &Vec<String>) -> Vec<(&str, i32)> {
    lines
        .iter()
        .map(|line| {
            let mut parts = line.split(" ");

            let instruction = parts.next().unwrap();
            let value = parts.next().unwrap().parse::<i32>().unwrap();

            (instruction, value)
        })
        .collect()
}
