use std::cmp;
use std::fs;

const FPS: [&str; 2] = ["./data/test.txt", "./data/input.txt"];
type Board = Vec<usize>;
type Line = ((usize, usize), (usize, usize));

fn main() {
    for fp in FPS {
        let contents = fs::read_to_string(fp).expect("Something went wrong reading the file");
        let contents: Vec<&str> = contents.lines().collect();

        let mut lines1: Vec<Line> = Vec::new();
        let mut lines2: Vec<Line> = Vec::new();

        let board: Board;

        let mut num_rows = 0;
        let mut num_cols = 0;

        for line in contents {
            let pair: Vec<&str> = line.split(" -> ").collect();
            let point1: Vec<&str> = pair[0].split(",").collect();
            let point2: Vec<&str> = pair[1].split(",").collect();

            let line: Line = (
                (point1[0].parse().unwrap(), point1[1].parse().unwrap()),
                (point2[0].parse().unwrap(), point2[1].parse().unwrap()),
            );

            if line.0 .0 == line.1 .0 || line.0 .1 == line.1 .1 {
                lines1.push(line);
            } else {
                lines2.push(line);
            }
        }

        for line in &lines1 {
            num_rows = cmp::max(num_rows, cmp::max(line.0 .1, line.1 .1));
            num_cols = cmp::max(num_cols, cmp::max(line.0 .0, line.1 .0));
        }

        for line in &lines2 {
            num_rows = cmp::max(num_rows, cmp::max(line.0 .1, line.1 .1));
            num_cols = cmp::max(num_cols, cmp::max(line.0 .0, line.1 .0));
        }

        // Add one b/c 0 based indexing
        num_rows += 1;
        num_cols += 1;
        board = vec![0; num_rows * num_cols];

        let counts = do_work(lines1, lines2, board, num_cols);
        println!("Result 1 for file at path {} is {}", fp, counts.0);
        println!("Result 2 for file at path {} is {}", fp, counts.1);
    }
}

fn do_work(
    lines1: Vec<Line>,
    lines2: Vec<Line>,
    mut board: Board,
    num_cols: usize,
) -> (usize, usize) {
    let mut num_over_1: (usize, usize) = (0, 0);

    for line in lines1 {
        // Only one of these will be non-zero
        let x_dist = (line.1 .0 as i32 - line.0 .0 as i32).abs() as usize;
        let y_dist = (line.1 .1 as i32 - line.0 .1 as i32).abs() as usize;

        let x_mult: isize = if line.0 .0 > line.1 .0 { -1 } else { 1 };
        let y_mult: isize = if line.0 .1 > line.1 .1 { -1 } else { 1 };

        if x_dist > 0 {
            for x in 0..x_dist + 1 {
                board[((line.0 .0 as isize + x as isize * x_mult) * num_cols as isize
                    + line.0 .1 as isize) as usize] += 1;
            }
        } else if y_dist > 0 {
            for y in 0..y_dist + 1 {
                board[(line.0 .0 as isize * num_cols as isize
                    + line.0 .1 as isize
                    + y as isize * y_mult) as usize] += 1;
            }
        }
    }

    calc_board(&board, &mut num_over_1.0);

    for line in lines2 {
        // x and y dist are the same for diagonals, this calculates the x
        let dist = (line.1 .0 as i32 - line.0 .0 as i32).abs() as usize;

        let x_mult: isize = if line.0 .0 > line.1 .0 { -1 } else { 1 };
        let y_mult: isize = if line.0 .1 > line.1 .1 { -1 } else { 1 };

        for i in 0..dist + 1 {
            board[((line.0 .0 as isize + i as isize * x_mult) * num_cols as isize
                + line.0 .1 as isize
                + i as isize * y_mult) as usize] += 1;
        }
    }

    calc_board(&board, &mut num_over_1.1);

    num_over_1
}

fn calc_board(board: &[usize], dest: &mut usize) {
    for space in board {
        if *space > 1 {
            *dest += 1;
        }
    }
}
