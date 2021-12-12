use std::fs;

const FPS: [&str; 2] = ["./data/test.txt", "./data/input.txt"];
const BOARD_DIM: usize = 10;
const NUM_STEPS: usize = 100;
const RADIX: u32 = 10;
type Board = [[(usize, bool); BOARD_DIM]; BOARD_DIM];

fn main() {
    for fp in FPS {
        let contents = fs::read_to_string(fp).expect("Something went wrong reading the files");
        let lines: Vec<&str> = contents.lines().collect();

        let mut board: Board = [[(0, false); BOARD_DIM]; BOARD_DIM];

        for (i, line) in lines.iter().enumerate() {
            for (j, c) in line.chars().enumerate() {
                board[i][j] = (c.to_digit(RADIX).unwrap() as usize, false);
            }
        }

        let results = do_work(&mut board);
        println!("Result 1 for file at path {} is {}", fp, results.0);
        println!("Result 2 for file at path {} is {}", fp, results.1);
    }
}

fn do_work(board: &mut Board) -> (usize, usize) {
    let mut flash_count = 0;
    let mut step = 1;

    let mut sync_step = 0;
    let mut step_found = false;

    while !step_found {
        let mut flashes_this_step = 0;

        for row in 0..BOARD_DIM {
            for col in 0..BOARD_DIM {
                let flashes = flash(board, row, col);
                flashes_this_step += flashes;

                if step <= NUM_STEPS {
                    flash_count += flashes;
                }
            }
        }

        if flashes_this_step == BOARD_DIM * BOARD_DIM && !step_found {
            sync_step = step;
            step_found = true;
        }

        for row in 0..BOARD_DIM {
            for col in 0..BOARD_DIM {
                if board[row][col].0 >= 9 && board[row][col].1 {
                    board[row][col] = (0, false);
                }
            }
        }

        step += 1;
    }

    (flash_count, sync_step)
}

fn flash(board: &mut Board, row: usize, col: usize) -> usize {
    let mut flash_count = 0;

    board[row][col].0 += 1;

    if board[row][col].0 > 9 && !board[row][col].1 {
        board[row][col].1 = true;

        flash_count += 1;

        if row > 0 {
            flash_count += flash(board, row - 1, col);
        }

        if row < BOARD_DIM - 1 {
            flash_count += flash(board, row + 1, col);
        }

        if col > 0 {
            flash_count += flash(board, row, col - 1);
        }

        if col < BOARD_DIM - 1 {
            flash_count += flash(board, row, col + 1);
        }

        if row > 0 && col > 0 {
            flash_count += flash(board, row - 1, col - 1);
        }

        if row > 0 && col < BOARD_DIM - 1 {
            flash_count += flash(board, row - 1, col + 1);
        }

        if row < BOARD_DIM - 1 && col < BOARD_DIM - 1 {
            flash_count += flash(board, row + 1, col + 1);
        }

        if row < BOARD_DIM - 1 && col > 0 {
            flash_count += flash(board, row + 1, col - 1);
        }
    }

    flash_count
}
