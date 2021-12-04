use std::fs;

const FPS: [&str; 2] = ["./data/test.txt", "./data/input.txt"];

fn main() {
    for fp in FPS {
        let commands = fs::read_to_string(fp).expect("Something went wrong reading the file");
        let commands: Vec<&str> = commands.split_whitespace().collect();

        let directions: Vec<&str> = commands.iter().step_by(2).map(|x| *x).collect();
        let amounts: Vec<i32> = commands
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != 0)
            .step_by(2)
            .map(|x| x.1.parse::<i32>().unwrap())
            .collect();

        println!(
            "Result 1 for file at path {} is: {}",
            fp,
            do_work_1(&directions, &amounts)
        );
        println!(
            "Result 2 for file at path {} is: {}",
            fp,
            do_work_2(&directions, &amounts)
        );
    }
}

fn do_work_1(directions: &[&str], amounts: &[i32]) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;

    for (direction, amount) in directions.iter().zip(amounts) {
        if *direction == "up" {
            depth -= amount;
        } else if *direction == "down" {
            depth += amount;
        } else if *direction == "forward" {
            horizontal += amount;
        }
    }

    horizontal * depth
}

fn do_work_2(directions: &[&str], amounts: &[i32]) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for (direction, amount) in directions.iter().zip(amounts) {
        if *direction == "up" {
            aim -= amount;
        } else if *direction == "down" {
            aim += amount;
        } else if *direction == "forward" {
            horizontal += amount;
            depth += aim * amount;
        }
    }

    horizontal * depth
}
