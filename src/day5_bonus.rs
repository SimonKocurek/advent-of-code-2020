mod utils;

const ROWS: i32 = 127;
const COLUMNS: i32 = 7;

fn main() {
    let lines = utils::read_input();

    let mut seat_taken = vec![vec![false; (COLUMNS + 1) as usize]; (ROWS + 1) as usize];

    lines.iter().for_each(|line| {
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

        seat_taken[row_start as usize][column_start as usize] = true;
    });

    for (row_num, row) in seat_taken.iter().enumerate() {
        for column in 1..row.len() - 1 {
            if row[column - 1] && !row[column] && row[column + 1] {
                println!("{}", row_num * 8 + column);
                return;
            }
        }
    }
}
