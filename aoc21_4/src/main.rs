use std::fs;

const FPS: [&str; 2] = ["./data/test.txt", "./data/input.txt"];
const BOARD_SIZE: usize = 25;
const BOARD_DIM: usize = 5;

#[derive(Clone, Copy)]
struct Board {
    nums: [u32; BOARD_SIZE],
    hits: [bool; BOARD_SIZE],
    won: bool,
}

fn new_board() -> Board {
    Board {
        nums: [0; BOARD_SIZE],
        hits: [false; BOARD_SIZE],
        won: false,
    }
}

fn main() {
    for fp in FPS {
        let contents = fs::read_to_string(fp).expect("Something went wrong reading the file");
        let mut contents = contents.lines();

        let draws: Vec<u32> = contents
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect();
        let mut boards: Vec<Board> = Vec::new();

        let mut fill_idx = 0;
        for line in contents {
            if line == "" {
                let board = new_board();
                boards.push(board);
                fill_idx = 0;

                continue;
            }

            let mut board = boards.last_mut().unwrap();
            let line: Vec<u32> = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();

            for num in line {
                board.nums[fill_idx] = num;
                fill_idx += 1;
            }
        }

        println!(
            "Result 1 for file at path {} is {}",
            fp,
            do_work_1(&draws, &boards)
        );
        println!(
            "Result 2 for file at path {} is {}",
            fp,
            do_work_2(&draws, &boards)
        );
    }
}

fn do_work_1(draws: &[u32], boards: &[Board]) -> u32 {
    let mut winning_draw: u32 = 0;
    let mut winning_board: Board = new_board();

    let mut working_boards = vec![new_board(); boards.len()];
    working_boards.clone_from_slice(boards);

    'outer: for draw in draws {
        for i in 0..boards.len() {
            for (j, num) in boards[i].nums.iter().enumerate() {
                if num == draw {
                    working_boards[i].hits[j] = true;

                    if has_won(working_boards[i]) {
                        winning_draw = *draw;
                        winning_board = working_boards[i];

                        break 'outer;
                    }
                }
            }
        }
    }

    calculate_score(winning_draw, winning_board)
}

fn do_work_2(draws: &[u32], boards: &[Board]) -> u32 {
    let mut last_draw: u32 = 0;
    let mut last_board: Board = new_board();
    let mut not_won = boards.len();

    let mut working_boards = vec![new_board(); boards.len()];
    working_boards.clone_from_slice(boards);

    'outer: for draw in draws {
        for i in 0..boards.len() {
            for (j, num) in boards[i].nums.iter().enumerate() {
                if num == draw {
                    working_boards[i].hits[j] = true;

                    if has_won(working_boards[i]) && !working_boards[i].won {
                        working_boards[i].won = true;
                        not_won -= 1;

                        if not_won == 0 {
                            last_draw = *draw;
                            last_board = working_boards[i];

                            break 'outer;
                        }
                    }
                }
            }
        }
    }

    calculate_score(last_draw, last_board)
}

fn has_won(board: Board) -> bool {
    return check_rows(&board) || check_cols(&board) || check_diagonals(&board);
}

fn check_rows(board: &Board) -> bool {
    for row in 0..BOARD_DIM {
        let mut hits = 0;

        for col in 0..BOARD_DIM {
            if board.hits[row * BOARD_DIM + col] {
                hits += 1
            }
        }

        if hits == BOARD_DIM {
            return true;
        }
    }

    return false;
}

fn check_cols(board: &Board) -> bool {
    for col in 0..BOARD_DIM {
        let mut hits = 0;

        for row in 0..BOARD_DIM {
            if board.hits[row * BOARD_DIM + col] {
                hits += 1
            }
        }

        if hits == BOARD_DIM {
            return true;
        }
    }

    return false;
}

fn check_diagonals(board: &Board) -> bool {
    for col in 0..BOARD_DIM {
        let row = col;
        let mut hits = 0;

        if board.hits[row * BOARD_DIM + col] {
            hits += 1
        }

        if hits == BOARD_DIM {
            return true;
        }
    }

    for col in 0..BOARD_DIM {
        let row = 4 - col;
        let mut hits = 0;

        if board.hits[row * BOARD_DIM + col] {
            hits += 1
        }

        if hits == BOARD_DIM {
            return true;
        }
    }

    return false;
}

fn calculate_score(winning_draw: u32, winning_board: Board) -> u32 {
    let mut score = 0;

    for i in 0..BOARD_SIZE {
        if !winning_board.hits[i] {
            score += winning_board.nums[i];
        }
    }

    score * winning_draw
}
