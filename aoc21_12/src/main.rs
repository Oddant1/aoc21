use std::collections::HashMap;
use std::fs;

const FPS: [&str; 2] = ["./data/test.txt", "./data/input.txt"];

#[derive(std::cmp::PartialEq, Clone, Copy)]
enum Part {
    One,
    Two,
}

fn main() {
    for fp in FPS {
        let contents = fs::read_to_string(fp).expect("Something went wrong reading the files");
        let lines: Vec<&str> = contents.lines().collect();
        let mut layout = HashMap::new();
        let mut current_path: Vec<&str> = Vec::new();

        for line in lines {
            let mapping: Vec<&str> = line.split("-").collect();
            let from = mapping[0];
            let to = mapping[1];

            if !layout.contains_key(from) {
                let mut values: Vec<&str> = Vec::new();
                values.push(to);

                layout.insert(from, values);
            } else {
                let map = layout.get_mut(from).unwrap();
                map.push(to);
            }

            if !layout.contains_key(to) {
                let mut values: Vec<&str> = Vec::new();
                values.push(from);

                layout.insert(to, values);
            } else {
                let map = layout.get_mut(to).unwrap();
                map.push(from);
            }
        }

        let mut acc = 0;
        do_work(
            &layout,
            &mut current_path,
            "start",
            false,
            &mut acc,
            Part::One,
        );
        println!("Result 1 for file at path {} is {}", fp, acc);
        acc = 0;
        do_work(
            &layout,
            &mut current_path,
            "start",
            false,
            &mut acc,
            Part::Two,
        );
        println!("Result 2 for file at path {} is {}", fp, acc);
    }
}

fn do_work<'a>(
    layout: &HashMap<&'a str, Vec<&'a str>>,
    current_path: &mut Vec<&'a str>,
    current: &'a str,
    double: bool,
    acc: &mut usize,
    part: Part,
) {
    current_path.push(current);

    if current == "end" {
        *acc += 1;
    } else {
        for next in layout.get(current).unwrap() {
            let mut copy = current_path.clone();

            if !(next.chars().all(|c| is_lower(c)) && current_path.contains(next)) {
                do_work(&layout, &mut copy, next, double, acc, part);
            } else if *next != "start" && !double && part == Part::Two {
                do_work(&layout, &mut copy, next, true, acc, part);
            }
        }
    }
}

fn is_lower(c: char) -> bool {
    c as usize >= 97 && c as usize <= 122
}
