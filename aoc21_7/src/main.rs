use ferris_says::say;
use std::fs;
use std::io::{stdout, BufWriter};

enum Part {
    One,
    Two,
}

const FPS: [&str; 2] = ["./data/test.txt", "./data/input.txt"];

fn main() {
    for fp in FPS {
        let crabs = fs::read_to_string(fp).expect("Something went wrong reading the file");
        let crabs: Vec<usize> = crabs.split(",").map(|x| x.parse().unwrap()).collect();

        let max_dist = crabs.iter().max().unwrap();
        let mut writer = BufWriter::new(stdout());

        let message1 = format!(
            "Result 1 for file at path {} is {}",
            fp,
            do_work(&crabs, *max_dist, Part::One)
        );
        say(message1.as_bytes(), message1.chars().count(), &mut writer).unwrap();

        let message2 = format!(
            "Result 2 for file at path {} is {}",
            fp,
            do_work(&crabs, *max_dist, Part::Two)
        );
        say(message2.as_bytes(), message2.chars().count(), &mut writer).unwrap();
    }
}

fn do_work(crabs: &[usize], max_dist: usize, part: Part) -> usize {
    let mut dist_sums: Vec<usize> = vec![0; max_dist];

    for crab in crabs {
        for pos in 0..max_dist {
            let num_steps = (*crab as isize - pos as isize).abs() as usize;

            match part {
                Part::One => dist_sums[pos] += num_steps,
                Part::Two => {
                    dist_sums[pos] += ((num_steps + 1) as f32 * (num_steps as f32 / 2.0)) as usize
                }
            }
        }
    }

    *dist_sums.iter().min().unwrap()
}
