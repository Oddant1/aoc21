use std::fs;

const FPS: [&str; 2] = ["./data/test.txt", "./data/input.txt"];

fn main() {
    for fp in FPS {
        println!("Result 1 for file at path {} is: {}", fp, do_work_1(fp));
        println!("Result 2 for file at path {} is: {}", fp, do_work_2(fp));
    }
}

fn do_work_1(fp: &str) -> u32 {
    let depths = fs::read_to_string(fp)
        .expect("Something went wrong reading the file");

    let mut depths = depths.lines();

    let mut times_increased: u32 = 0;
    // Really need a less gross way of doing this
    let mut prev_depth: u32 = depths
        .next()
        .unwrap()
        .parse()
        .unwrap();

    for depth in depths {
        let depth = depth.parse().unwrap();
        if depth > prev_depth {
            times_increased += 1;
        }

        prev_depth = depth;
    }

    times_increased
}

fn do_work_2(fp: &str) -> u32 {
    let depths = fs::read_to_string(fp)
        .expect("Something went wrong reading the file");

    let mut depths = depths.lines();

    let mut times_increased: u32 = 0;
    let mut prev_depth3: u32 = depths
        .next()
        .unwrap()
        .parse()
        .unwrap();

    let mut prev_depth2: u32 = depths
        .next()
        .unwrap()
        .parse()
        .unwrap();

    let mut prev_depth1: u32 = depths
        .next()
        .unwrap()
        .parse()
        .unwrap();

    let mut prev_depth_sum = prev_depth3 + prev_depth2 + prev_depth1;

    for depth in depths {
        let depth: u32 = depth.parse().unwrap();
        let depth_sum = prev_depth_sum - prev_depth3 + depth;

        if depth_sum > prev_depth_sum {
            times_increased += 1;
        }

        prev_depth_sum = depth_sum;
        prev_depth3 = prev_depth2;
        prev_depth2 = prev_depth1;
        prev_depth1 = depth;
    }

    times_increased
}
