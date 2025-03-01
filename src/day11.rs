use utils::{in_bounds, DIRECTIONS_8};

mod utils;

#[derive(Clone)]
enum Place {
    Floor,
    Empty,
    Occupied,
}

fn main() {
    let lines = utils::read_input();
    let map = parse_input(&lines);
    let final_map = get_map_with_seats_taken(&map);

    let occupied_seats = final_map.iter().flatten().filter(|&place| match place {
        Place::Occupied => true,
        _ => false,
    }).count();

    println!("{}", occupied_seats);
}

fn parse_input(lines: &Vec<String>) -> Vec<Vec<Place>> {
    lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'L' => Place::Empty,
                    '#' => Place::Occupied,
                    '.' => Place::Floor,
                    _ => panic!("Invalid character"),
                })
                .collect()
        })
        .collect()
}

fn get_map_with_seats_taken(initial_map: &Vec<Vec<Place>>) -> Vec<Vec<Place>> {
    let mut result = initial_map.clone();

    let mut changed = true;
    while changed {
        changed = false;

        result = result
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .map(|(x, place)| match get_place_update(&result, y, x, place) {
                        Some(new_place) => {
                            changed = true;
                            new_place
                        }
                        None => place.clone(),
                    })
                    .collect()
            })
            .collect();
    }

    result
}

fn get_place_update(map: &Vec<Vec<Place>>, y: usize, x: usize, place: &Place) -> Option<Place> {
    match place {
        // Empty place changes only if there are no neighbours
        Place::Empty => match get_occupied_neighbour_count(map, y, x) {
            0 => Some(Place::Occupied),
            _ => None,
        },

        // Occupied changes to empty if there are 4 or more neighbours
        Place::Occupied => match get_occupied_neighbour_count(map, y, x) {
            neighbours if neighbours >= 4 => Some(Place::Empty),
            _ => None,
        },

        // Floor never changes
        Place::Floor => None,
    }
}

fn get_occupied_neighbour_count(map: &Vec<Vec<Place>>, y: usize, x: usize) -> usize {
    DIRECTIONS_8
        .iter()
        .filter(|&(x_diff, y_diff)| {
            let new_x = x as i32 + x_diff;
            let new_y = y as i32 + y_diff;

            if !in_bounds(new_x, new_y, map.first().unwrap().len(), map.len()) {
                return false;
            }

            match map[new_y as usize][new_x as usize] {
                Place::Occupied => true,
                _ => false,
            }
        })
        .count()
}
