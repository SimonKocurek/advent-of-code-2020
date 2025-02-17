mod utils;

const ROWS: i32 = 127;
const COLUMNS: i32 = 7;

fn main() {
    let lines = utils::read_input();

    let result = lines
        .iter()
        .map(|line| {
            let mut row_start = 0;
            let mut row_end = ROWS;
            let mut column_start = 0;
            let mut column_end = COLUMNS;

            for row in line[0..7].chars() {
                let half = (row_end + row_start) / 2 + 1;
                match row {
                    'F' => row_end = half - 1,
                    'B' => row_start = half,
                    _ => panic!("Invalid row {}", row),
                }
            }

            for column in line[7..].chars() {
                let half = (column_end + column_start) / 2 + 1;
                match column {
                    'L' => column_end = half - 1,
                    'R' => column_start = half,
                    _ => panic!("Invalid column {}", column),
                }
            }

            row_start * 8 + column_start
        })
        .max()
        .unwrap();

    println!("{}", result);
}
