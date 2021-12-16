use std::fs;

const FPS: [&str; 2] = ["./data/test.txt", "./data/input.txt"];

fn main() {
    for fp in FPS {
        let contents = fs::read_to_string(fp).expect("Something went wrong reading the files");
        let lines: Vec<&str> = contents.lines().collect();

        let mut coords: Vec<(usize, usize)> = Vec::new();
        let mut folds: Vec<(&str, usize)> = Vec::new();

        let mut max_x = 0;
        let mut max_y = 0;

        let mut line_idx = 0;
        while lines[line_idx] != "" {
            let coord: Vec<&str> = lines[line_idx].split(",").collect();
            let coord: (usize, usize) = (
                coord[0].parse().unwrap(),
                coord[1].parse().unwrap(),
            );

            if coord.0 > max_x {
                max_x = coord.0;
            }

            if coord.1 > max_y {
                max_y = coord.1;
            }

            coords.push(coord);
            line_idx += 1;
        }

        // Skip the empty line
        line_idx += 1;

        while line_idx < lines.len() {
            let fold: Vec<&str> = lines[line_idx].split(" ").last().unwrap().split("=").collect();
            let fold: (&str, usize) = (fold[0], fold[1].parse().unwrap());

            folds.push(fold);
            line_idx += 1;
        }

        let results = do_work(&mut coords, folds, &mut max_x, &mut max_y);
        println!("Result 1 for file at path {} is {}", fp, results.0);
        println!("Result 2 for file at path {} is {}", fp, results.1);
    }
}

fn do_work(
    coords: &mut Vec<(usize, usize)>,
    folds: Vec<(&str, usize)>,
    max_x: &mut usize,
    max_y: &mut usize,
) -> (usize, usize) {
    let mut results: (usize, usize) = (0, 0);

    for fold in folds {
        if fold.0 == "x" {
            *max_x /= 2;
        } else {
            *max_y /= 2;
        }

        for i in 0..coords.len() {
            let mut coord = &mut coords[i];

            if fold.0 == "x" && coord.0 > fold.1 {
                coord.0 = 2 * fold.1 - coord.0;
            } else if fold.0 == "y" && coord.1 > fold.1 {
                coord.1 = 2 * fold.1 - coord.1;
            }
        }

        coords.sort();
        coords.dedup();

        if results.0 == 0 {
            results.0 = coords.len();
        }
    }

    results.1 = coords.len();
    let mut board: Vec<Vec<char>> = vec![vec!['.'; *max_x]; *max_y];

    for coord in coords {
        board[coord.1][coord.0] = '#';
    }

    for line in board {
        for c in line {
            print!("{}", c);
        }
        println!();
    }

    results
}
