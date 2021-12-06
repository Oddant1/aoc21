use std::cmp;
use std::fs;

const FPS: [&str; 2] = ["./data/test.txt", "./data/input.txt"];
type Board = Vec<usize>;
type Line = ((usize, usize), (usize, usize));

fn main() {
    for fp in FPS {
        let contents = fs::read_to_string(fp).expect("Something went wrong reading the file");
        let contents: Vec<&str> = contents.lines().collect();

        let mut lines: Vec<Line> = Vec::new();
        let mut lines2: Vec<Line> = Vec::new();

        let board: Board;
        let board2: Board;

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
            lines.push(line);
            lines2.push(line);
        }

        for line in &lines {
            num_rows = cmp::max(num_rows, cmp::max(line.0 .1, line.1 .1));
            num_cols = cmp::max(num_cols, cmp::max(line.0 .0, line.1 .0));
        }

        num_rows += 1;
        num_cols += 1;

        board = vec![0; num_rows * num_cols];
        board2 = vec![0; num_rows * num_cols];

        println!(
            "Result 1 for file at path {} is {}",
            fp,
            do_work_1(lines, board, num_cols, 1)
        );
        println!(
            "Result 2 for file at path {} is {}",
            fp,
            do_work_1(lines2, board2, num_cols, 2)
        );
    }
}

fn do_work_1(lines: Vec<Line>, mut board: Board, num_cols: usize, part: usize) -> usize {
    let mut num_over_1 = 0;

    let mut start = (0, 0);
    let mut end = (0, 0);

    for line in lines {
        if part == 1 && (line.0 .0 != line.1 .0) && (line.0 .1 != line.1 .1) {
            continue;
        }

        let x_dist = (line.1 .0 as i32 - line.0 .0 as i32).abs() as usize;
        let y_dist = (line.1 .1 as i32 - line.0 .1 as i32).abs() as usize;
        let mut found = false;

        match line.0 .0.cmp(&line.1 .0) {
            std::cmp::Ordering::Less => {
                start.0 = line.0 .0;
                start.1 = line.0 .1;

                end.0 = line.1 .0;
                end.1 = line.1 .1;

                found = true;
            }
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Greater => {
                start.0 = line.1 .0;
                start.1 = line.1 .1;

                end.0 = line.0 .0;
                end.1 = line.0 .1;

                found = true;
            }
        }

        if !found {
            match line.0 .1.cmp(&line.1 .1) {
                std::cmp::Ordering::Less => {
                    start.0 = line.0 .0;
                    start.1 = line.0 .1;

                    end.0 = line.1 .0;
                    end.0 = line.1 .1
                }
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Greater => {
                    start.0 = line.1 .0;
                    start.1 = line.1 .1;

                    end.0 = line.0 .0;
                    end.1 = line.0 .1;
                }
            }
        }

        if x_dist > 0 && y_dist > 0 && part == 2 {
            let mut x_mult: isize = 1;
            let mut y_mult: isize = 1;

            if start.0 > end.0 {
                x_mult = -1;
            }
            if start.1 > end.1 {
                y_mult = -1;
            }

            for i in 0..x_dist + 1 {
                board[((start.0 as isize + i as isize * x_mult) * num_cols as isize
                    + start.1 as isize
                    + i as isize * y_mult) as usize] += 1;
            }
        } else if x_dist > 0 {
            for x in 0..x_dist + 1 {
                board[(start.0 + x) * num_cols + start.1] += 1;
            }
        } else if y_dist > 0 {
            for y in 0..y_dist + 1 {
                board[start.0 * num_cols + start.1 + y] += 1;
            }
        }
    }

    for space in board {
        if space > 1 {
            num_over_1 += 1;
        }
    }

    num_over_1
}
