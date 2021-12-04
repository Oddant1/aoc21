use std::fs;

const FPS: [&str; 2] = ["./data/test.txt", "./data/input.txt"];

fn main() {
    for fp in FPS {
        let depths = fs::read_to_string(fp)
            .expect("Something went wrong reading the file");

        let depths: Vec<u32> = depths
            .lines()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        println!("Result 1 for file at path {} is: {}", fp, do_work_1(&depths));
        println!("Result 2 for file at path {} is: {}", fp, do_work_2(&depths));
    }
}

fn do_work_1(depths: &[u32])-> u32 {
    let mut times_increased: u32 = 0;
    let mut prev_depth: u32 = depths[0];

    for depth in depths[1..].iter() {
        let depth = *depth;
        if depth > prev_depth {
            times_increased += 1;
        }

        prev_depth = depth;
    }

    times_increased
}

fn do_work_2(depths: &[u32]) -> u32 {
    let mut times_increased: u32 = 0;
    let mut prev_depth_sum: u32 = depths[0..3].iter().sum();

    for idx in 0..depths.len() - 3 {
        let depth_sum = prev_depth_sum - depths[idx] + depths[idx + 3];

        if depth_sum > prev_depth_sum {
            times_increased += 1;
        }

        prev_depth_sum = depth_sum;
    }

    times_increased
}
