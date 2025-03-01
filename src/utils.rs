use std::fs::read_to_string;

pub fn read_input() -> Vec<String> {
    read_to_string("input")
        .expect("Failed to read input file")
        .lines()
        .map(String::from)
        .collect()
}

pub const DIRECTIONS_8: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

pub const DIRECTIONS_4: [(i32, i32); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];

pub fn in_bounds(x: i32, y: i32, width: usize, height: usize) -> bool {
    x >= 0 && x < width as i32 && y >= 0 && y < height as i32
}
