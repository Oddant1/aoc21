use ferris_says::say;
use std::fs;
use std::io::{stdout, BufWriter};

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
            do_work1(&crabs, *max_dist)
        );
        say(message1.as_bytes(), message1.chars().count(), &mut writer).unwrap();

        let message2 = format!(
            "Result 2 for file at path {} is {}",
            fp,
            do_work2(&crabs, *max_dist)
        );
        say(message2.as_bytes(), message2.chars().count(), &mut writer).unwrap();
    }
}

fn do_work1(crabs: &[usize], max_dist: usize) -> usize {
    let mut dist_sums: Vec<usize> = vec![0; max_dist];

    for crab in crabs {
        for pos in 0..max_dist {
            dist_sums[pos] += (*crab as isize - pos as isize).abs() as usize;
        }
    }

    *dist_sums.iter().min().unwrap()
}

fn do_work2(crabs: &[usize], max_dist: usize) -> usize {
    let mut dist_sums: Vec<usize> = vec![0; max_dist];

    for crab in crabs {
        for pos in 0..max_dist {
            let num_steps = (*crab as isize - pos as isize).abs() as usize;
            dist_sums[pos] += (1..num_steps + 1).fold(0, |a, b| a + b) as usize;
        }
    }

    *dist_sums.iter().min().unwrap()
}
