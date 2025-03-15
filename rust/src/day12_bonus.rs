mod utils;

#[derive(Debug)]
enum Instruction {
    MoveWaypointNorth,
    MoveWaypointSouth,
    MoveWaypointEast,
    MoveWaypointWest,
    GoForward,
    TurnWaypointCounterClockwise,
    TurnWaypointClockwise,
}

fn main() {
    let lines = utils::read_input();
    let instructions = parse_input(&lines);

    let (x, y) = move_ship(&instructions);

    let manhattan_distance_from_start = x.abs() + y.abs();

    println!("{}", manhattan_distance_from_start);
}

fn parse_input(lines: &Vec<String>) -> Vec<(Instruction, i64)> {
    lines
        .iter()
        .map(|line| {
            let mut  chars = line.chars();

            let instruction = match chars.next().unwrap() {
                'N' => Instruction::MoveWaypointNorth,
                'S' => Instruction::MoveWaypointSouth,
                'E' => Instruction::MoveWaypointEast,
                'W' => Instruction::MoveWaypointWest,
                'F' => Instruction::GoForward,
                'L' => Instruction::TurnWaypointCounterClockwise,
                'R' => Instruction::TurnWaypointClockwise,
                _ => panic!("Unexpected instruction"),
            };

            let count = chars.collect::<String>().parse().unwrap();

            (instruction, count)
        })
        .collect()
}

fn move_ship(instructions: &Vec<(Instruction, i64)>) -> (i64, i64) {
    let mut ship_x = 0;
    let mut ship_y = 0;

    let mut waypoint_x = 10;
    let mut waypoint_y = -1;

    for (instruction, count) in instructions {
        match instruction {
            Instruction::MoveWaypointNorth => waypoint_y -= count,
            Instruction::MoveWaypointSouth => waypoint_y += count,
            Instruction::MoveWaypointEast => waypoint_x += count,
            Instruction::MoveWaypointWest => waypoint_x -= count,
            Instruction::GoForward => {
                ship_x += waypoint_x * count;
                ship_y += waypoint_y * count;
            },
            Instruction::TurnWaypointClockwise => {
                for _ in 0..count / 90 {
                    // Based on:
                    // x = x cos(pi/2) - y sin(pi/2) = -y
                    // y = x sin(pi/2) + y cos(pi/2) = x
                    (waypoint_x, waypoint_y) = (-waypoint_y, waypoint_x);
                }
            },
            Instruction::TurnWaypointCounterClockwise => {
                for _ in 0..count / 90 {
                    (waypoint_x, waypoint_y) = (waypoint_y, -waypoint_x);
                }
            },
        }
    }

    (ship_x, ship_y)
}
