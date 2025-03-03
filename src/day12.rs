use utils::DIRECTIONS_4;

mod utils;

#[derive(Debug)]
enum Instruction {
    GoNorth,
    GoSouth,
    GoEast,
    GoWest,
    GoForward,
    TurnLeft,
    TurnRight,
}

fn main() {
    let lines = utils::read_input();
    let instructions = parse_input(&lines);

    let (x, y) = move_ship(&instructions);

    let manhattan_distance_from_start = x.abs() + y.abs();

    println!("{}", manhattan_distance_from_start);
}

fn parse_input(lines: &Vec<String>) -> Vec<(Instruction, i32)> {
    lines
        .iter()
        .map(|line| {
            let mut  chars = line.chars();

            let instruction = match chars.next().unwrap() {
                'N' => Instruction::GoNorth,
                'S' => Instruction::GoSouth,
                'E' => Instruction::GoEast,
                'W' => Instruction::GoWest,
                'F' => Instruction::GoForward,
                'L' => Instruction::TurnLeft,
                'R' => Instruction::TurnRight,
                _ => panic!("Unexpected instruction"),
            };

            let count = chars.collect::<String>().parse().unwrap();

            (instruction, count)
        })
        .collect()
}


fn move_ship(instructions: &Vec<(Instruction, i32)>) -> (i32, i32) {
    let mut direction_idx = 1; // East in DIRECTIONS_4
    let mut x = 0;
    let mut y = 0;

    for (instruction, count) in instructions {
        match instruction {
            Instruction::GoNorth => y -= count,
            Instruction::GoSouth => y += count,
            Instruction::GoEast => x += count,
            Instruction::GoWest => x -= count,
            Instruction::GoForward => {
                let direction = DIRECTIONS_4[direction_idx];

                x += direction.0 * count;
                y += direction.1 * count;
            },
            Instruction::TurnLeft => {
                let new_direction = direction_idx as i32 - (count / 90);
                if new_direction >= 0 {
                    direction_idx = new_direction as usize;
                } else {
                    direction_idx = (DIRECTIONS_4.len() as i32 + new_direction) as usize;
                }
            },
            Instruction::TurnRight => {
                direction_idx = (direction_idx + (count / 90) as usize) % DIRECTIONS_4.len();
            },
        }
    }

    (x, y)
}
