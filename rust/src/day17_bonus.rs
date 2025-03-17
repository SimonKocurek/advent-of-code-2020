use std::collections::{HashMap, HashSet};

mod utils;

const ITERATIONS: i32 = 6;

fn main() {
    let lines = utils::read_input();
    let enabled_cell_at_start = parse_input(&lines);

    let enabled_cells_at_end = run_cell_iterations(&enabled_cell_at_start);
    let result = enabled_cells_at_end.len();

    println!("{}", result);
}

fn parse_input(lines: &Vec<String>) -> HashSet<(i32, i32, i32, i32)> {
    lines
        .iter()
        .enumerate()
        .flat_map(|(ref y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(x, _)| (x as i32, *y as i32, 0, 0))
                .collect::<Vec<(i32, i32, i32, i32)>>()
        })
        .collect()
}

fn get_neighbour_count(enabled_cell: &HashSet<(i32, i32, i32, i32)>) -> HashMap<(i32, i32, i32, i32), i32> {
    let mut neighbour_count = HashMap::<(i32, i32, i32, i32), i32>::with_capacity(enabled_cell.len());

    for (x, y, z, w) in enabled_cell {
        for x_diff in -1..=1 {
            let new_x = x + x_diff;

            for y_diff in -1..=1 {
                let new_y = y + y_diff;

                for z_diff in -1..=1 {
                    let new_z = z + z_diff;

                    for w_diff in -1..=1 {
                        if x_diff == 0 && y_diff == 0 && z_diff == 0 && w_diff == 0 {
                            continue;
                        }

                        let new_w = w + w_diff;
    
                        neighbour_count
                            .entry((new_x, new_y, new_z, new_w))
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                    }
                }
            }
        }
    }

    neighbour_count
}

fn run_cell_iterations(enabled_cell_at_start: &HashSet<(i32, i32, i32, i32)>) -> HashSet<(i32, i32, i32, i32)> {
    let mut enabled_cell = enabled_cell_at_start.clone();

    for _ in 0..ITERATIONS {
        enabled_cell = get_neighbour_count(&enabled_cell)
            .iter()
            .filter_map(|(coordinates, &count)| {
                if (count == 2 || count == 3) && enabled_cell.contains(coordinates) {
                    return Some(coordinates.clone());
                }

                if (count == 3) && !enabled_cell.contains(coordinates) {
                    return Some(coordinates.clone());
                }

                None
            })
            .collect::<HashSet<(i32, i32, i32, i32)>>()
    }

    enabled_cell
}
