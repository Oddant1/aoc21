use std::fs;

const FPS: [&str; 2] = ["./src/data/test.txt", "./src/data/input.txt"];

fn main() {
    for fp in FPS {
        println!("Result for file at path {} is: {}", fp, do_work_1(fp));
        println!("Result for file at path {} is: {}", fp, do_work_2(fp));
    }
}

fn do_work_1(fp: &str) -> u32 {
    let depths = fs::read_to_string(fp)
        .expect("Something went wrong reading the file");

    let mut depths = depths.lines();

    let mut times_increased: u32 = 0;
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
