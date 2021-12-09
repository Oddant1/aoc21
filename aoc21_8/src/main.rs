use std::fs;

const FPS: [&str; 2] = ["./data/test.txt", "./data/input.txt"];

fn main() {
    for fp in FPS {
        let contents = fs::read_to_string(fp).expect("Something went wrong reading the file");
        let contents: Vec<&str> = contents.lines().collect();

        let mut inputs: Vec<Vec<&str>> = Vec::new();
        let mut outputs: Vec<Vec<&str>> = Vec::new();

        for line in contents {
            let mut input: Vec<&str> = Vec::new();
            let mut output: Vec<&str> = Vec::new();

            let mut line = line.split(" | ");

            for i in line.next().unwrap().split(" ") {
                input.push(i);
            }

            for o in line.next().unwrap().split(" ") {
                output.push(o);
            }

            inputs.push(input);
            outputs.push(output);
        }

        println!(
            "Result 1 for file at path {} is {}",
            fp,
            do_work1(&outputs)
        );

        println!(
            "Result 2 for file at path {} is {}",
            fp,
            do_work2(&inputs, &outputs)
        );
    }
}

fn do_work1(outputs: &[Vec<&str>]) -> usize {
    let mut unique_counts = 0;

    for output in outputs {
        for signal in output {
            let len = signal.len();

            if len == 2 || len == 3 || len == 4 || len == 7 {
                unique_counts += 1;
            }
        }
    }

    unique_counts
}

fn do_work2(inputs: &[Vec<&str>], outputs: &[Vec<&str>]) -> usize {
    let mut sum = 0;

    for (i, input) in inputs.iter().enumerate() {
        let mut mappings: [&str; 10] = [""; 10];

        for signal in input.iter().chain(&outputs[i]) {
            let len = signal.len();

            if len == 2 {
                mappings[1] = signal;
            } else if len == 3 {
                mappings[7] = signal;
            } else if len == 4 {
                mappings[4] = signal;
            } else if len == 7 {
                mappings[8] = signal;
            }
        }

        for signal in input.iter().chain(&outputs[i]) {
            if diff(mappings[8], signal) == 1 {
                if diff(mappings[4], signal) == 0 {
                    mappings[9] = signal;
                } else if diff(mappings[7], signal) == 0 {
                    mappings[0] = signal;
                } else if diff(mappings[1], signal) == 1 {
                    mappings[6] = signal;
                }
            } else if diff(mappings[4], signal) == 1 {
                if diff(mappings[7], signal) == 0 {
                    mappings[3] = signal;
                } else if diff(mappings[7], signal) == 1 {
                    mappings[5] = signal;
                }
            } else if diff(mappings[4], signal) == 2 && diff(mappings[1], signal) == 1 {
                mappings[2] = signal
            }
        }

        sum += calc_value(&outputs[i], mappings);
    }

    sum
}

fn diff(str1: &str, str2: &str) -> usize {
    let mut diff = str1.len();

    'outer: for c1 in str1.chars() {
        for c2 in str2.chars() {
            if c1 == c2 {
                diff -= 1;
                continue 'outer;
            }
        }
    }

    diff
}

fn calc_value(outputs: &[&str], mappings: [&str; 10]) -> usize {
    let mut mult = 1000;
    let mut val = 0;

    'outer: for output in outputs {
        for (i, mapping) in mappings.iter().enumerate() {
            let first = if output.len() >= mapping.len() { output } else { mapping };
            let second = if output.len() < mapping.len() { output } else { mapping };

            if diff(first, second) == 0 {
                val += i * mult;
                mult /= 10;
                continue 'outer;
            }
        }
    }

    val
}
