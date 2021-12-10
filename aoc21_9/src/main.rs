use std::fs;

const FPS: [&str; 2] = ["./data/test.txt", "./data/input.txt"];
const RADIX: u32 = 10;
const MAX_HEIGHT: usize = 9;
const CHECKED: usize = 10;

type Board = Vec<Vec<usize>>;

fn main() {
    for fp in FPS {
        let contents = fs::read_to_string(fp).expect("Something went wrong reading the file");
        let contents: Vec<&str> = contents.lines().collect();

        let mut board: Board = Vec::new();

        for line in contents {
            let mut row: Vec<usize> = Vec::new();

            for num in line.chars() {
                row.push(num.to_digit(RADIX).unwrap() as usize);
            }

            board.push(row);
        }

        let result = do_work1(board);
        let sum1 = result.0;
        let sum2 = result.1;

        println!("Result 1 for file at path {} is {}", fp, sum1);
        println!("Result 2 for file at path {} is {}", fp, sum2);
    }
}

fn do_work1(board: Board) -> (usize, usize) {
    let num_rows = board.len();
    let num_cols = board[0].len();

    let mut sum1 = 0;
    let mut basins: Vec<usize> = Vec::new();

    let mut working_board = board.clone();

    for (row_idx, row) in board.iter().enumerate() {
        for (col_idx, num) in row.iter().enumerate() {
            if row_idx > 0 && num >= &board[row_idx - 1][col_idx] {
                continue;
            }
            if row_idx < num_rows - 1 && num >= &board[row_idx + 1][col_idx] {
                continue;
            }
            if col_idx > 0 && num >= &board[row_idx][col_idx - 1] {
                continue;
            }
            if col_idx < num_cols - 1 && num >= &board[row_idx][col_idx + 1] {
                continue;
            }

            sum1 += num + 1;
            basins.push(calc_basin_size(
                &mut working_board,
                row_idx,
                col_idx,
                num_rows,
                num_cols,
            ));
        }
    }

    basins.sort_by(|a, b| b.cmp(a));
    (sum1, basins[0] * basins[1] * basins[2])
}

fn calc_basin_size(
    working_board: &mut Board,
    row_idx: usize,
    col_idx: usize,
    num_rows: usize,
    num_cols: usize,
) -> usize {
    let mut size = 1;
    working_board[row_idx][col_idx] = CHECKED;

    if col_idx > 0 && working_board[row_idx][col_idx - 1] < MAX_HEIGHT {
        size += calc_basin_size(working_board, row_idx, col_idx - 1, num_rows, num_cols)
    }
    if col_idx < num_cols - 1 && working_board[row_idx][col_idx + 1] < MAX_HEIGHT {
        size += calc_basin_size(working_board, row_idx, col_idx + 1, num_rows, num_cols)
    }
    if row_idx > 0 && working_board[row_idx - 1][col_idx] < MAX_HEIGHT {
        size += calc_basin_size(working_board, row_idx - 1, col_idx, num_rows, num_cols)
    }
    if row_idx < num_rows - 1 && working_board[row_idx + 1][col_idx] < MAX_HEIGHT {
        size += calc_basin_size(working_board, row_idx + 1, col_idx, num_rows, num_cols)
    }

    size
}
