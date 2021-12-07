use std::fs;

const FPS: [&str; 2] = ["./data/test.txt", "./data/input.txt"];
const PART_1_DAYS: usize = 80;
const PART_2_DAYS: usize = 256;
const MAX_TIMER: usize = 9;

fn main() {
    for fp in FPS {
        let fish_list = fs::read_to_string(fp).expect("Something went wrong reading the file");
        let fish_list: Vec<&str> = fish_list.split(",").collect();
        let mut fish_counts: [usize; MAX_TIMER] = [0; MAX_TIMER];

        for fish in fish_list {
            fish_counts[fish.parse::<usize>().unwrap()] += 1;
        }

        println!(
            "Result 1 for file at path {} is {}",
            fp,
            do_work(&fish_counts, PART_1_DAYS)
        );
        println!(
            "Result 2 for file at path {} is {}",
            fp,
            do_work(&fish_counts, PART_2_DAYS)
        );
    }
}

fn do_work(fish_counts: &[usize; MAX_TIMER], num_days: usize) -> usize {
    let mut working_fish_counts: [usize; MAX_TIMER] = [0; MAX_TIMER];
    working_fish_counts.copy_from_slice(fish_counts);

    for _day in 0..num_days {
        let new_fish = working_fish_counts[0];

        for count in 0..MAX_TIMER - 1 {
            working_fish_counts[count] = working_fish_counts[count + 1];
        }

        working_fish_counts[6] += new_fish;
        working_fish_counts[8] = new_fish;
    }

    working_fish_counts.iter().sum()
}
